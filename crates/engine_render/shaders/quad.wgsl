
// quad.wgsl
// Packed layout using vec4 (16-byte alignment) to match Rust structs.
// Each instance = 64 bytes (two vec4 for Transform, two vec4 for Sprite).

struct Transform {
  t0: vec4<f32>, // position.x, position.y, rotation(rad), pad
  t1: vec4<f32>, // scale.x, scale.y, pad, pad
};

struct Sprite {
  s0: vec4<f32>,    // dimensions.x, dimensions.y, pad, pad
  color: vec4<f32>, // RGBA
};

struct InstanceData {
  transform: Transform,
  sprite: Sprite,
};

@group(0) @binding(0)
var<storage, read> instances: array<InstanceData>;

struct VertexOutput {
  @builtin(position) position: vec4<f32>,
  @location(0) color: vec4<f32>,
};

@vertex
fn vs_main(
  @builtin(vertex_index) vertex_index: u32,
  @builtin(instance_index) instance_index: u32
) -> VertexOutput {
  // Quad positions (two triangles)
  let quad = array<vec2<f32>, 6>(
    vec2<f32>(-0.5, -0.5), vec2<f32>( 0.5, -0.5), vec2<f32>( 0.5,  0.5),
    vec2<f32>(-0.5, -0.5), vec2<f32>( 0.5,  0.5), vec2<f32>(-0.5,  0.5)
  );

  let inst = instances[instance_index];

  // Unpack fields from packed vec4 lanes
  let pos     = vec2<f32>(inst.transform.t0.x, inst.transform.t0.y);
  let rot_rad = inst.transform.t0.z;
  let scl     = vec2<f32>(inst.transform.t1.x, inst.transform.t1.y);
  let dims    = vec2<f32>(inst.sprite.s0.x,    inst.sprite.s0.y);

  // Base quad scaled by dimensions and scale
  var p = quad[vertex_index] * dims;
  p = p * scl;

  // Rotate
  let c = cos(rot_rad);
  let s = sin(rot_rad);
  let r = vec2<f32>(p.x * c - p.y * s, p.x * s + p.y * c);

  // Translate
  let world_pos = r + pos;

  var out: VertexOutput;
  out.position = vec4<f32>(world_pos, 0.0, 1.0);
  out.color = inst.sprite.color;
  return out;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
  return input.color;
}
