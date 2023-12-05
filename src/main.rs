use eframe::egui;

mod canvas;
mod gui;

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        &format!("glou v{}", env!("CARGO_PKG_VERSION")),
        eframe::NativeOptions {
            renderer: eframe::Renderer::Glow,
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
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.gui.tick(ctx, &mut self.canvas);

        if let Some(shader) = &mut self.canvas.shader {
            shader
                .load(
                    frame
                        .gl()
                        .expect("Cannot get reference to the underlying `glow` context"),
                )
                .unwrap()
        }
        self.canvas.tick(ctx);
    }
}
