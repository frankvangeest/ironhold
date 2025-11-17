/**
 * path: /crates/platform_web/src/wgpu_init.rs
 * description: WebGPU initialization for web platform.
 */

use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;
use wgpu::*;
use std::ptr::NonNull;
// NEW: use the rwh re-export from wgpu (matches wgpu's version exactly)
use wgpu::rwh;

/// Graphics context tied to a specific HTML canvas.
pub struct WgpuContext {
    pub canvas: HtmlCanvasElement,
    pub surface: Surface<'static>,
    pub device: Device,
    pub queue: Queue,
    pub config: SurfaceConfiguration,
}

/// Thin wrapper that turns an HtmlCanvasElement into a window/display handle.
struct CanvasHandle {
    canvas: HtmlCanvasElement,
}

impl CanvasHandle {
    fn new(canvas: HtmlCanvasElement) -> Self {
        Self { canvas }
    }
}

impl rwh::HasWindowHandle for CanvasHandle {
    fn window_handle(&self) -> Result<rwh::WindowHandle<'_>, rwh::HandleError> {
        let js_value: &wasm_bindgen::JsValue = &self.canvas;
        let obj: std::ptr::NonNull<std::ffi::c_void> = std::ptr::NonNull::from(js_value).cast();
        let web_canvas = rwh::WebCanvasWindowHandle::new(obj);
        let raw = rwh::RawWindowHandle::from(web_canvas);
        // 1-arg form on the version bundled with wgpu 0.20.x
        let handle = unsafe { rwh::WindowHandle::borrow_raw(raw) };
        Ok(handle)
    }
}

impl rwh::HasDisplayHandle for CanvasHandle {
    fn display_handle(&self) -> Result<rwh::DisplayHandle<'_>, rwh::HandleError> {
        let web_display = rwh::WebDisplayHandle::new();
        let raw = rwh::RawDisplayHandle::from(web_display);
        let handle = unsafe { rwh::DisplayHandle::borrow_raw(raw) };
        Ok(handle)
    }
}

/// Initialize WebGPU/WGPU for the given canvas.
pub async fn init_wgpu(canvas: HtmlCanvasElement) -> Result<WgpuContext, JsValue> {
    let instance = Instance::new(InstanceDescriptor {
        backends: Backends::all(),
        ..Default::default()
    }); // Instance is the entry point. [2](https://docs.rs/wgpu/latest/wgpu/enum.SurfaceTarget.html)

    // LEAK the handle to get an &'static CanvasHandle (surface needs 'static)
    let target_ref: &'static CanvasHandle = Box::leak(Box::new(CanvasHandle::new(canvas.clone())));

    let surface = instance
        .create_surface(target_ref)
        .map_err(|e| JsValue::from_str(&format!("create_surface failed: {e:?}")))?; // Requires HasWindow/DisplayHandle. 

    let adapter = instance
        .request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        })
        .await
        .ok_or_else(|| JsValue::from_str("No suitable WebGPU adapter found"))?;

    let required_limits = {
        // Prefer web‑portable presets on WASM:
        // - downlevel_defaults(): safe baseline for WebGPU
        // - downlevel_webgl2_defaults(): even stricter (if you target older GPUs)
        #[cfg(target_arch = "wasm32")]
        // { Limits::downlevel_defaults() }
        { Limits::downlevel_webgl2_defaults() }

        #[cfg(not(target_arch = "wasm32"))]
        { Limits::default() } // or adapter.limits() with tweaks
    };

    let (device, queue) = adapter
        .request_device(
            &DeviceDescriptor {
                label: Some("ironhold_device"),
                required_features: Features::empty(),
                required_limits: required_limits,
            },
            None,
        )
        .await
        .map_err(|e| JsValue::from_str(&format!("request_device failed: {e}")))?; // [2](https://docs.rs/wgpu/latest/wgpu/enum.SurfaceTarget.html)

    let caps = surface.get_capabilities(&adapter); // formats, present/alpha/present_modes. [1](https://webgpu-native.github.io/webgpu-headers/Surfaces.html)
    let format = caps
        .formats
        .iter()
        .copied()
        .find(|f| matches!(f, TextureFormat::Bgra8UnormSrgb | TextureFormat::Rgba8UnormSrgb))
        .unwrap_or_else(|| caps.formats[0]);
    let present_mode = if caps.present_modes.contains(&PresentMode::AutoVsync) {
        PresentMode::AutoVsync
    } else {
        PresentMode::Fifo
    }; // Fifo is always supported; Auto* gracefully falls back. [3](https://docs.rs/wgpu/latest/wgpu/)
    let alpha_mode = caps
        .alpha_modes
        .iter()
        .copied()
        .find(|m| *m == CompositeAlphaMode::Opaque)
        .unwrap_or(caps.alpha_modes[0]);

    let width = canvas.width().max(1);
    let height = canvas.height().max(1);

    let config = SurfaceConfiguration {
        usage: TextureUsages::RENDER_ATTACHMENT,
        format,
        width,
        height,
        present_mode,
        desired_maximum_frame_latency: 2,
        alpha_mode,
        view_formats: vec![],
    }; // Fields per current docs. [4](https://github.com/gfx-rs/wgpu/issues/5661)

    surface.configure(&device, &config); // [1](https://webgpu-native.github.io/webgpu-headers/Surfaces.html)

    Ok(WgpuContext {
        canvas,
        surface,
        device,
        queue,
        config,
    })
}