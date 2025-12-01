use engine_scene::Entity;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen::JsCast;

use engine_types::{
    MeshVertex,
    MeshData,
};


/// Stub function to load mesh data from an entity
/// In future: parse GLTF or other formats based on entity.mesh path
pub fn load_mesh_stub(entity: &Entity) -> Option<MeshData> {
    if entity.mesh.is_none() {
        return None;
    }

    // Hardcoded triangle for now
    let vertices = vec![
        MeshVertex { position: [0.0, 0.5, 0.0], normal: [0.0, 0.0, 1.0], uv: [0.0, 0.0] },
        MeshVertex { position: [-0.5, -0.5, 0.0], normal: [0.0, 0.0, 1.0], uv: [1.0, 0.0] },
        MeshVertex { position: [0.5, -0.5, 0.0], normal: [0.0, 0.0, 1.0], uv: [0.5, 1.0] },
    ];
    let indices = vec![0, 1, 2];

    Some(MeshData { vertices, indices })
}

pub async fn load_mesh_gltf(url: &str) -> Result<MeshData, JsValue> {
    // Fetch GLTF file
    let promise = web_sys::window()
        .unwrap()
        .fetch_with_str(url);
    let resp_value = JsFuture::from(promise).await?;
    let resp: web_sys::Response = resp_value.dyn_into().unwrap();

    let buf_promise = resp.array_buffer()?;
    let buf_value = JsFuture::from(buf_promise).await?;
    let buf: js_sys::ArrayBuffer = buf_value.dyn_into().unwrap();

    let bytes = js_sys::Uint8Array::new(&buf).to_vec();

    // Parse GLTF
    let gltf = gltf::Gltf::from_slice(&bytes).map_err(|e| JsValue::from_str(&format!("GLTF parse error: {:?}", e)))?;
    let mesh = gltf.meshes().next().ok_or(JsValue::from_str("No mesh found"))?;
    let primitive = mesh.primitives().next().ok_or(JsValue::from_str("No primitive found"))?;

    // Extract positions, normals, UVs
    let reader = primitive.reader(|buffer| Some(&bytes[buffer.index()..]));
    let positions: Vec<[f32; 3]> = reader.read_positions().unwrap().collect();
    let normals: Vec<[f32; 3]> = reader
        .read_normals()
        .map(|iter| iter.collect()).expect("No normals found!");;
    let uvs: Vec<[f32; 2]> = reader
        .read_tex_coords(0)
        .map(|tc| tc.into_f32().collect())
        .unwrap_or_else(|| vec![[0.0, 0.0]; positions.len()]);
    let indices: Vec<u32> = reader.read_indices().unwrap().into_u32().collect();

    // Convert to MeshVertex
    let vertices: Vec<MeshVertex> = positions.iter().enumerate().map(|(i, pos)| MeshVertex {
        position: *pos,
        normal: normals[i],
        uv: uvs[i],
    }).collect();

    Ok(MeshData { vertices, indices })
}

// Integration point in engine_wasm_api:
// Iterate scene.entities, if entity.mesh exists, call load_mesh_stub and create MeshPipeline.
