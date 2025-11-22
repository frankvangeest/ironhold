/**
 * path: /crates/platform_web/src/lib.rs
 * description: Web platform support for the project, including WebGPU feature detection
 * and hot-reloading capabilities.
 */
pub mod wgpu_init;
pub use wgpu_init::WgpuContext;

use js_sys::ArrayBuffer;
use serde::Deserialize;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    Blob,
    Event,
    FileReader,
    MessageEvent,
    WebSocket,
};

#[derive(Deserialize, Debug)]
struct AssetChanged {
    #[serde(rename = "type")]
    kind: String, // "asset-changed"
    url: String, // e.g., "/assets/foo.ron"
}

// Optional: if you plan multiple message kinds later
#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
enum WsMessage {
    #[serde(rename = "asset-changed")]
    AssetChanged { url: String },
    #[serde(rename = "hello")]
    Hello,
    #[serde(other)]
    Unknown,
}

#[wasm_bindgen]
pub fn webgpu_supported() -> bool {
    // Minimal feature-detection via JS (navigator.gpu)
    js_sys::Reflect::has(&web_sys::window().unwrap(), &JsValue::from_str("navigator"))
        .unwrap_or(false)
}

pub fn compute_ws_url() -> Result<String, JsValue> {
    let win = web_sys::window().ok_or(JsValue::from_str("no window"))?;
    let loc = win.location();

    // protocol: "http:" or "https:"
    let protocol_js = loc.protocol()?;
    let protocol: String = protocol_js.into();
    let ws_scheme = if protocol == "https:" { "wss" } else { "ws" };

    // hostname without port (e.g., "127.0.0.1")
    let hostname_js = loc.hostname()?;
    let hostname: String = hostname_js.into();

    // port may be empty; default to 5173 if so
    let port_js = loc.port()?;
    let port_str: String = port_js.into();
    let http_port: u16 = if port_str.is_empty() {
        5173
    } else {
        port_str.parse::<u16>().unwrap_or(5173)
    };

    let ws_port = http_port.saturating_add(1);
    Ok(format!("{ws_scheme}://{hostname}:{ws_port}/ws"))
}

/// Start a WebSocket connection for hot reload notifications.
pub fn start_hot_reload<F>(on_asset_changed: F) -> Result<(), JsValue>
where
    F: 'static + Fn(String),
{
    let url = compute_ws_url()?; // e.g., ws://127.0.0.1:5174/ws
    let ws = WebSocket::new(&url)?;

    // Keep the user's callback as Rc so our closure remains FnMut (not FnOnce)
    let handler: Rc<dyn Fn(String)> = Rc::new(on_asset_changed);

    // Accept text and binary; if binary, browser will give us Blob
    ws.set_binary_type(web_sys::BinaryType::Blob);

    let onmessage = {
        let handler = handler.clone();
        wasm_bindgen::closure::Closure::<dyn FnMut(MessageEvent)>::new(move |e: MessageEvent| {
            let data = e.data();

            // ---- TEXT branch -------------------------------------------------
            if let Some(s) = data.as_string() {
                match serde_json::from_str::<WsMessage>(&s) {
                    Ok(WsMessage::AssetChanged { url }) => handler.clone()(url),
                    Ok(WsMessage::Hello) => {
                        // Optional debug; normally we keep this quiet
                        web_sys::console::debug_1(&"WS hello received".into());
                    }
                    Ok(WsMessage::Unknown) => {
                        // Ignore unknown types without warning to avoid console noise
                        web_sys::console::debug_1(&format!("WS ignored: {}", s).into());
                    }
                    Err(err) => {
                        // Only malformed JSON should end up here now
                        web_sys::console::warn_1(
                            &format!("WS JSON parse error (text): {err} => {s}").into(),
                        );
                    }
                }
                return;
            }

            // ---- BLOB branch -------------------------------------------------
            if data.is_instance_of::<Blob>() {
                let blob: Blob = data.clone().unchecked_into();
                let fr = FileReader::new().expect("FileReader new");
                let fr_clone = fr.clone();
                let cb = handler.clone();

                let onloadend =
                    wasm_bindgen::closure::Closure::<dyn FnMut(Event)>::new(move |_| {
                        let result = fr_clone.result().unwrap_or_else(|_| JsValue::NULL);
                        if let Some(txt) = result.as_string() {
                            match serde_json::from_str::<WsMessage>(&txt) {
                                Ok(WsMessage::AssetChanged { url }) => cb.clone()(url),
                                Ok(WsMessage::Hello) => {
                                    web_sys::console::debug_1(&"WS hello (blob)".into());
                                }
                                Ok(WsMessage::Unknown) => {
                                    web_sys::console::debug_1(
                                        &format!("WS ignored (blob): {}", txt).into(),
                                    );
                                }
                                Err(err) => {
                                    web_sys::console::warn_1(
                                        &format!("WS JSON parse error (blob): {err} => {txt}")
                                            .into(),
                                    );
                                }
                            }
                        } else {
                            web_sys::console::warn_1(&"WS Blob result not a string".into());
                        }
                    });

                fr.set_onloadend(Some(onloadend.as_ref().unchecked_ref()));
                onloadend.forget();

                if let Err(e) = fr.read_as_text(&blob) {
                    web_sys::console::warn_1(&format!("read_as_text failed: {:?}", e).into());
                }
                return;
            }

            // ---- ARRAYBUFFER branch (for completeness) -----------------------
            if data.is_instance_of::<ArrayBuffer>() {
                // If your server ever sends JSON as binary, decode UTF-8 here.
                web_sys::console::warn_1(
                    &"WS data is ArrayBuffer (binary); ignoring for now".into(),
                );
                return;
            }

            // ---- FALLBACK ----------------------------------------------------
            web_sys::console::debug_1(&"WS data not text/blob/arraybuffer".into());
        })
    };

    ws.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
    onmessage.forget();

    let onopen = wasm_bindgen::closure::Closure::<dyn FnMut(Event)>::new(move |_| {
        web_sys::console::log_1(&"Hot reload WebSocket connected".into());
    });
    ws.set_onopen(Some(onopen.as_ref().unchecked_ref()));
    onopen.forget();

    let onerror = wasm_bindgen::closure::Closure::<dyn FnMut(Event)>::new(move |_| {
        web_sys::console::warn_1(&"Hot reload WebSocket error".into());
    });
    ws.set_onerror(Some(onerror.as_ref().unchecked_ref()));
    onerror.forget();

    Ok(())
}
