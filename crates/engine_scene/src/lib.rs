
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Scene {
    pub name: String,
}

impl Scene {
    pub fn from_ron_str(s: &str) -> Result<Scene, ron::error::SpannedError> { ron::from_str(s) }
    pub fn to_ron_string(&self) -> String { ron::to_string(self).unwrap_or_default() }
}
