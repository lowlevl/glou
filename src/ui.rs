use eframe::egui;

use super::App;

pub fn draw(app: &mut App, ctx: &egui::Context) {
    bar(app, ctx);
    sidebar(app, ctx);
    errors(ctx);
}

fn bar(app: &mut App, ctx: &egui::Context) {
    egui::TopBottomPanel::top("bar").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Load shader..").clicked() {
                    ui.close_menu();

                    app.path = rfd::FileDialog::new()
                        .set_title("Select fragment shader")
                        .pick_file();
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

fn sidebar(app: &App, ctx: &egui::Context) {
    egui::SidePanel::left("sidebar").show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.label("Currently loaded shader:");
            ui.monospace(
                app.path
                    .as_ref()
                    .map(|path| path.display().to_string())
                    .unwrap_or("(none)".into()),
            );

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
}

fn errors(ctx: &egui::Context) {
    egui::TopBottomPanel::bottom("errors").show(ctx, |ui| {
        ui.collapsing("Errors ⚠", |ui| {
            ui.monospace("There are no errors for now ✔");
        });
    });
}
