use std::sync::Arc;

use eframe::{
    egui, egui_glow,
    glow::{self, HasContext},
};

pub struct Canvas(Option<glow::Texture>, egui::Painter, egui::Rect);

impl Canvas {
    pub fn new(
        texture: Option<glow::Texture>,
        painter: egui::Painter,
        viewport: egui::Rect,
    ) -> Self {
        Self(texture, painter, viewport)
    }

    pub fn paint(self) {
        if let Some(texture) = self.0 {
            self.1.add(egui::PaintCallback {
                rect: self.2,
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
                            glow::LINEAR,
                        );

                        painter.gl().bind_framebuffer(glow::READ_FRAMEBUFFER, None);
                        painter.gl().delete_framebuffer(buffer);
                    }
                })),
            });
        }
    }
}
