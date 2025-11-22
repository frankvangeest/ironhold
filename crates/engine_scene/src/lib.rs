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
}

impl Scene {
    pub fn from_ron_str(s: &str) -> Result<Scene, ron::error::SpannedError> {
        ron::from_str(s)
    }
    pub fn to_ron_string(&self) -> String {
        ron::to_string(self).unwrap_or_default()
    }
}
