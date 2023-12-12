use std::{
    path::{Path, PathBuf},
    rc::Rc,
    time::{self, SystemTime},
};

use eframe::{
    egui,
    glow::{self, HasContext},
};

use super::Uniforms;
use crate::{guard, AllocGuard, Error};

#[derive(Debug)]
pub struct Shader {
    path: PathBuf,
    rebuilt_at: f64,
    inner: Option<(AllocGuard<glow::Program>, AllocGuard<glow::VertexArray>)>,
}

impl Shader {
    const VERTEX: &'static str = r#"
        #version 330 core

        const vec2 vertices[4] = vec2[](vec2(-1.0, -1.0), vec2(1.0, -1.0), vec2(-1.0, 1.0), vec2(1.0, 1.0));

        void main() {
            gl_Position = vec4(vertices[gl_VertexID], 0.0, 1.0);
        }
    "#;

    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            rebuilt_at: 0f64,
            inner: None,
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn rebuild(&mut self, gl: &Rc<glow::Context>) -> Result<bool, Error> {
        if std::fs::metadata(&self.path)?
            .modified()?
            .duration_since(time::UNIX_EPOCH)
            .expect("Time went backwards >.>")
            .as_secs_f64()
            > self.rebuilt_at
        {
            tracing::info!(
                "Source file at `{}` was updated, compiling shader..",
                self.path.display()
            );

            self.rebuilt_at = SystemTime::now()
                .duration_since(time::UNIX_EPOCH)
                .expect("Time went backwards >.>")
                .as_secs_f64();

            let source = std::fs::read_to_string(&self.path)?;

            unsafe {
                let program = guard!(
                    gl,
                    gl.create_program().map_err(Error::Gl)?,
                    move |program| gl.delete_program(program)
                );

                let vert = Self::shader(gl, glow::VERTEX_SHADER, Self::VERTEX)?;
                let frag = Self::shader(gl, glow::FRAGMENT_SHADER, &source)?;

                gl.attach_shader(*program, *vert);
                gl.attach_shader(*program, *frag);
                gl.link_program(*program);
                gl.detach_shader(*program, *vert);
                gl.detach_shader(*program, *frag);

                let vertices = guard!(
                    gl,
                    gl.create_vertex_array().map_err(Error::Gl)?,
                    move |vertices| gl.delete_vertex_array(vertices)
                );

                self.inner = Some((program, vertices));

                tracing::info!(
                    "Successfully compiled loaded new shader from `{}`",
                    self.path.display()
                );
            }

            Ok(true)
        } else {
            Ok(false)
        }
    }

    unsafe fn shader(
        gl: &Rc<glow::Context>,
        ty: u32,
        source: &str,
    ) -> Result<AllocGuard<glow::Shader>, Error> {
        let shader = guard!(
            gl,
            gl.create_shader(ty).map_err(Error::Gl)?,
            move |shader| gl.delete_shader(shader)
        );

        gl.shader_source(*shader, source);
        gl.compile_shader(*shader);

        if gl.get_shader_compile_status(*shader) {
            Ok(shader)
        } else {
            Err(Error::Compile(format!(
                "Failed to compile shader:\n{}",
                gl.get_shader_info_log(*shader),
            )))
        }
    }

    unsafe fn render(&self, gl: &Rc<glow::Context>, uniforms: &Uniforms) {
        if let Some((program, vertices)) = &self.inner {
            gl.use_program(Some(**program));

            uniforms.apply(gl, **program);

            gl.bind_vertex_array(Some(**vertices));
            gl.draw_arrays(glow::TRIANGLE_STRIP, 0, 4);
            gl.bind_vertex_array(None);
        }
    }

    pub unsafe fn render_to_texture(
        &self,
        gl: &Rc<glow::Context>,
        uniforms: &Uniforms,
        size: egui::Vec2,
    ) -> Result<AllocGuard<glow::Texture>, Error> {
        let texture = guard!(
            gl,
            gl.create_texture().map_err(Error::Gl)?,
            move |texture| gl.delete_texture(texture)
        );

        gl.bind_texture(glow::TEXTURE_2D, Some(*texture));
        gl.tex_image_2d(
            glow::TEXTURE_2D,
            0,
            glow::RGBA as i32,
            size.x as i32,
            size.y as i32,
            0,
            glow::RGBA,
            glow::UNSIGNED_BYTE,
            None,
        );

        let buffer = guard!(
            gl,
            gl.create_framebuffer().map_err(Error::Gl)?,
            move |buffer| gl.delete_framebuffer(buffer)
        );

        gl.bind_framebuffer(glow::FRAMEBUFFER, Some(*buffer));
        gl.framebuffer_texture_2d(
            glow::FRAMEBUFFER,
            glow::COLOR_ATTACHMENT0,
            glow::TEXTURE_2D,
            Some(*texture),
            0,
        );
        gl.draw_buffer(glow::COLOR_ATTACHMENT0);

        assert!(gl.check_framebuffer_status(glow::FRAMEBUFFER) == glow::FRAMEBUFFER_COMPLETE);

        gl.viewport(0, 0, size.x as i32, size.y as i32);

        self.render(gl, uniforms);

        gl.bind_framebuffer(glow::FRAMEBUFFER, None);
        gl.bind_texture(glow::TEXTURE_2D, None);

        Ok(texture)
    }
}
