
use engine_scene::Scene;
use engine_assets::{load_mesh_stub, MeshData};
use engine_types::{
    // MeshVertex,
    InstanceData,
    Transform2D,
    Sprite,
};

// Helper to allow storing closures (not fully used yet)
// struct RcCell<T>(std::rc::Rc<std::cell::RefCell<Option<T>>>);
// impl<T> RcCell<T> { fn new(v: Option<T>) -> Self { Self(std::rc::Rc::new(std::cell::RefCell::new(v))) } }

pub fn scene_to_instances(scene: &Scene) -> Vec<InstanceData> {
    scene
        .entities
        .iter()
        .filter(|e| e.transform_2d.is_some())
        .map(|e| {
            let transform_2d: &engine_scene::Transform2D  = e.transform_2d.as_ref().expect("Entity missing transform_2d component");

            // Rotation is authored in degrees in RON; WGSL expects radians.
            let rot_rad = transform_2d.rotation.to_radians();

            let sprite: &engine_scene::Sprite = e.sprite.as_ref().expect("Entity missing sprite component");

            InstanceData {
                transform: Transform2D {
                    // t0: position.x, position.y, rotation(rad), pad
                    t0: [
                        transform_2d.position.0,
                        transform_2d.position.1,
                        rot_rad,
                        0.0,
                    ],
                    // t1: scale.x, scale.y, pad, pad
                    t1: [
                        transform_2d.scale.0,
                        transform_2d.scale.1,
                        0.0,
                        0.0,
                    ],
                },
                sprite: Sprite {
                    // s0: dimensions.x, dimensions.y, pad, pad
                    s0: [
                        sprite.dimensions.0,
                        sprite.dimensions.1,
                        0.0,
                        0.0,
                    ],
                    // RGBA
                    color: [
                        sprite.color.0,
                        sprite.color.1,
                        sprite.color.2,
                        sprite.color.3,
                    ],
                },
            }
        })
        .collect()
}

/// Convert a Scene into a list of MeshData for rendering.
/// Currently uses `load_mesh_stub()` for placeholder mesh loading.
pub fn scene_to_meshes(scene: &Scene) -> Vec<MeshData> {
    let mut meshes = Vec::new();
    for entity in &scene.entities {
        if entity.mesh.is_some() {
            if let Some(mesh_data) = load_mesh_stub(entity) {
                meshes.push(mesh_data);
            } else {
                web_sys::console::warn_1(
                    &format!("Mesh entity '{}' could not be loaded", entity.name).into(),
                );
            }
        }
    }
    meshes
}
