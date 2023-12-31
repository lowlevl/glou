use eframe::egui;

use super::Error;

#[derive(Debug, Default)]
pub struct Errors(pub Option<Error>);

impl Errors {
    pub fn show(&self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("errors").show(ctx, |ui| {
            egui::CollapsingHeader::new("⚠ Errors")
                .default_open(true)
                .show(ui, |ui| match &self.0 {
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
