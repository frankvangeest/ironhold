/**
 * path: /crates/engine_render/src/lib.rs
 * description: Rendering module for the engine using wgpu.
 */
use wgpu::util::DeviceExt;
use wgpu::*;
use bytemuck::{Pod, Zeroable};
use glam::Mat4; // for matrix operations

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    pub view_proj: [f32; 16], // column-major 4x4 matrix
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Transform {
    pub t0: [f32; 4], // position.x, position.y, rotation, pad
    pub t1: [f32; 4], // scale.x, scale.y, pad, pad
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Sprite {
    pub s0: [f32; 4],   // dimensions.x, dimensions.y, pad, pad
    pub color: [f32; 4] // RGBA
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct InstanceData {
    pub transform: Transform,
    pub sprite: Sprite,
}

pub struct QuadPipeline {
    pub pipeline: RenderPipeline,
    pub instance_bind_group: BindGroup,
    pub camera_bind_group: BindGroup,
    pub instance_buffer: Buffer,
    pub camera_buffer: Buffer,
    pub instance_count: u32,
}

impl QuadPipeline {
    pub fn new(device: &Device, texture_format: TextureFormat, scene_instances: &[InstanceData]) -> Self {
        
        let safe_instances = if scene_instances.is_empty() {
            vec![InstanceData::zeroed()] // bytemuck::Zeroable
        } else {
            scene_instances.to_vec()
        };

        // Camera setup (orthographic projection)
        let ortho_matrix = Mat4::orthographic_lh(0.0, 1024.0, 0.0, 720.0, -1.0, 1.0);
        let camera_uniform = CameraUniform {
            view_proj: ortho_matrix.to_cols_array(),
        };

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Uniform Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        // Create instance buffer
        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Instance Buffer"),
            contents: bytemuck::cast_slice(&safe_instances),
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
        });
        
        // Instance bind group layout
        let instance_bgl = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Instance Bind Group Layout"),
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::VERTEX,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let instance_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Instance Bind Group"),
            layout: &instance_bgl,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: instance_buffer.as_entire_binding(),
            }],
        });

        // Camera bind group layout
        let camera_bgl = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Camera Bind Group Layout"),
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::VERTEX,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: std::num::NonZeroU64::new(std::mem::size_of::<CameraUniform>() as u64),
                },
                count: None,
            }],
        });

        let camera_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Camera Bind Group"),
            layout: &camera_bgl,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
        });

        // Shader module
        let shader_src = include_str!("../shaders/quad.wgsl");
        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Quad Shader"),
            source: ShaderSource::Wgsl(shader_src.into()),
        });

        // Pipeline layout
        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Quad Pipeline Layout"),
            bind_group_layouts: &[&instance_bgl, &camera_bgl],
            push_constant_ranges: &[],
        });

        // Render pipeline
        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Quad Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[], // No vertex buffer; we use vertex_index
                compilation_options: PipelineCompilationOptions::default(),
            },
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(ColorTargetState {
                    format: texture_format, // Match surface format
                    blend: Some(BlendState::REPLACE),
                    write_mask: ColorWrites::ALL,
                })],
                compilation_options: PipelineCompilationOptions::default(),
            }),
            primitive: PrimitiveState::default(),
            depth_stencil: None,
            multisample: MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        Self {
            pipeline,
            instance_bind_group,
            camera_bind_group,
            instance_buffer,
            camera_buffer,
            instance_count: scene_instances.len() as u32,
        }
    }

    /// Update camera matrix (e.g., on window resize).
    pub fn update_camera(&self, queue: &Queue, width: u32, height: u32) {
        // let ortho = Mat4::orthographic_lh(0.0, width as f32, 0.0, height as f32, -1.0, 1.0);
        let ortho = Mat4::orthographic_lh(-10.0, 10.0, -10.0, 10.0, -1.0, 1.0);
        let camera_uniform = CameraUniform {
            view_proj: ortho.to_cols_array(),
        };
        queue.write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(&[camera_uniform]));
    }

    /// Update GPU buffer contents without recreating pipeline/bind group.
    pub fn update_instances(&mut self, queue: &Queue, instances: &[InstanceData]) {
        queue.write_buffer(
            &self.instance_buffer, 
            0, 
            bytemuck::cast_slice(instances)
        );
        self.instance_count = instances.len() as u32; // ✅ update count
    }

    pub fn ensure_capacity(
        &mut self,
        device: &wgpu::Device,
        instances: &[InstanceData],
    ) {
        let needed = (instances.len() * std::mem::size_of::<InstanceData>()) as u64;
        let current = self.instance_buffer.size(); // wgpu 0.20+ has Buffer::size()
        if needed > current {
            // grow to next power of two or 1.5x to amortize re-allocation
            let mut new_size = current.max(64) * 2;
            while new_size < needed {
                new_size *= 2;
            }
            // re-create buffer
            self.instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("Instance Buffer (resized)"),
                size: new_size,
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
            // re-create bind group (same layout)
            self.instance_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Instance Bind Group (resized)"),
                layout: &self.pipeline.get_bind_group_layout(0), // or store layout
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: self.instance_buffer.as_entire_binding(),
                }],
            });
        }
    }

    pub fn draw<'a>(&'a self, rpass: &mut RenderPass<'a>) {
        rpass.set_pipeline(&self.pipeline);
        rpass.set_bind_group(0, &self.instance_bind_group, &[]);
        rpass.set_bind_group(1, &self.camera_bind_group, &[]);
        rpass.draw(0..6, 0..self.instance_count);
    }
}
