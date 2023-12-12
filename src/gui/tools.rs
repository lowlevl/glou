use eframe::egui;
use strum::IntoEnumIterator;

use crate::{Renderer, UniformStyle};

#[derive(Debug, Default)]
pub struct Tools;

impl Tools {
    pub fn show(&mut self, ctx: &egui::Context, renderer: &mut Renderer) {
        egui::SidePanel::left("sidebar").show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label("Loaded shader:");
                    ui.monospace(
                        renderer
                            .shader
                            .as_ref()
                            .map(|shader| shader.path().display().to_string())
                            .unwrap_or("(none)".into()),
                    );

                    ui.collapsing("⛶ Render", |ui| {
                        ui.label("Shader external rendering and exporting parameters.");

                        ui.add_enabled_ui(renderer.shader.is_some(), |ui| {
                            ui.vertical_centered_justified(|ui| {
                                ui.horizontal(|ui| {
                                    ui.add_enabled_ui(renderer.resizable, |ui| {
                                        ui.strong("Texture size");
                                        ui.add(
                                            egui::DragValue::new(&mut renderer.size.x)
                                                .clamp_range(320..=3840)
                                                .suffix(" px"),
                                        );
                                        ui.label("x");
                                        ui.add(
                                            egui::DragValue::new(&mut renderer.size.y)
                                                .clamp_range(240..=2160)
                                                .suffix(" px"),
                                        );
                                    });

                                    if ui
                                        .button(if renderer.resizable { "🔓" } else { "🔒" })
                                        .clicked()
                                    {
                                        renderer.resizable = !renderer.resizable;
                                    }
                                });
                            });

                            ui.separator();

                            ui.label("Newtek NDI® Source");

                            ui.horizontal(|ui| {
                                ui.strong("Framerate");
                                ui.add(
                                    egui::DragValue::new(&mut renderer.ndi_framerate)
                                        .clamp_range(24..=240)
                                        .suffix(" fps"),
                                );
                            });

                            ui.vertical_centered_justified(|ui| {
                                ui.add(
                                    egui::TextEdit::singleline(&mut renderer.ndi_name)
                                        .hint_text("Source name"),
                                );

                                match &renderer.ndi {
                                    Some(_ndi) => {
                                        if ui.button("⏹ Stop").clicked() {
                                            renderer.ndi = None;
                                        }
                                    }
                                    None => {
                                        if ui.button("▶ Start").clicked() {
                                            renderer.ndi = Some(
                                                ndi::SendBuilder::new()
                                                    .ndi_name(renderer.ndi_name.clone())
                                                    .clock_video(true)
                                                    .build()
                                                    .expect("Unable to create NDI device"),
                                            );
                                        }
                                    }
                                }
                            });
                        });
                    });

                    egui::CollapsingHeader::new("⚙ Uniforms")
                        .default_open(true)
                        .show(ui, |ui| {
                            ui.label("Uniform values sent to the fragment shader.");

                            ui.vertical_centered_justified(|ui| {
                                for style in UniformStyle::iter() {
                                    ui.selectable_value(
                                        &mut renderer.uniforms.style,
                                        style,
                                        style.as_ref(),
                                    );
                                }

                                for (name, value) in renderer.uniforms.to_iter() {
                                    ui.horizontal(|ui| {
                                        ui.strong(name);
                                        ui.code(format!("{:.02?}", value));
                                    });
                                }

                                ui.separator();

                                if ui.button("⏳ Reset time").clicked() {
                                    renderer.uniforms.reset_time();
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
