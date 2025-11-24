/**
 * path: /crates/engine_render/src/lib.rs
 * description: Rendering module for the engine using wgpu.
 */
use wgpu::util::DeviceExt;
use wgpu::*;
use bytemuck::{Pod, Zeroable};

#[repr(C)] #[derive(Clone, Copy, Pod, Zeroable)]
pub struct Transform {
    pub t0: [f32; 4], // position.x, position.y, rotation, pad
    pub t1: [f32; 4], // scale.x, scale.y, pad, pad
}

#[repr(C)] #[derive(Clone, Copy, Pod, Zeroable)]
pub struct Sprite {
    pub s0: [f32; 4],   // dimensions.x, dimensions.y, pad, pad
    pub color: [f32; 4] // RGBA
}

#[repr(C)] #[derive(Clone, Copy, Pod, Zeroable)]
pub struct InstanceData {
    pub transform: Transform,
    pub sprite: Sprite,
}

pub struct QuadPipeline {
    pub pipeline: RenderPipeline,
    pub bind_group: BindGroup,
    pub instance_buffer: Buffer,
    pub instance_count: u32,
}

impl QuadPipeline {
    pub fn new(device: &Device, texture_format: TextureFormat, scene_instances: &[InstanceData]) -> Self {
        
        let safe_instances = if scene_instances.is_empty() {
            vec![InstanceData::zeroed()] // bytemuck::Zeroable
        } else {
            scene_instances.to_vec()
        };

        // 1. Create instance buffer
        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Instance Buffer"),
            contents: bytemuck::cast_slice(&safe_instances),
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
        });

        // 2. Bind group layout
        let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
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

        // 3. Bind group
        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Instance Bind Group"),
            layout: &bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: instance_buffer.as_entire_binding(),
            }],
        });

        // 4. Shader module
        let shader_src = include_str!("../shaders/quad.wgsl");
        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Quad Shader"),
            source: ShaderSource::Wgsl(shader_src.into()),
        });

        // 5. Pipeline layout
        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Quad Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        // 6. Render pipeline
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
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        Self {
            pipeline,
            bind_group,
            instance_buffer,
            instance_count: scene_instances.len() as u32,
        }
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
            self.bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
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
        rpass.set_bind_group(0, &self.bind_group, &[]);
        rpass.draw(0..6, 0..self.instance_count);
    }
}
