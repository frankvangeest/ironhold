
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::HtmlCanvasElement;
use wgpu::*;

#[derive(thiserror::Error, Debug)]
pub enum InitError {
    #[error("webgpu not available")] WebGpuUnavailable,
    #[error("wgpu init failed")] InitFailed,
}

pub async fn init_wgpu(_canvas: &HtmlCanvasElement) -> Result<(Device, Queue, SurfaceConfiguration), JsValue> {
    // This is a stub. Real implementation would request adapter/device, configure surface.
    Err(JsValue::from_str("wgpu init stub"))
}
