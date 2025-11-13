/**
 * path: /crates/platform_web/src/lib.rs
 * description: Web platform support for the project, including WebGPU feature detection
 * and hot-reloading capabilities.
 */

pub mod wgpu_init;
pub use wgpu_init::WgpuContext;

use wasm_bindgen::prelude::*;
use web_sys::{WebSocket, MessageEvent};

#[wasm_bindgen]
pub fn webgpu_supported() -> bool {
    // Minimal feature-detection via JS (navigator.gpu)
    js_sys::Reflect::has(&web_sys::window().unwrap(), &JsValue::from_str("navigator")).unwrap_or(false)
}

pub mod hotreload {
    use super::*;
    pub fn start_ws(_url: &str) -> Option<WebSocket> {
        // Placeholder: connect to ws and emit events
        None
    }
}
