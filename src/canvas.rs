use std::collections::BTreeMap;
use std::sync::Arc;

use eframe::{
    egui,
    egui_glow::{self, glow},
    glow::HasContext,
};

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
                        Self::draw(painter.gl());
                    })),
                });

                ui.expand_to_include_rect(painter.clip_rect());
            });
    }

    unsafe fn draw(gl: &glow::Context) {
        let program = gl.create_program().unwrap();
    }
}
