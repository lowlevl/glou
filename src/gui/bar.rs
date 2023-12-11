use eframe::egui;

use crate::{Canvas, Shader};

#[derive(Debug, Default)]
pub struct Bar;

impl Bar {
    pub fn show(&self, ctx: &egui::Context, canvas: &mut Canvas) {
        egui::TopBottomPanel::top("bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::widgets::global_dark_light_mode_switch(ui);

                ui.separator();

                ui.menu_button("File", |ui| {
                    if ui.button("Load shader..").clicked() {
                        ui.close_menu();

                        canvas.shader = rfd::FileDialog::new()
                            .set_title("Select fragment shader")
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
}
