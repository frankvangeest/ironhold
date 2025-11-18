/**
 * path: /crates/engine_wasm_api/src/lib.rs
 * description: WASM bindings for the engine using wasm-bindgen.
 */

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;
use engine_core::{EngineApp, Mode, set_mode};
use engine_scene::Scene;
use js_sys;
use crate::js_sys::Date;
use std::cell::RefCell;
use std::rc::Rc;
use platform_web::WgpuContext;
use wgpu::{StoreOp};

// Build info functions
fn build_id() -> &'static str {
    option_env!("IRONHOLD_BUILD_ID").unwrap_or("unknown")
}
fn build_sha() -> &'static str {
    option_env!("IRONHOLD_GIT_SHA").unwrap_or("unknown")
}
fn build_time() -> &'static str {
    option_env!("IRONHOLD_BUILD_TIME").unwrap_or("unknown")
}


#[wasm_bindgen]
pub struct Engine {
    app: EngineApp,
    canvas: Option<HtmlCanvasElement>,
    gfx: Option<WgpuContext>,

    // Keep the RAF closure alive across frames
    raf_closure: Option<wasm_bindgen::closure::Closure<dyn FnMut(f64)>>,
    running: bool,
    last_ts: f64,
}

#[wasm_bindgen]
impl Engine {
    pub async fn mount_async(&mut self) -> Result<(), JsValue> {
        let canvas = self.canvas.take().ok_or(JsValue::from_str("no canvas bound"))?;
        let gfx = platform_web::wgpu_init::init_wgpu(canvas).await?;
        self.gfx = Some(gfx);
        Ok(())
    }
}

#[wasm_bindgen]
pub struct EngineOptions {
    canvas_id: Option<String>,
    assets_base_url: Option<String>,
    enable_2d: bool,
    enable_3d: bool,
}

#[wasm_bindgen]
impl EngineOptions {
    #[wasm_bindgen(constructor)]
    pub fn new() -> EngineOptions {
        EngineOptions { canvas_id: None, assets_base_url: None, enable_2d: true, enable_3d: true }
    }
    pub fn canvas_id(mut self, id: String) -> Self { self.canvas_id = Some(id); self }
    pub fn assets_base_url(mut self, url: String) -> Self { self.assets_base_url = Some(url); self }
    pub fn enable_2d(mut self, v: bool) -> Self { self.enable_2d = v; self }
    pub fn enable_3d(mut self, v: bool) -> Self { self.enable_3d = v; self }
}


#[wasm_bindgen]
pub async fn init(opts: EngineOptions) -> Result<Engine, JsValue> {
    console_error_panic_hook::set_once();

    // Log build info
    web_sys::console::log_1(
        &format!(
            "Ironhold build {} ({} @ {})",
            build_id(), build_sha(), build_time()
        ).into()
    );

    let mut canvas: Option<HtmlCanvasElement> = None;
    if let Some(id) = opts.canvas_id.clone() {
        let window = web_sys::window().ok_or_else(|| JsValue::from_str("no window"))?;
        let doc = window.document().ok_or_else(|| JsValue::from_str("no document"))?;
        let el = doc.get_element_by_id(&id).ok_or_else(|| JsValue::from_str("canvas not found"))?;
        canvas = Some(el.dyn_into::<HtmlCanvasElement>().map_err(|_| JsValue::from_str("bad canvas"))?);
    }

    Ok(Engine {
        app: EngineApp::default(),
        canvas,
        gfx: None,
        raf_closure: None,
        running: false,
        last_ts: Date::now(), // milliseconds
    })
}



#[wasm_bindgen]
impl Engine {
    pub fn mount(&mut self) -> Result<(), JsValue> {
        if self.canvas.is_none() {
            return Err(JsValue::from_str("no canvas bound"));
        }
        // TODO: configure wgpu surface for the canvas here
        Ok(())
    }

