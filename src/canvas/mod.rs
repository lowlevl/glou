use std::{collections::HashMap, sync::Arc, time};

use eframe::{egui, egui_glow};

mod error;
pub use error::Error;

mod shader;
pub use shader::Shader;

#[derive(Debug, Default)]
pub struct Canvas {
    pub shader: Option<Shader>,
    pub time: Option<time::Instant>,
    pub uniforms: HashMap<&'static str, Vec<f32>>,
}

impl Canvas {
    pub fn tick(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default()
            .frame(egui::Frame::canvas(&ctx.style()))
            .show(ctx, |ui| {
                self.paint(ui);
            });
    }

    fn paint(&mut self, ui: &mut egui::Ui) {
        let (response, painter) =
            ui.allocate_painter(ui.available_size_before_wrap(), egui::Sense::hover());

        self.uniforms.insert(
            "u_resolution",
            vec![response.rect.width(), response.rect.height()],
        );
        if let Some(pos) = response.hover_pos() {
            self.uniforms.insert(
                "u_mouse",
                vec![pos.x - response.rect.left(), response.rect.bottom() - pos.y],
            );
        }
        self.uniforms.insert(
            "u_time",
            vec![self
                .time
                .get_or_insert_with(time::Instant::now)
                .elapsed()
                .as_secs_f32()],
        );

        if let Some(shader) = &self.shader {
            painter.add(egui::PaintCallback {
                rect: response.rect,
                callback: Arc::new(egui_glow::CallbackFn::new({
                    let shader = shader.clone();
                    let uniforms = self.uniforms.clone();

                    move |_, painter| unsafe {
                        shader.draw(painter.gl(), &uniforms);
                    }
                })),
            });
        }
    }
}
