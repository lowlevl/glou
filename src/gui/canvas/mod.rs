use std::{rc::Rc, sync::Arc, time};

use eframe::{
    egui, egui_glow,
    glow::{self, HasContext},
};

mod shader;
pub use shader::Shader;

mod uniforms;
pub use uniforms::{UniformStyle, Uniforms};

#[derive(Debug, Default)]
pub struct Canvas {
    pub shader: Option<Shader>,
    pub time: Option<time::Instant>,
    pub uniforms: Uniforms,
}

impl Canvas {
    pub fn tick(&mut self, ctx: &egui::Context, gl: &Rc<glow::Context>) {
        egui::CentralPanel::default()
            .frame(egui::Frame::canvas(&ctx.style()))
            .show(ctx, |ui| {
                self.paint(ui, gl);
            });
    }

    fn paint(&mut self, ui: &mut egui::Ui, gl: &Rc<glow::Context>) {
        let (response, painter) = ui.allocate_painter(ui.available_size(), egui::Sense::hover());
        let viewport = egui::Rect {
            min: painter.round_pos_to_pixels(painter.clip_rect().min),
            max: painter.round_pos_to_pixels(painter.clip_rect().max),
        };

        // Update uniforms from from viewport and mouse position
        self.uniforms.update(
            viewport,
            response
                .hover_pos()
                .map(|pos| painter.round_pos_to_pixels(pos)),
        );

        if let Some(shader) = &self.shader {
            // Draw shader to right-sized texture
            let texture = unsafe {
                shader
                    .render_to_texture(
                        gl,
                        &self.uniforms,
                        egui::vec2(viewport.width(), viewport.height()),
                    )
                    .expect("Unable to render shader")
            };

            // Finally paint the texture on the screen
            if let Some(texture) = texture {
                painter.add(egui::PaintCallback {
                    rect: viewport,
                    callback: Arc::new(egui_glow::CallbackFn::new({
                        move |info, painter| unsafe {
                            let buffer = painter
                                .gl()
                                .create_framebuffer()
                                .expect("Unable to create frame buffer");

                            painter
                                .gl()
                                .bind_framebuffer(glow::READ_FRAMEBUFFER, Some(buffer));
                            painter.gl().framebuffer_texture_2d(
                                glow::READ_FRAMEBUFFER,
                                glow::COLOR_ATTACHMENT0,
                                glow::TEXTURE_2D,
                                Some(texture),
                                0,
                            );
                            painter.gl().read_buffer(glow::COLOR_ATTACHMENT0);

                            assert!(
                                painter
                                    .gl()
                                    .check_framebuffer_status(glow::READ_FRAMEBUFFER)
                                    == glow::FRAMEBUFFER_COMPLETE
                            );

                            let viewport = info.viewport_in_pixels();
                            painter.gl().blit_framebuffer(
                                0,
                                0,
                                viewport.width_px,
                                viewport.height_px,
                                viewport.left_px,
                                viewport.from_bottom_px,
                                viewport.left_px + viewport.width_px,
                                viewport.from_bottom_px + viewport.height_px,
                                glow::COLOR_BUFFER_BIT,
                                glow::NEAREST,
                            );

                            painter.gl().bind_framebuffer(glow::READ_FRAMEBUFFER, None);

                            painter.gl().delete_framebuffer(buffer);
                            painter.gl().delete_texture(texture);
                        }
                    })),
                });
            }
        }
    }
}
