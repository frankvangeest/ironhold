/**
 * path: /crates/engine_wasm_api/src/lib.rs
 * description: WASM bindings for the engine using wasm-bindgen.
 */
use crate::js_sys::Date;
use engine_core::{
    set_mode,
    EngineApp,
    Mode,
};
use engine_scene::Scene;
use js_sys;
use platform_web::WgpuContext;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;
use engine_render::{QuadPipeline, InstanceData};

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
    raf_handle: Option<
        std::rc::Rc<std::cell::RefCell<Option<wasm_bindgen::closure::Closure<dyn FnMut(f64)>>>>,
    >,
    pipeline: Option<QuadPipeline>,
    current_scene: Option<engine_scene::Scene>,
    running: bool,
    last_ts: f64,
}

#[wasm_bindgen]
impl Engine {
    pub async fn mount_async(&mut self) -> Result<(), JsValue> {
        let canvas = self
            .canvas
            .take()
            .ok_or(JsValue::from_str("no canvas bound"))?;
        let gfx = platform_web::wgpu_init::init_wgpu(canvas).await?;

        // Build initial instance data (empty or from current_scene)
        let instances: Vec<InstanceData> = if let Some(scene) = &self.current_scene {
            scene_to_instances(scene) // mapping helper (see below)
        } else {
            Vec::new()
        };

        let quad = QuadPipeline::new(&gfx.device, gfx.config.format, &instances);
        self.pipeline = Some(quad);
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
        EngineOptions {
            canvas_id: None,
            assets_base_url: None,
            enable_2d: true,
            enable_3d: true,
        }
    }
    pub fn canvas_id(mut self, id: String) -> Self {
        self.canvas_id = Some(id);
        self
    }
    pub fn assets_base_url(mut self, url: String) -> Self {
        self.assets_base_url = Some(url);
        self
    }
    pub fn enable_2d(mut self, v: bool) -> Self {
        self.enable_2d = v;
        self
    }
    pub fn enable_3d(mut self, v: bool) -> Self {
        self.enable_3d = v;
        self
    }
}

