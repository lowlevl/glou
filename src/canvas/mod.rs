use std::{collections::HashMap, rc::Rc, sync::Arc, time};

use eframe::{
    egui, egui_glow,
    glow::{self, HasContext},
};

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
    pub fn tick(&mut self, ctx: &egui::Context, gl: &Rc<glow::Context>) {
        egui::CentralPanel::default()
            .frame(egui::Frame::canvas(&ctx.style()))
            .show(ctx, |ui| {
                self.paint(ui, gl);
            });
    }

    fn paint(&mut self, ui: &mut egui::Ui, gl: &Rc<glow::Context>) {
        let (response, painter) = ui.allocate_painter(ui.available_size(), egui::Sense::hover());
        let rect = painter.clip_rect();

        // Setup uniforms from various parameters
        self.uniforms.insert(
            "u_resolution",
            vec![
                painter.round_to_pixel(rect.width()),
                painter.round_to_pixel(rect.height()),
            ],
        );
        if let Some(pos) = response.hover_pos() {
            self.uniforms.insert(
                "u_mouse",
                vec![
                    painter.round_to_pixel(pos.x - rect.left()),
                    painter.round_to_pixel(rect.bottom() - pos.y),
                ],
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
            // Draw shader to right-sized texture
            let texture = unsafe {
                shader
                    .render_to_texture(
                        gl,
                        &self.uniforms,
                        egui::vec2(
                            painter.round_to_pixel(rect.width()).max(1.0),
                            painter.round_to_pixel(rect.height()).max(1.0),
                        ),
                    )
                    .expect("Unable to render shader")
            };

            // Finally paint the texture on the screen
            painter.add(egui::PaintCallback {
                rect,
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
