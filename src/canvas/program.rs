use std::rc::Rc;

use eframe::glow::{self, HasContext};

use super::Error;

#[derive(Debug)]
pub struct Program {
    gl: Rc<glow::Context>,
    inner: glow::Program,
    vertices: glow::VertexArray,
}

impl Program {
    pub unsafe fn new(gl: &Rc<glow::Context>) -> Result<Self, Error> {
        let program = gl.create_program().map_err(Error::Gl)?;

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
        )?;
        let frag = Self::shader(
            gl,
            glow::FRAGMENT_SHADER,
            r#"
            #version 330

            void main() {
                gl_FragColor = vec4(1.0, 0.0, 0.0, 1.0);
            }
        "#,
        )?;

        gl.attach_shader(program, vert);
        gl.attach_shader(program, frag);
        gl.link_program(program);
        gl.detach_shader(program, vert);
        gl.detach_shader(program, frag);
        gl.delete_shader(vert);
        gl.delete_shader(frag);

        let vertices = gl.create_vertex_array().map_err(Error::Gl)?;

        Ok(Self {
            gl: gl.clone(),
            inner: program,
            vertices,
        })
    }

    pub unsafe fn draw(&self) {
        self.gl.use_program(Some(self.inner));
        self.gl.bind_vertex_array(Some(self.vertices));
        self.gl.draw_arrays(glow::TRIANGLE_STRIP, 0, 4);
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
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_vertex_array(self.vertices);
            self.gl.delete_program(self.inner);
        }
    }
}
