
use wgpu::util::DeviceExt;
use wgpu::*;
// use bytemuck::{Pod, Zeroable};
use glam::Mat4;

use engine_types::{
    MeshVertex, 
    CameraUniform,
};


/// Pipeline for rendering 3D meshes
pub struct GUIPipeline {
    pub pipeline: RenderPipeline,
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
    pub camera_bind_group: BindGroup,
    pub camera_buffer: Buffer,
    pub index_count: u32,
}

impl GUIPipeline {
    pub fn new(
        device: &Device,
        texture_format: TextureFormat,
        vertices: &[MeshVertex],
        indices: &[u32],
    ) -> Self {
        // Camera setup (perspective projection)
        let aspect_ratio = 16.0 / 9.0;
        let perspective = Mat4::perspective_rh_gl(45.0_f32.to_radians(), aspect_ratio, 0.1, 100.0);
        let camera_uniform = CameraUniform {
            view_proj: perspective.to_cols_array(),
        };

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Uniform Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Mesh Vertex Buffer"),
            contents: bytemuck::cast_slice(vertices),
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Mesh Index Buffer"),
            contents: bytemuck::cast_slice(indices),
            usage: BufferUsages::INDEX | BufferUsages::COPY_DST,
        });

        // Bind group for camera
        let camera_bgl = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Camera Bind Group Layout"),
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::VERTEX,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
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
        let shader_src = include_str!("../shaders/mesh.wgsl");
        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Mesh Shader"),
            source: ShaderSource::Wgsl(shader_src.into()),
        });

        // Pipeline layout
        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Mesh Pipeline Layout"),
            bind_group_layouts: &[&camera_bgl],
            push_constant_ranges: &[],
        });

        // Vertex buffer layout
        let vertex_layout = VertexBufferLayout {
            array_stride: std::mem::size_of::<MeshVertex>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &[
                VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: VertexFormat::Float32x3, // position
                },
                VertexAttribute {
                    offset: 12,
                    shader_location: 1,
                    format: VertexFormat::Float32x3, // normal
                },
                VertexAttribute {
                    offset: 24,
                    shader_location: 2,
                    format: VertexFormat::Float32x2, // uv
                },
            ],
        };

        // Render pipeline
        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Mesh Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[vertex_layout],
                compilation_options: PipelineCompilationOptions::default(),
            },
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(ColorTargetState {
                    format: texture_format,
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
            vertex_buffer,
            index_buffer,
            camera_bind_group,
            camera_buffer,
            index_count: indices.len() as u32,
        }
    }

    pub fn draw<'a>(&'a self, rpass: &mut RenderPass<'a>) {
        rpass.set_pipeline(&self.pipeline);
        rpass.set_bind_group(0, &self.camera_bind_group, &[]);
        rpass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        rpass.set_index_buffer(self.index_buffer.slice(..), IndexFormat::Uint32);
        rpass.draw_indexed(0..self.index_count, 0, 0..1);
    }
    
    pub fn update_camera(&self, queue: &wgpu::Queue, width: u32, height: u32) {
        let aspect_ratio = if width > 0 && height > 0 {
            width as f32 / height as f32
        } else {
            16.0 / 9.0
        };
        let perspective = glam::Mat4::perspective_rh_gl(45.0_f32.to_radians(), aspect_ratio, 0.1, 100.0);
        let camera_uniform = CameraUniform {
            view_proj: perspective.to_cols_array(),
        };
        queue.write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(&[camera_uniform]));
    }

    pub fn ensure_vertex_capacity(&mut self, device: &wgpu::Device, vertices: &[MeshVertex]) {
        let needed = (vertices.len() * std::mem::size_of::<MeshVertex>()) as u64;
        let current = self.vertex_buffer.size();
        if needed > current {
            let mut new_size = current.max(64) * 2;
            while new_size < needed {
                new_size *= 2;
            }
            self.vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("Mesh Vertex Buffer (resized)"),
                size: new_size,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
        }
    }

    pub fn ensure_index_capacity(&mut self, device: &wgpu::Device, indices: &[u32]) {
        let needed = (indices.len() * std::mem::size_of::<u32>()) as u64;
        let current = self.index_buffer.size();
        if needed > current {
            let mut new_size = current.max(64) * 2;
            while new_size < needed {
                new_size *= 2;
            }
            self.index_buffer = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("Mesh Index Buffer (resized)"),
                size: new_size,
                usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
        }
    }
}
