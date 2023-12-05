use std::collections::BTreeMap;
use std::sync::Arc;

use eframe::{egui, egui_glow};

mod error;
pub use error::Error;

mod shader;
pub use shader::Shader;

#[derive(Debug, Default)]
pub struct Canvas {
    pub shader: Option<Shader>,
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
                if let Some(program) = &self.shader {
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
                }
            });
    }
}
