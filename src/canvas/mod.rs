use std::sync::Arc;
use std::{collections::BTreeMap, rc::Rc};

use eframe::{egui, egui_glow, glow};

mod error;
pub use error::Error;

mod program;
pub use program::Program;

#[derive(Debug, Default)]
pub struct Canvas {
    uniforms: BTreeMap<String, Vec<f32>>,
}

impl Canvas {
    pub fn tick(&mut self, ctx: &egui::Context, gl: &Rc<glow::Context>) {
        egui::CentralPanel::default()
            .frame(egui::Frame {
                inner_margin: egui::Margin::ZERO,
                ..Default::default()
            })
            .show(ctx, |ui| {
                let program = unsafe { Program::new(gl).unwrap() };

                let painter = egui::Painter::new(
                    ui.ctx().clone(),
                    ui.layer_id(),
                    ui.available_rect_before_wrap(),
                );

                painter.add(egui::PaintCallback {
                    rect: painter.clip_rect(),
                    callback: Arc::new(egui_glow::CallbackFn::new({
                        let program = program.clone();

                        move |_, painter| unsafe {
                            program.draw(painter.gl());
                        }
                    })),
                });
            });
    }
}
