/**
 * path: /crates/engine_types/src/lib.rs
 * description: Engine Types, used by multiple other engine crates, like engine_render, engine_assets.
 */

use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    pub view_proj: [f32; 16], // column-major 4x4 matrix
}

/// Vertex structure for 3D meshes
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct MeshVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2],
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Transform2D {
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
    pub transform: Transform2D,
    pub sprite: Sprite,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Transform3D {
    pub t0: [f32; 8], // position.x, position.y, position,z, rotation.x, rotation.y, rotation.z, pad, pad
    pub t1: [f32; 4], // scale.x, scale.y, scale.z, pad
}

#[derive(Clone)]
pub struct MeshData {
    pub vertices: Vec<MeshVertex>,
    pub indices: Vec<u32>,
}

