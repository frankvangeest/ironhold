wgsl
// mesh.wgsl
// Basic 3D mesh rendering shader with perspective projection.

struct Camera {
    view_proj: mat4x4<f32>,
};
@group(0) @binding(0)
var<uniform> camera: Camera;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) normal: vec3<f32>,
    @location(1) uv: vec2<f32>,
};

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    let world_pos = vec4<f32>(in.position, 1.0);
    out.position = camera.view_proj * world_pos;
    out.normal = in.normal;
    out.uv = in.uv;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // For now, output a simple light gray color
    return vec4<f32>(0.9, 0.9, 0.9, 1.0);
}