    /// Start the requestAnimationFrame loop.
    pub fn start(&mut self) -> Result<(), JsValue> {
        if self.running {
            return Ok(());
        }
        self.running = true;

        let window = web_sys::window().ok_or("no window")?;
        // We’ll pass a raw pointer to self into the closure.
        let engine_ptr: *mut Engine = self as *mut _;

        let f: Rc<RefCell<Option<wasm_bindgen::closure::Closure<dyn FnMut(f64)>>>> =
            Rc::new(RefCell::new(None));
        let g = f.clone();

        *g.borrow_mut() = Some(wasm_bindgen::closure::Closure::wrap(Box::new(move |ts_ms: f64| {
            // SAFETY: Only used while Engine is alive and running.
            let engine: &mut Engine = unsafe { &mut *engine_ptr };
            if !engine.running {
                return; // allow graceful stop
            }

            // dt in milliseconds (f32)
            let dt_ms = (ts_ms - engine.last_ts) as f32;
            engine.last_ts = ts_ms;

            engine.tick(dt_ms);

            // Schedule next frame using the same closure
            if let Some(win) = web_sys::window() {
                // The closure is stored in `f`, so we can reborrow it here.
                let cb = f.borrow();
                let cb_ref = cb.as_ref().unwrap();
                let _ = win.request_animation_frame(cb_ref.as_ref().unchecked_ref());
            }
        }) as Box<dyn FnMut(f64)>));

        // Kick off the first frame
        let cb_ref = g.borrow();
        let cb_func = cb_ref.as_ref().unwrap();
        let _ = window.request_animation_frame(cb_func.as_ref().unchecked_ref());

        // Store the closure on self so it doesn't get dropped
        // We have to move it out of Rc<RefCell<Option<_>>>. Clone it by taking again.
        self.raf_closure = Some(g.borrow_mut().take().unwrap());
        Ok(())
    }

    /// Stop the RAF loop gracefully.
    pub fn stop(&mut self) -> Result<(), JsValue> {
        self.running = false;
        // Drop the closure reference so JS side can GC it eventually
        self.raf_closure = None;
        Ok(())
    }

    pub fn tick(&mut self, _dt_ms: f32) {
        self.app.update();

        let Some(gfx) = self.gfx.as_mut() else { return; };

        // Handle surface acquisition with basic recovery on Lost/Outdated.
        let frame = match gfx.surface.get_current_texture() {
            Ok(frame) => frame,
            Err(err) => {
                // If surface is lost or outdated, reconfigure it to the current canvas size.
                let needs_reconfig = matches!(
                    err,
                    wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated
                );
                if needs_reconfig {
                    // Update config to current canvas size (in physical pixels)
                    let w = gfx.canvas.width().max(1);
                    let h = gfx.canvas.height().max(1);
                    if gfx.config.width != w || gfx.config.height != h {
                        gfx.config.width = w;
                        gfx.config.height = h;
                    }
                    gfx.surface.configure(&gfx.device, &gfx.config);
                    match gfx.surface.get_current_texture() {
                        Ok(f) => f,
                        Err(e2) => {
                            web_sys::console::error_1(
                                &format!("surface reacquire failed after reconfigure: {e2:?}").into(),
                            );
                            return;
                        }
                    }
                } else if matches!(err, wgpu::SurfaceError::Timeout) {
                    // Skip this frame; try again next RAF tick.
                    return;
                } else {
                    // OutOfMemory or other fatal errors: stop rendering to avoid spinning.
                    web_sys::console::error_1(&format!("fatal surface error: {err:?}").into());
                    self.running = false;
                    return;
                }
            }
        };

        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = gfx
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("ironhold_clear_encoder"),
            });

        {
            let _rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("ironhold_clear_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        // Pick any color you like:
                        load: wgpu::LoadOp::Clear(wgpu::Color {  
                            // Sky blue
                            r: 135.0/255.0, // ~0.529
                            g: 206.0/255.0, // ~0.808
                            b: 235.0/255.0, // ~0.922
                            a: 1.0,
                        }),
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
            // _rpass drops here -> encoder contains the clear commands
        }

        gfx.queue.submit(Some(encoder.finish()));
        frame.present();
    }

    pub fn load_scene_from_ron(&mut self, ron_str: &str) -> Result<(), JsValue> {
        let _scene: Scene =
            Scene::from_ron_str(ron_str).map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(())
    }

    pub fn set_play_mode(&mut self, play: bool) {
        set_mode(&mut self.app, if play { Mode::Play } else { Mode::Edit });
    }
}


// Helper to allow storing closures (not fully used yet)
// struct RcCell<T>(std::rc::Rc<std::cell::RefCell<Option<T>>>);
// impl<T> RcCell<T> { fn new(v: Option<T>) -> Self { Self(std::rc::Rc::new(std::cell::RefCell::new(v))) } }
