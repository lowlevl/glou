use std::rc::Rc;

use eframe::{egui, glow};

mod bar;
use bar::Bar;

mod errors;
use errors::Errors;

mod tools;
use tools::Tools;

use super::{Canvas, Error};

#[derive(Debug, Default)]
pub struct Gui {
    bar: Bar,
    tools: Tools,
    errors: Errors,
    canvas: Canvas,
    live_mode: bool,
}

impl Gui {
    pub fn show(&mut self, ctx: &egui::Context, gl: &Rc<glow::Context>) {
        if ctx.input(|i| i.key_pressed(egui::Key::L)) {
            self.live_mode = !self.live_mode;
        }

        if !self.live_mode {
            self.bar.show(ctx, &mut self.canvas);
            self.tools.show(ctx, &mut self.canvas);
            self.errors.show(ctx);
        }

        if let Some(shader) = &mut self.canvas.shader {
            match shader.rebuild(gl) {
                Ok(success) if success => self.errors.clear(),
                Err(err) => {
                    tracing::warn!("An error occured while compiling shader: {err}");

                    self.errors.set(err);
                }
                _ => (),
            }
        }

        egui::CentralPanel::default()
            .frame(egui::Frame::canvas(&ctx.style()))
            .show(ctx, |ui| {
                self.canvas.paint(ui, gl);
            });
    }
}
