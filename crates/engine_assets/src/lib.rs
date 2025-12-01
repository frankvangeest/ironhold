/**
 * path: /crates/engine_assets/src/lib.rs
 * description: Asset management for the engine.
 */
use thiserror::Error;
mod mesh_loader;
pub use mesh_loader::{load_mesh_stub};
pub use engine_types::MeshData;

#[derive(Error, Debug)]
pub enum AssetError {
    #[error("io: {0}")]
    Io(String),
    #[error("format: {0}")]
    Format(String),
}

pub fn hot_reload_stub(url: &str) -> Result<(), AssetError> {
    let _ = url;
    Ok(())
}
