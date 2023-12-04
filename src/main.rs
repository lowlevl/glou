use eframe::egui;

mod canvas;
mod gui;

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
    gui: gui::Gui,
    canvas: canvas::Canvas,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.gui.tick(ctx);
        self.canvas.tick(ctx);
    }
}
