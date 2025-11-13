// /**
//  * path: /apps/editor_web/src/lib.rs
//  * description: Web-based editor application using eframe and WASM.
//  */

// Only compile this file’s content on wasm32:

#![cfg(target_arch = "wasm32")]
use eframe::egui;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlCanvasElement;

use eframe::web::WebRunner;
use eframe::WebOptions;


#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let web_options = WebOptions::default();

    spawn_local(async move {
        // Find the canvas
        let window = web_sys::window().expect("no window");
        let document = window.document().expect("no document");
        let element = document
            .get_element_by_id("editor_canvas")
            .expect("#editor_canvas not found");
        let canvas: HtmlCanvasElement = element
            .dyn_into::<HtmlCanvasElement>()
            .expect("#editor_canvas is not a canvas");

        WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|_cc| {
                    Ok::<Box<dyn eframe::App>, Box<dyn std::error::Error + Send + Sync>>(
                        Box::new(EditorApp::default()),
                    )
                }),
            )
            .await
            .expect("failed to start eframe");
    });

    Ok(())
}

#[derive(Default)]
struct EditorApp {
    show_play: bool,
}

impl eframe::App for EditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            ui.heading("Ironhold Editor (Web)");
            if ui.button(if self.show_play { "Stop" } else { "Play" }).clicked() {
                self.show_play = !self.show_play;
            }
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Viewport placeholder — engine frame will render here");
            let avail = ui.available_size();
            let (rect, _resp) = ui.allocate_exact_size(avail, egui::Sense::hover());
            ui.painter_at(rect)
                .rect_filled(rect, 0.0, egui::Color32::from_rgb(22, 22, 22));
        });
        ctx.request_repaint();
    }
}