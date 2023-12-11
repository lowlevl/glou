use eframe::egui;

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
    live_mode: bool,
}

impl Gui {
    pub fn show(&mut self, ctx: &egui::Context, canvas: &mut Canvas) {
        if ctx.input(|i| i.key_pressed(egui::Key::L)) {
            self.live_mode = !self.live_mode;
        }

        if !self.live_mode {
            self.bar.show(ctx, canvas);
            self.tools.show(ctx, canvas);
            self.errors.show(ctx);
        }
    }

    pub fn set_error(&mut self, error: Error) {
        self.errors.0 = Some(error);
    }

    pub fn clear_error(&mut self) {
        self.errors.0 = None;
    }
}
