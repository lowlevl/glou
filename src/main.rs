use std::collections::BTreeMap;

use eframe::egui;

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
    path: Option<String>,
    uniforms: BTreeMap<String, Vec<f32>>,
    live_mode: bool,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if ctx.input(|i| i.key_pressed(egui::Key::L)) {
            self.live_mode = !self.live_mode;
        }

        if !self.live_mode {
            egui::TopBottomPanel::top("bar").show(ctx, |ui| {
                egui::menu::bar(ui, |ui| {
                    ui.menu_button("File", |ui| {});
                });
            });

            egui::SidePanel::left("tools").show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label("Currently loaded shader:");
                    ui.monospace(self.path.as_deref().unwrap_or("(none)"));

                    ui.collapsing("Uniforms", |ui| {
                        ui.label("Uniforms value and types provided to the shader.");
                    });

                    ui.collapsing("Reference", |ui| {
                        ui.label("Some documentation about the GLSL methods and types.");
                    });

                    ui.separator();

                    ui.label("Press <L> to toggle live mode.");
                });
            });

            egui::TopBottomPanel::bottom("errors").show(ctx, |ui| {
                ui.collapsing("Errors ⚠", |ui| {
                    ui.monospace("There are no errors for now ✔");
                });
            });
        }

        egui::CentralPanel::default()
            .frame(egui::Frame {
                inner_margin: egui::Margin::ZERO,
                ..Default::default()
            })
            .show(ctx, |ui| {
                let painter = egui::Painter::new(
                    ui.ctx().clone(),
                    ui.layer_id(),
                    ui.available_rect_before_wrap(),
                );

                // GLSL goes here
                painter.rect_filled(
                    painter.clip_rect(),
                    eframe::epaint::Rounding::ZERO,
                    eframe::epaint::Color32::BLACK,
                );

                ui.expand_to_include_rect(painter.clip_rect());
            });
    }
}
