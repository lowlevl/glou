use core::panic;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    rc::Rc,
    time::{self, SystemTime},
};

use eframe::glow::{self, HasContext};

use super::Error;

#[derive(Debug, Clone)]
pub struct Shader {
    path: PathBuf,
    timestamp: f64,
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
            timestamp: 0f64,
            inner: None,
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn load(&mut self, gl: &glow::Context) -> Result<bool, Error> {
        if std::fs::metadata(&self.path)?
            .modified()?
            .duration_since(time::UNIX_EPOCH)
            .expect("Time went backwards >.>")
            .as_secs_f64()
            > self.timestamp
        {
            self.timestamp = SystemTime::now()
                .duration_since(time::UNIX_EPOCH)
                .expect("Time went backwards >.>")
                .as_secs_f64();

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

    unsafe fn shader(gl: &glow::Context, ty: u32, source: &str) -> Result<glow::Shader, Error> {
        let shader = gl.create_shader(ty).map_err(Error::Gl)?;

        gl.shader_source(shader, source);
        gl.compile_shader(shader);

        if gl.get_shader_compile_status(shader) {
            Ok(shader)
        } else {
            let err = Error::Compile(format!(
                "Failed to compile shader:\n{}",
                gl.get_shader_info_log(shader),
            ));

            gl.delete_shader(shader);

            Err(err)
        }
    }

    pub unsafe fn render(
        &self,
        gl: &Rc<glow::Context>,
        uniforms: &HashMap<&'static str, Vec<f32>>,
    ) {
        if let Some((program, vertices)) = self.inner {
            gl.use_program(Some(program));

            for (name, value) in uniforms {
                let location = gl.get_uniform_location(program, name);

                match value.as_slice() {
                    [x] => gl.uniform_1_f32(location.as_ref(), *x),
                    [x, y] => gl.uniform_2_f32(location.as_ref(), *x, *y),
                    [x, y, z] => gl.uniform_3_f32(location.as_ref(), *x, *y, *z),
                    [x, y, z, w] => gl.uniform_4_f32(location.as_ref(), *x, *y, *z, *w),
                    _ => panic!("Misconfigured uniform"),
                }
            }

            gl.bind_vertex_array(Some(vertices));
            gl.draw_arrays(glow::TRIANGLE_STRIP, 0, 4);
            gl.bind_vertex_array(None);
        }
    }
}
