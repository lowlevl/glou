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

        let vert = Self::shader(
            gl,
            glow::VERTEX_SHADER,
            r#"
            #version 330 core

            const vec2 vertices[4] = vec2[](vec2(-1.0, -1.0), vec2(1.0, -1.0), vec2(-1.0, 1.0), vec2(1.0, 1.0));
            void main() {
                gl_Position = vec4(vertices[gl_VertexID], 0.0, 1.0);
            }
        "#,
        );
        let frag = Self::shader(
            gl,
            glow::FRAGMENT_SHADER,
            r#"
            #version 330

            void main() {
                gl_FragColor = vec4(1.0, 0.0, 0.0, 1.0);
            }
        "#,
        );

        gl.attach_shader(program, vert);
        gl.attach_shader(program, frag);
        gl.link_program(program);
        gl.detach_shader(program, vert);
        gl.detach_shader(program, frag);
        gl.delete_shader(vert);
        gl.delete_shader(frag);

        let vertices = gl
            .create_vertex_array()
            .expect("Cannot create vertex array");

        gl.use_program(Some(program));
        gl.bind_vertex_array(Some(vertices));
        gl.draw_arrays(glow::TRIANGLE_STRIP, 0, 4);
    }

    unsafe fn shader(gl: &glow::Context, ty: u32, source: &str) -> glow::Shader {
        let shader = gl.create_shader(ty).expect("Cannot create shader");

        gl.shader_source(shader, source);
        gl.compile_shader(shader);

        assert!(
            gl.get_shader_compile_status(shader),
            "Failed to compile {ty}: {}",
            gl.get_shader_info_log(shader)
        );

        shader
    }
}
