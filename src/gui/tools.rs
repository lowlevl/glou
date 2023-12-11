use eframe::egui;
use strum::IntoEnumIterator;

use crate::{Canvas, UniformStyle};

#[derive(Debug, Default)]
pub struct Tools;

impl Tools {
    pub fn show(&mut self, ctx: &egui::Context, canvas: &mut Canvas) {
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

                    // ui.collapsing("⛶ Renderer", |ui| {
                    //     ui.label("Export the shader to various formats and sinks.");

                    //     ui.add_enabled_ui(canvas.shader.is_some(), |ui| {
                    //         ui.vertical_centered_justified(|ui| {
                    //             ui.horizontal(|ui| {
                    //                 ui.strong("Export size");
                    //                 ui.add(
                    //                     egui::DragValue::new(&mut self.output_size.0)
                    //                         .clamp_range(1..=3840)
                    //                         .suffix(" px"),
                    //                 );
                    //                 ui.label("x");
                    //                 ui.add(
                    //                     egui::DragValue::new(&mut self.output_size.1)
                    //                         .clamp_range(1..=2160)
                    //                         .suffix(" px"),
                    //                 );
                    //             });
                    //         });

                    //         ui.separator();

                    //         ui.vertical_centered_justified(|ui| {
                    //             if ui.button("Capture screenshot").clicked() {}
                    //         });

                    //         ui.separator();

                    //         ui.label("Newtek NDI®");

                    //         ui.vertical_centered_justified(|ui| {
                    //             ui.text_edit_singleline(&mut String::new());

                    //             if ui
                    //                 .button(if self.ndi { "⏹ Stop" } else { "▶ Start" })
                    //                 .clicked()
                    //             {
                    //                 self.ndi = !self.ndi;
                    //             }
                    //         });
                    //     });
                    // });

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
