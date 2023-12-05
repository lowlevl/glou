use std::collections::BTreeMap;
use std::sync::Arc;

use eframe::{egui, egui_glow, glow::HasContext};

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
            .frame(egui::Frame::canvas(&ctx.style()))
            .show(ctx, |ui| {
                self.paint(ui);
            });
    }

    fn paint(&self, ui: &mut egui::Ui) {
        let (response, painter) =
            ui.allocate_painter(ui.available_size_before_wrap(), egui::Sense::hover());

        if let Some(shader) = &self.shader {
            painter.add(egui::PaintCallback {
                rect: response.rect,
                callback: Arc::new(egui_glow::CallbackFn::new({
                    let shader = shader.clone();

                    move |_, painter| unsafe {
                        shader.draw(painter.gl());
                    }
                })),
            });
        }
    }
}
