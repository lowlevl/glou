use std::{rc::Rc, time};

use eframe::{
    egui,
    glow::{self, HasContext},
};

mod shader;
pub use shader::Shader;

mod uniforms;
pub use uniforms::{UniformStyle, Uniforms};

use crate::canvas::Canvas;

#[derive(Default)]
pub struct Renderer {
    pub uniforms: Uniforms,
    pub shader: Option<Shader>,
    pub buffer: Vec<u8>,

    pub size: egui::Vec2,
    pub resizable: bool,

    pub ndi: Option<nndi::send::Send>,
    pub ndi_name: String,
    pub ndi_framerate: u8,
}

impl std::fmt::Debug for Renderer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Renderer")
            .field("uniforms", &self.uniforms)
            .field("shader", &self.shader)
            .field("size", &self.size)
            .field("resizable", &self.resizable)
            .field("ndi", &())
            .field("ndi_name", &self.ndi_name)
            .field("ndi_framerate", &self.ndi_framerate)
            .finish()
    }
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

        let mut texture = None;

        if let Some(shader) = &self.shader {
            unsafe {
                // Draw shader to right-sized texture
                texture = Some(
                    shader
                        .render_to_texture(gl, &self.uniforms, viewport.size())
                        .expect("Unable to render shader"),
                );
            };
        }

        Canvas::new(texture, painter, viewport)
    }

    fn render_to_buffer(&mut self, gl: &Rc<glow::Context>) -> Option<egui::Vec2> {
        if let Some(shader) = &self.shader {
            let viewport = egui::Rect::from_x_y_ranges(0.0..=self.size.x, 0.0..=self.size.y);
            self.uniforms.update(viewport, None);

            let texture = unsafe {
                // Draw shader to right-sized texture
                shader
                    .render_to_texture(gl, &self.uniforms, viewport.size())
                    .expect("Unable to render shader")
            };

            self.buffer.resize(
                viewport.width() as usize * viewport.height() as usize * 4,
                0,
            );

            unsafe {
                gl.bind_texture(glow::TEXTURE_2D, Some(*texture));
                gl.get_tex_image(
                    glow::TEXTURE_2D,
                    0,
                    glow::RGBA,
                    glow::UNSIGNED_BYTE,
                    glow::PixelPackData::Slice(&mut self.buffer),
                );
                gl.bind_texture(glow::TEXTURE_2D, None);
            };

            Some(viewport.size())
        } else {
            None
        }
    }

    // Reverse the Y-axis because it seems
    // NDI expects origin to be top-left while OpenGL places it bottom-left
    // let mut pixels = pixels
    //     .chunks(viewport.width() as usize * 4)
    //     .rev()
    //     .flatten()
    //     .copied()
    //     .collect::<Vec<_>>();

    pub fn send(&mut self, gl: &Rc<glow::Context>) {
        if self.ndi.is_some() {
            if let Some(size) = self.render_to_buffer(gl) {
                if let Some(ndi) = &self.ndi {
                    //ndi.send_video(&ndi::VideoData::from_buffer(
                    //    size.x as i32,
                    //    size.y as i32,
                    //    ndi::FourCCVideoType::RGBA,
                    //    self.ndi_framerate.into(),
                    //    1,
                    //    ndi::FrameFormatType::Progressive,
                    //    time::SystemTime::now()
                    //        .duration_since(time::UNIX_EPOCH)
                    //        .expect("Time went backwards >.>")
                    //        .as_secs() as i64,
                    //    0,
                    //    None,
                    //    &mut self.buffer,
                    //))
                }
            }
        }
    }
}
