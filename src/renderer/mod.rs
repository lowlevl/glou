use std::rc::Rc;

use eframe::{
    egui,
    glow::{self, HasContext},
};

mod shader;
pub use shader::Shader;

mod uniforms;
pub use uniforms::{UniformStyle, Uniforms};

use crate::canvas::Canvas;

#[derive(Debug, Default)]
pub struct Renderer {
    pub uniforms: Uniforms,
    pub shader: Option<Shader>,
    pub texture: Option<glow::Texture>,

    pub size: egui::Vec2,
    pub resizable: bool,

    pub ndi_name: String,
    pub ndi_enabled: bool,
}

impl Renderer {
    pub fn render_to_canvas(&mut self, gl: &Rc<glow::Context>, ui: &mut egui::Ui) -> Canvas {
        let (response, painter) = ui.allocate_painter(ui.available_size(), egui::Sense::hover());
        let viewport = egui::Rect {
            min: painter.round_pos_to_pixels(painter.clip_rect().min),
            max: painter.round_pos_to_pixels(painter.clip_rect().max),
        };

        if !self.resizable {
            self.size = viewport.size();
        }

        self.uniforms.update(
            viewport,
            response
                .hover_pos()
                .map(|pos| painter.round_pos_to_pixels(pos)),
        );

        if let Some(shader) = &self.shader {
            unsafe {
                if let Some(texture) = self.texture {
                    tracing::trace!("Freed memory for cached texture {texture:?}");

                    gl.delete_texture(texture);
                }

                // Draw shader to right-sized texture
                self.texture = shader
                    .render_to_texture(gl, &self.uniforms, viewport.size())
                    .expect("Unable to render shader");
            };
        }

        Canvas::new(self.texture, painter, viewport)
    }
}
