/**
 * path: /crates/engine_scene/src/lib.rs
 * description: Scene representation and serialization.
 * Declarative scene format using RON.
 * engine_scene should only describe what exists (entities, components, references to assets).
 * It should not know how assets are loaded or rendered.
 */
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Scene {
    pub name: String,
    pub entities: Vec<Entity>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Entity {
    pub id: u32,
    pub name: String,

    // Optional components
    #[serde(default)]
    pub transform_2d: Option<Transform2D>,

    #[serde(default)]
    pub transform_3d: Option<Transform3D>,

    #[serde(default)]
    pub sprite: Option<Sprite>,

    #[serde(default)]
    pub mesh: Option<Mesh>,
}

/// Spatial transform.
/// - `position`: (x, y) in world units
/// - `rotation`: degrees (clockwise, screen-space; adjust later if needed)
/// - `scale`: (sx, sy) multiplicative scaling
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Transform2D {
    pub position: (f32, f32),
    pub rotation: f32,
    pub scale: (f32, f32),
}

/// Sprite rendering description.
/// - `dimensions`: (width, height) in world units (pre-scale)
/// - `color`: RGBA (0..1)
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Sprite {
    pub dimensions: (f32, f32),
    pub color: (f32, f32, f32, f32),
}

/// Spatial transform.
/// - `position`: (x, y, z) in world units
/// - `rotation`: degrees for (x, y, z, clockwize)
/// - `scale`: (sx, sy, sz) multiplicative scaling
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Transform3D {
    pub position: (f32, f32, f32),
    pub rotation: (f32, f32, f32),
    pub scale: (f32, f32, f32),
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Mesh {
    pub file: String,
    pub node: Option<usize>,
    pub primitive: Option<usize>,
}

impl Scene {
    pub fn from_ron_str(s: &str) -> Result<Scene, ron::error::SpannedError> {
        ron::from_str(s)
    }
    pub fn to_ron_string(&self) -> String {
        ron::to_string(self).unwrap_or_default()
    }
}
