
use eframe::egui;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    let web_options = eframe::WebOptions::default();
    wasm_bindgen_futures::spawn_local(async move {
        eframe::WebRunner::new()
            .start("editor_canvas", web_options, Box::new(|_| Box::<EditorApp>::default()))
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
            if ui.button(if self.show_play {"Stop"} else {"Play"}).clicked() {
                self.show_play = !self.show_play;
            }
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Viewport placeholder — engine frame will render here");
            let avail = ui.available_size();
            let (rect, _resp) = ui.allocate_exact_size(avail, egui::Sense::hover());
            let painter = ui.painter_at(rect);
            painter.rect_filled(rect, 0.0, egui::Color32::from_rgb(22,22,22));
        });
    }
}
