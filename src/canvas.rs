use std::sync::Arc;

use eframe::{
    egui, egui_glow,
    glow::{self, HasContext},
};

use crate::{guard, AllocGuard};

pub struct Canvas(Option<AllocGuard<glow::Texture>>, egui::Painter);

impl Canvas {
    pub fn new(texture: Option<AllocGuard<glow::Texture>>, painter: egui::Painter) -> Self {
        Self(texture, painter)
    }

    pub fn paint(mut self) {
        if let Some(texture) = self.0.take() {
            // This removes the guard, but we'll re-add it after thread boudary
            let texture = AllocGuard::into_inner(texture);

            self.1.add(egui::PaintCallback {
                rect: self.1.clip_rect(),
                callback: Arc::new(egui_glow::CallbackFn::new({
                    move |info, painter| unsafe {
                        let gl = painter.gl();

                        // Readded allocation guard to ensure the resource is deleted after use
                        let texture =
                            guard!(gl, texture, move |texture| gl.delete_texture(texture));

                        let buffer = guard!(
                            gl,
                            gl.create_framebuffer()
                                .expect("Unable to create frame buffer"),
                            move |buffer| gl.delete_framebuffer(buffer)
                        );

                        painter
                            .gl()
                            .bind_framebuffer(glow::READ_FRAMEBUFFER, Some(*buffer));
                        painter.gl().framebuffer_texture_2d(
                            glow::READ_FRAMEBUFFER,
                            glow::COLOR_ATTACHMENT0,
                            glow::TEXTURE_2D,
                            Some(*texture),
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
                    }
                })),
            });
        }
    }
}
