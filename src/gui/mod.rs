use std::time;

use eframe::egui;

use crate::canvas::{self, Canvas, Shader};

#[derive(Debug)]
pub struct Gui {
    live_mode: bool,
    rendered_at: time::Instant,
    pub error: Option<canvas::Error>,
}

impl Default for Gui {
    fn default() -> Self {
        Self {
            live_mode: Default::default(),
            rendered_at: time::Instant::now(),
            error: Default::default(),
        }
    }
}

impl Gui {
    pub fn tick(&mut self, ctx: &egui::Context, canvas: &mut Canvas) {
        if ctx.input(|i| i.key_pressed(egui::Key::L)) {
            self.live_mode = !self.live_mode;
        }

        if !self.live_mode {
            self.toolbar(ctx, canvas);
            self.sidebar(ctx, canvas);
            self.errors(ctx);
        }
    }

    fn toolbar(&mut self, ctx: &egui::Context, canvas: &mut Canvas) {
        egui::TopBottomPanel::top("bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::widgets::global_dark_light_mode_switch(ui);

                ui.separator();

                ui.menu_button("File", |ui| {
                    if ui.button("Load shader..").clicked() {
                        ui.close_menu();

                        canvas.shader = rfd::FileDialog::new()
                            .set_title("Select fragment shader to load")
                            .pick_file()
                            .map(Shader::new);
                    }

                    if ui.button("Clear shader..").clicked() {
                        ui.close_menu();

                        canvas.shader = None;
                    }

                    ui.separator();

                    if ui.button("Quit").clicked() {
                        ui.close_menu();

                        std::process::exit(0);
                    }
                });
            });
        });
    }

    fn sidebar(&mut self, ctx: &egui::Context, canvas: &Canvas) {
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
                            self.rendered_at.elapsed().as_micros() as f32 / 1000.0
                        ));

                        self.rendered_at = time::Instant::now();
                    });

                    egui::CollapsingHeader::new("⚙ Uniforms")
                        .default_open(true)
                        .show(ui, |ui| {
                            ui.label("Uniform values sent to the fragment shader.");

                            for (name, value) in &canvas.uniforms {
                                ui.horizontal(|ui| {
                                    ui.strong(*name);
                                    ui.code(format!("{:.02?}", value));
                                });
                            }
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

    fn errors(&self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("errors").show(ctx, |ui| {
            egui::CollapsingHeader::new("⚠ Errors")
                .default_open(true)
                .show(ui, |ui| match &self.error {
                    None => ui.label(
                        egui::RichText::new("There are no errors for now ✔")
                            .italics()
                            .weak(),
                    ),
                    Some(error) => {
                        let error = error.to_string();
                        let error = error
                            .strip_suffix("\r\n")
                            .or(error.strip_suffix('\n'))
                            .unwrap_or(&error);

                        ui.monospace(error)
                    }
                });
        });
    }
}
