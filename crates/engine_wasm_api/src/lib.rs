
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;
use engine_core::{EngineApp, Mode, set_mode};
use engine_scene::Scene;

#[wasm_bindgen]
pub struct Engine {
    app: EngineApp,
    canvas: Option<HtmlCanvasElement>,
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
    // Resolve canvas if provided
    let mut canvas: Option<HtmlCanvasElement> = None;
    if let Some(id) = opts.canvas_id.clone() {
        let window = web_sys::window().ok_or_else(|| JsValue::from_str("no window"))?;
        let doc = window.document().ok_or_else(|| JsValue::from_str("no document"))?;
        let el = doc.get_element_by_id(&id).ok_or_else(|| JsValue::from_str("canvas not found"))?;
        canvas = Some(el.dyn_into::<HtmlCanvasElement>().map_err(|_| JsValue::from_str("bad canvas"))?);
    }

    let engine = Engine { app: EngineApp::default(), canvas };
    Ok(engine)
}

#[wasm_bindgen]
impl Engine {
    pub fn mount(&mut self) -> Result<(), JsValue> {
        // In the real impl, configure surface with wgpu and link to canvas
        if self.canvas.is_none() {
            return Err(JsValue::from_str("no canvas bound"));
        }
        Ok(())
    }

    pub fn start(&mut self) -> Result<(), JsValue> {
        // RequestAnimationFrame loop stub
        let f = RcCell::new(None);
        let g = RcCell::new(None);
        let mut last = js_sys::Date::now();
        let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
            let now = js_sys::Date::now();
            let _dt = (now - last) as f32; last = now;
            // TODO: call into self.tick via a static mut or store somewhere accessible
        }) as Box<dyn FnMut()>);
        // can't actually schedule without access to window here; stub
        std::mem::forget(closure);
        Ok(())
    }

    pub fn tick(&mut self, _dt_ms: f32) { self.app.update(); }

    pub fn load_scene_from_ron(&mut self, ron_str: &str) -> Result<(), JsValue> {
        let _scene: Scene = Scene::from_ron_str(ron_str).map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(())
    }

    pub fn set_play_mode(&mut self, play: bool) { set_mode(&mut self.app, if play { Mode::Play } else { Mode::Edit }); }
}

// Helper to allow storing closures (not fully used yet)
struct RcCell<T>(std::rc::Rc<std::cell::RefCell<Option<T>>>);
impl<T> RcCell<T> { fn new(v: Option<T>) -> Self { Self(std::rc::Rc::new(std::cell::RefCell::new(v))) } }