#[wasm_bindgen]
pub async fn init(opts: EngineOptions) -> Result<Engine, JsValue> {
    console_error_panic_hook::set_once();

    // Log build info
    web_sys::console::log_1(
        &format!(
            "Ironhold build {} ({} @ {})",
            build_id(),
            build_sha(),
            build_time()
        )
        .into(),
    );

    let mut canvas: Option<HtmlCanvasElement> = None;
    if let Some(id) = opts.canvas_id.clone() {
        let window = web_sys::window().ok_or_else(|| JsValue::from_str("no window"))?;
        let doc = window
            .document()
            .ok_or_else(|| JsValue::from_str("no document"))?;
        let el = doc
            .get_element_by_id(&id)
            .ok_or_else(|| JsValue::from_str("canvas not found"))?;
        canvas = Some(
            el.dyn_into::<HtmlCanvasElement>()
                .map_err(|_| JsValue::from_str("bad canvas"))?,
        );
    }

    Ok(Engine {
        app: EngineApp::default(),
        canvas,
        gfx: None,
        raf_handle: None,
        pipeline: None,
        running: false,
        last_ts: Date::now(), // milliseconds
        current_scene: None,
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

    // Reconfigure the WebGPU surface to match the current canvas size.
    // This should be called on window resize or when surface acquisition fails.
    pub fn reconfigure_surface(&mut self) {
        if let Some(gfx) = self.gfx.as_mut() {
            let old_w = gfx.config.width;
            let old_h = gfx.config.height;
            let new_w = gfx.canvas.width().max(1);
            let new_h = gfx.canvas.height().max(1);

            if old_w != new_w || old_h != new_h {
                web_sys::console::log_1(
                    &format!("Surface resize detected: {old_w}x{old_h} -> {new_w}x{new_h}").into(),
                );
            }

            platform_web::wgpu_init::reconfigure_surface(gfx);
        } else {
            web_sys::console::warn_1(&"reconfigure_surface called but gfx is None".into());
        }
    }

    /// Start the requestAnimationFrame loop.
    pub fn start(&mut self) -> Result<(), JsValue> {
        if self.running {
            return Ok(());
        }
        self.running = true;

        let window = web_sys::window().ok_or("no window")?;
        // Raw pointer so the closure can call back into self
        let engine_ptr: *mut Engine = self as *mut _;

        // Shared handle that will keep the closure alive.
        use std::cell::RefCell;
        use std::rc::Rc;
        let f: Rc<RefCell<Option<wasm_bindgen::closure::Closure<dyn FnMut(f64)>>>> =
            Rc::new(RefCell::new(None));
        let f_for_closure = Rc::clone(&f);

        // Install the closure
        *f.borrow_mut() = Some(wasm_bindgen::closure::Closure::wrap(
            Box::new(move |ts_ms: f64| {
                // SAFETY: Engine lives as long as start()/stop() contract
                let engine: &mut Engine = unsafe { &mut *engine_ptr };
                if !engine.running {
                    return; // allow graceful stop
                }

                let dt_ms = (ts_ms - engine.last_ts) as f32;
                engine.last_ts = ts_ms;
                engine.tick(dt_ms);

                if let Some(win) = web_sys::window() {
                    // Borrow immutably just to pass the same JS function back to RAF
                    if let Some(cb) = f_for_closure.borrow().as_ref() {
                        let _ = win.request_animation_frame(cb.as_ref().unchecked_ref());
                    }
                }
            }) as Box<dyn FnMut(f64)>,
        ));

        // Kick off the first frame using the same handle.
        {
            let cb_ref = f.borrow();
            let cb_func = cb_ref
                .as_ref()
                .ok_or(JsValue::from_str("RAF closure missing"))?;
            let _ = window.request_animation_frame(cb_func.as_ref().unchecked_ref());
        } // immutable borrow ends here

        // Keep the Rc alive on self (do not move the closure out)
        self.raf_handle = Some(f);

        Ok(())
    }

    /// Stop the RAF loop gracefully.
    pub fn stop(&mut self) -> Result<(), JsValue> {
        self.running = false;
        if let Some(handle) = self.raf_handle.take() {
            // Drop the JS closure so the browser can GC it
            *handle.borrow_mut() = None;
        }
        Ok(())
    }

    // pub fn start_hot_reload() {
    //     if let Some(ws) = platform_web::hotreload::start_ws("ws://127.0.0.1:5174/ws") {
    //         let onmessage = Closure::wrap(Box::new(move |e: web_sys::MessageEvent| {
    //             web_sys::console::log_1(&format!("HotReload message: {:?}", e.data()).into());
    //         }) as Box<dyn FnMut(_)>);
    //         ws.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
    //         onmessage.forget();
    //     } else {
    //         web_sys::console::warn_1(&"HotReload WS not available".into());
    //     }
    // }

    /// Start the Hot Reload WebSocket and register a simple asset-changed handler.
    #[wasm_bindgen]
    pub fn start_hot_reload(&mut self) -> Result<(), JsValue> {
        platform_web::start_hot_reload(|url: String| {
            web_sys::console::log_1(&format!("Hot reload: asset changed at {url}").into());
            // TODO: integrate with engine_assets::hot_reload_stub(url);
        })
    }

    pub fn tick(&mut self, _dt_ms: f32) {
        self.app.update();

        let Some(gfx) = self.gfx.as_mut() else {
            return;
        };

        // Handle surface acquisition with basic recovery on Lost/Outdated.
        let frame = match gfx.surface.get_current_texture() {
            Ok(frame) => frame,
            Err(err) => {
                web_sys::console::warn_1(
                    &format!("surface acquire error, reconfiguring: {err:?}").into(),
                );
                platform_web::wgpu_init::reconfigure_surface(gfx);

                match gfx.surface.get_current_texture() {
                    Ok(f) => f,
                    Err(e2) => {
                        web_sys::console::error_1(
                            &format!("acquire failed after reconfigure: {e2:?}").into(),
                        );
                        return;
                    }
                }
            }
        };

        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = gfx
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("ironhold_encoder"),
            });

        // Update instances on GPU
        if let (Some(pip), Some(scene)) = (self.pipeline.as_mut(), self.current_scene.as_ref()) {
            let instances = scene_to_instances(scene);
            pip.ensure_capacity(&gfx.device, &instances);
            pip.update_instances(&gfx.queue, &instances);

            // Render pass: clear + draw quads
            {
                let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("ironhold_render_pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        depth_slice: None,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color { 
                                r: 135.0/255.0, 
                                g: 206.0/255.0, 
                                b: 235.0/255.0, a: 1.0 
                            }),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    occlusion_query_set: None,
                    timestamp_writes: None,
                });
                pip.draw(&mut rpass);
            }
        }

        gfx.queue.submit(Some(encoder.finish()));
        frame.present();
    }

    pub fn load_scene_from_ron(&mut self, ron_str: &str) -> Result<(), JsValue> {
        let scene: Scene =
            Scene::from_ron_str(ron_str).map_err(|e| JsValue::from_str(&e.to_string()))?;
        self.current_scene = Some(scene);
        Ok(())
    }

    pub fn set_play_mode(&mut self, play: bool) {
        set_mode(&mut self.app, if play { Mode::Play } else { Mode::Edit });
    }
}

// Helper to allow storing closures (not fully used yet)
// struct RcCell<T>(std::rc::Rc<std::cell::RefCell<Option<T>>>);
// impl<T> RcCell<T> { fn new(v: Option<T>) -> Self { Self(std::rc::Rc::new(std::cell::RefCell::new(v))) } }



pub fn scene_to_instances(scene: &engine_scene::Scene) -> Vec<engine_render::InstanceData> {
    scene
        .entities
        .iter()
        .map(|e| {
            // Rotation is authored in degrees in RON; WGSL expects radians.
            let rot_rad = e.transform.rotation.to_radians();

            engine_render::InstanceData {
                transform: engine_render::Transform {
                    // t0: position.x, position.y, rotation(rad), pad
                    t0: [
                        e.transform.position.0,
                        e.transform.position.1,
                        rot_rad,
                        0.0,
                    ],
                    // t1: scale.x, scale.y, pad, pad
                    t1: [
                        e.transform.scale.0,
                        e.transform.scale.1,
                        0.0,
                        0.0,
                    ],
                },
                sprite: engine_render::Sprite {
                    // s0: dimensions.x, dimensions.y, pad, pad
                    s0: [
                        e.sprite.dimensions.0,
                        e.sprite.dimensions.1,
                        0.0,
                        0.0,
                    ],
                    // RGBA
                    color: [
                        e.sprite.color.0,
                        e.sprite.color.1,
                        e.sprite.color.2,
                        e.sprite.color.3,
                    ],
                },
            }
        })
        .collect()
}

