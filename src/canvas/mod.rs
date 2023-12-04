use std::collections::BTreeMap;
use std::sync::Arc;

use eframe::{egui, egui_glow};

mod error;
pub use error::Error;

mod program;
pub use program::Program;

#[derive(Debug, Default)]
pub struct Canvas {
    uniforms: BTreeMap<String, Vec<f32>>,
}

impl Canvas {
    pub fn tick(&mut self, ctx: &egui::Context) {
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

                painter.add(egui::PaintCallback {
                    rect: painter.clip_rect(),
                    callback: Arc::new(egui_glow::CallbackFn::new(|_, painter| unsafe {
                        Program::new(painter.gl()).unwrap().draw()
                    })),
                });
            });
    }
}
