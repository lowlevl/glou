use std::rc::Rc;

use eframe::{egui, glow};

mod canvas;
use canvas::{Canvas, Shader, UniformStyle};

mod bar;
use bar::Bar;

mod errors;
use errors::Errors;

mod tools;
use tools::Tools;

use super::Error;

#[derive(Debug, Default)]
pub struct Gui {
    bar: Bar,
    tools: Tools,
    canvas: Canvas,
    errors: Errors,
    live_mode: bool,
}

impl Gui {
    pub fn tick(&mut self, ctx: &egui::Context, gl: &Rc<glow::Context>) {
        if ctx.input(|i| i.key_pressed(egui::Key::L)) {
            self.live_mode = !self.live_mode;
        }

        if !self.live_mode {
            self.bar.tick(ctx, &mut self.canvas);
            self.tools.tick(ctx, &mut self.canvas);
            self.errors.tick(ctx);
        }

        if let Some(shader) = &mut self.canvas.shader {
            match shader.load(gl) {
                Ok(success) if success => self.errors.clear(),
                Err(err) => {
                    tracing::warn!("An error occured while compiling shader: {err}");

                    self.errors.show(err);
                }
                _ => (),
            }
        }
        self.canvas.tick(ctx, gl);
    }
}
