use eframe::egui;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

mod canvas;
mod gui;

mod renderer;
use renderer::{Renderer, Shader, UniformStyle};

mod error;
use error::Error;

type AllocGuard<T> = scopeguard::ScopeGuard<T, Box<dyn FnOnce(T)>>;

#[macro_export]
macro_rules! guard {
    ($gl:ident, $alloc:expr, $delete:expr) => {
        ::scopeguard::guard($alloc, {
            let $gl = $gl.clone();

            Box::new($delete) as Box<dyn FnOnce(_)>
        })
    };
}

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
    renderer: renderer::Renderer,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Immediately request a redraw of the screen
        ctx.request_repaint();

        let gl = frame
            .gl()
            .expect("Cannot get reference to the underlying `glow` context");

        self.gui.show(ctx, &mut self.renderer);

        if let Some(shader) = &mut self.renderer.shader {
            match shader.rebuild(gl) {
                Ok(success) if success => self.gui.clear_error(),
                Err(err) => {
                    tracing::warn!("An error occured while compiling shader: {err}");

                    self.gui.set_error(err);
                }
                _ => (),
            }
        }

        egui::CentralPanel::default()
            .frame(egui::Frame::canvas(&ctx.style()))
            .show(ctx, |ui| {
                self.renderer.render_to_canvas(gl, ui).paint();
            });

        self.renderer.send(gl);
    }
}
