use eframe::egui;

use crate::canvas::{self, Canvas, Shader};

#[derive(Debug, Default)]
pub struct Gui {
    pub error: Option<canvas::Error>,
    live: bool,
}

impl Gui {
    pub fn tick(&mut self, ctx: &egui::Context, canvas: &mut Canvas) {
        if ctx.input(|i| i.key_pressed(egui::Key::L)) {
            self.live = !self.live;
        }

        if !self.live {
            self.toolbar(ctx, canvas);
            self.sidebar(ctx, canvas);
            self.errors(ctx);
        }
    }

    fn toolbar(&mut self, ctx: &egui::Context, canvas: &mut Canvas) {
        egui::TopBottomPanel::top("bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Load shader..").clicked() {
                        ui.close_menu();

                        canvas.shader = rfd::FileDialog::new()
                            .set_title("Select fragment shader")
                            .pick_file()
                            .map(Shader::new);
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

    fn sidebar(&self, ctx: &egui::Context, canvas: &Canvas) {
        egui::SidePanel::left("sidebar").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label("Currently loaded shader:");
                ui.monospace(
                    canvas
                        .shader
                        .as_ref()
                        .map(|shader| shader.path().display().to_string())
                        .unwrap_or("(none)".into()),
                );

                ui.collapsing("Uniforms", |ui| {
                    ui.label("Uniforms value and types provided to the shader.");

                    for (name, value) in &canvas.uniforms {
                        ui.horizontal(|ui| {
                            ui.strong(*name);
                            ui.code(format!("{:.02?}", value));
                        });
                    }
                });

                ui.collapsing("Reference", |ui| {
                    ui.label("Some documentation about the GLSL methods and types.");
                });

                ui.separator();

                ui.label("Press <L> to toggle live mode.");
            });
        });
    }

    fn errors(&self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("errors").show(ctx, |ui| {
            ui.collapsing("Errors ⚠", |ui| match &self.error {
                None => ui.label(
                    egui::RichText::new("There are no errors for now ✔")
                        .italics()
                        .weak(),
                ),
                Some(error) => ui.monospace(error.to_string()),
            });
        });
    }
}
