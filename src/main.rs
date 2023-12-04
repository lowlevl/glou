use std::{collections::BTreeMap, path::PathBuf};

use eframe::egui;

mod canvas;
mod ui;

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        &format!("glou v{}", env!("CARGO_PKG_VERSION")),
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder {
                min_inner_size: Some(egui::vec2(320.0, 240.0)),
                ..Default::default()
            },
            centered: true,
            ..Default::default()
        },
        Box::new(|_| Box::<App>::default()),
    )
}

#[derive(Debug, Default)]
struct App {
    path: Option<PathBuf>,
    uniforms: BTreeMap<String, Vec<f32>>,
    live_mode: bool,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if ctx.input(|i| i.key_pressed(egui::Key::L)) {
            self.live_mode = !self.live_mode;
        }

        if !self.live_mode {
            ui::draw(self, ctx);
        }
        canvas::draw(ctx);
    }
}
