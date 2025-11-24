/**
 * path: /crates/engine_scene/src/lib.rs
 * description: Scene representation and serialization.
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
    pub transform: Transform,
    pub sprite: Sprite,
}

/// Spatial transform.
/// - `position`: (x, y) in world units
/// - `rotation`: degrees (clockwise, screen-space; adjust later if needed)
/// - `scale`: (sx, sy) multiplicative scaling
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Transform {
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

impl Scene {
    pub fn from_ron_str(s: &str) -> Result<Scene, ron::error::SpannedError> {
        ron::from_str(s)
    }
    pub fn to_ron_string(&self) -> String {
        ron::to_string(self).unwrap_or_default()
    }
}
