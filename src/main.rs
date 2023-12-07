use eframe::egui;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

mod canvas;
mod gui;

fn main() -> Result<(), eframe::Error> {
    // Set-up the log and traces handler
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    tracing::info!("Starting application using `eframe` backend");

    // Create the frame and context and run the `App`
    eframe::run_native(
        &format!("glou v{}", env!("CARGO_PKG_VERSION")),
        eframe::NativeOptions {
            multisampling: 4,
            renderer: eframe::Renderer::Glow,
            viewport: egui::ViewportBuilder {
                min_inner_size: Some(egui::vec2(320.0, 240.0)),
                ..Default::default()
            },
            centered: true,
            ..Default::default()
        },
        Box::new(|_| Box::<App>::default()),
    )?;

    Ok(())
}

#[derive(Debug, Default)]
struct App {
    gui: gui::Gui,
    canvas: canvas::Canvas,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let gl = frame
            .gl()
            .expect("Cannot get reference to the underlying `glow` context");

        self.gui.tick(ctx, &mut self.canvas);

        if let Some(shader) = &mut self.canvas.shader {
            match shader.load(gl) {
                Ok(success) if success => self.gui.error = None,
                Err(err) => {
                    tracing::warn!("An error occured while compiling shader: {err}");

                    self.gui.error = Some(err);
                }
                _ => (),
            }
        }
        self.canvas.tick(ctx, gl);

        // Immediately request a redraw of the screen
        ctx.request_repaint();
    }
}
