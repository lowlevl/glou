use std::time;

use eframe::egui;
use strum::IntoEnumIterator;

use super::{Canvas, UniformStyle};

#[derive(Debug)]
pub struct Tools {
    last_render: time::Instant,
}

impl Default for Tools {
    fn default() -> Self {
        Self {
            last_render: time::Instant::now(),
        }
    }
}

impl Tools {
    pub fn tick(&mut self, ctx: &egui::Context, canvas: &mut Canvas) {
        egui::SidePanel::left("sidebar").show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label("Loaded shader:");
                    ui.monospace(
                        canvas
                            .shader
                            .as_ref()
                            .map(|shader| shader.path().display().to_string())
                            .unwrap_or("(none)".into()),
                    );

                    ui.horizontal(|ui| {
                        ui.label("Frame time:");
                        ui.monospace(format!(
                            "{:.02}ms",
                            self.last_render.elapsed().as_micros() as f32 / 1000.0
                        ));

                        self.last_render = time::Instant::now();
                    });

                    egui::CollapsingHeader::new("⚙ Uniforms")
                        .default_open(true)
                        .show(ui, |ui| {
                            ui.label("Uniform values sent to the fragment shader.");

                            ui.vertical_centered_justified(|ui| {
                                for style in UniformStyle::iter() {
                                    ui.selectable_value(
                                        &mut canvas.uniforms.style,
                                        style,
                                        style.as_ref(),
                                    );
                                }

                                for (name, value) in canvas.uniforms.to_iter() {
                                    ui.horizontal(|ui| {
                                        ui.strong(name);
                                        ui.code(format!("{:.02?}", value));
                                    });
                                }

                                ui.separator();

                                if ui.button("⏳ Reset time").clicked() {
                                    canvas.uniforms.reset_time();
                                }
                            });
                        });

                    ui.collapsing("📖 Reference", |ui| {
                        ui.label("Some documentation about the GLSL methods and types.");
                    });

                    ui.separator();

                    ui.label("Press <L> to toggle live mode.");
                });
            });
        });
    }
}
