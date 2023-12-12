use std::{rc::Rc, time};

use eframe::{
    egui,
    glow::{self, HasContext},
};
use strum::{AsRefStr, EnumIter};

#[derive(Debug, Default, PartialEq, Clone, Copy, EnumIter, AsRefStr)]
pub enum UniformStyle {
    #[default]
    #[strum(serialize = "Classic (u_<name>)")]
    Classic,

    #[strum(serialize = "Shader Toy (i<Name>)")]
    ShaderToy,

    #[strum(serialize = "GLSL Sandbox (<name>)")]
    GlslSandbox,
}

impl UniformStyle {
    pub fn format(&self, name: &str) -> String {
        let mut chars = name.chars();

        match self {
            Self::Classic => format!("u_{}", chars.as_str()),
            Self::ShaderToy => format!(
                "i{}{}",
                chars.next().unwrap_or('?').to_ascii_uppercase(),
                chars.as_str()
            ),
            Self::GlslSandbox => name.to_string(),
        }
    }
}

#[derive(Debug, Default)]
pub struct Uniforms {
    pub style: UniformStyle,
    epoch: Option<time::Instant>,

    time: f32,
    mouse: egui::Vec2,
    resolution: egui::Vec2,
}

impl Uniforms {
    pub fn update(&mut self, viewport: egui::Rect, mouse: Option<egui::Pos2>) {
        self.time = self
            .epoch
            .get_or_insert_with(time::Instant::now)
            .elapsed()
            .as_secs_f32();

        if let Some(mouse) = mouse {
            self.mouse = egui::vec2(mouse.x - viewport.left(), viewport.bottom() - mouse.y);
        } else {
            self.mouse = viewport.center() - viewport.min;
        }

        self.resolution = viewport.size();
    }

    pub fn to_iter(&self) -> impl Iterator<Item = (String, Vec<f32>)> {
        [
            (self.style.format("time"), vec![self.time]),
            (self.style.format("mouse"), vec![self.mouse.x, self.mouse.y]),
            (
                self.style.format("resolution"),
                vec![self.resolution.x, self.resolution.y],
            ),
        ]
        .into_iter()
    }

    pub fn reset_time(&mut self) {
        self.epoch = None;
    }

    pub unsafe fn apply(&self, gl: &Rc<glow::Context>, program: glow::Program) {
        for (name, value) in self.to_iter() {
            let location = gl.get_uniform_location(program, &name);

            match value.as_slice() {
                [x] => gl.uniform_1_f32(location.as_ref(), *x),
                [x, y] => gl.uniform_2_f32(location.as_ref(), *x, *y),
                [x, y, z] => gl.uniform_3_f32(location.as_ref(), *x, *y, *z),
                [x, y, z, w] => gl.uniform_4_f32(location.as_ref(), *x, *y, *z, *w),
                _ => panic!("Mis-sized uniform value"),
            }
        }
    }
}
