use std::{
    io::Read,
    path::{Path, PathBuf},
    rc::Rc,
    time::{SystemTime, UNIX_EPOCH},
};

use eframe::glow::{self, HasContext};

use super::Error;

#[derive(Debug, Clone)]
pub struct Shader {
    path: PathBuf,
    timestamp: SystemTime,
    inner: Option<(glow::Program, glow::VertexArray)>,
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
            timestamp: SystemTime::now(),
            inner: None,
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn load(&mut self, gl: &glow::Context) -> Result<(), Error> {
        if self.inner.is_none() || std::fs::metadata(&self.path)?.modified()? > self.timestamp {
            let source = std::fs::read_to_string(&self.path)?;

            unsafe {
                let program = gl.create_program().map_err(Error::Gl)?;

                let vert = Self::shader(gl, glow::VERTEX_SHADER, Self::VERTEX)?;
                let frag = Self::shader(gl, glow::FRAGMENT_SHADER, &source)?;

                gl.attach_shader(program, vert);
                gl.attach_shader(program, frag);
                gl.link_program(program);
                gl.detach_shader(program, vert);
                gl.detach_shader(program, frag);
                gl.delete_shader(vert);
                gl.delete_shader(frag);

                let vertices = gl.create_vertex_array().map_err(Error::Gl)?;

                if let Some((program, vertices)) = self.inner {
                    tracing::debug!("Freed memory for cached program and vertices");

                    gl.delete_program(program);
                    gl.delete_vertex_array(vertices);
                }

                self.inner = Some((program, vertices));
                self.timestamp = SystemTime::now();

                tracing::info!(
                    "Successfully compiled loaded new shader from `{}`",
                    self.path.display()
                );
            }
        }

        Ok(())
    }

    unsafe fn shader(gl: &glow::Context, ty: u32, source: &str) -> Result<glow::Shader, Error> {
        let shader = gl.create_shader(ty).map_err(Error::Gl)?;

        gl.shader_source(shader, source);
        gl.compile_shader(shader);

        if gl.get_shader_compile_status(shader) {
            Ok(shader)
        } else {
            gl.delete_shader(shader);

            Err(Error::Compile(format!(
                "Failed to compile shader: {}",
                gl.get_shader_info_log(shader)
            )))
        }
    }

    pub unsafe fn draw(&self, gl: &Rc<glow::Context>) {
        if let Some((program, vertices)) = self.inner {
            gl.use_program(Some(program));
            gl.bind_vertex_array(Some(vertices));
            gl.draw_arrays(glow::TRIANGLE_STRIP, 0, 4);
        }
    }
}
