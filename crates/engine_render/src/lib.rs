/**
 * path: /crates/engine_render/src/lib.rs
 * description: Rendering module for the engine using wgpu.
 */
mod bg_pipeline;
mod quad_pipeline;
mod mesh_pipeline;
mod gui_pipeline;

pub use bg_pipeline::{BGPipeline};
pub use quad_pipeline::{QuadPipeline};
pub use mesh_pipeline::{MeshPipeline};
pub use gui_pipeline::{GUIPipeline};
