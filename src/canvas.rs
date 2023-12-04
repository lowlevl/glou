use eframe::egui;

pub fn draw(ctx: &egui::Context) {
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
