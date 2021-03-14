use ultraviolet::{Mat4, Vec4};
use crate::renderer::bidimensional::transform::Position2D;

#[repr(C)]
pub(crate) struct GlVec2 {
    pub x: f32,
    pub y: f32,
}

impl From<&Position2D> for GlVec2{
    fn from(position: &Position2D) -> Self {
        Self{ x: position.x, y: position.y }
    }
}

#[repr(C)]
#[derive(Clone)]
pub(crate) struct GlColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

#[repr(C)]
#[derive(Clone)]
pub(crate) struct GlVec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl From<Vec4> for GlVec4 {
    fn from(vec: Vec4) -> Self {
        GlVec4 {
            x: vec.x,
            y: vec.y,
            z: vec.z,
            w: vec.w,
        }
    }
}

#[repr(C)]
pub(crate) struct GlVertex {
    pub pos: GlVec2,
    pub color: GlColor,
}

#[repr(C)]
pub(crate) struct GlUniform {
    pub offset: (f32, f32),
    pub trans: (GlVec4, GlVec4, GlVec4, GlVec4),
    pub scale: (GlVec4, GlVec4, GlVec4, GlVec4),
}

pub(crate) fn create_glmat4(t: &mut Mat4) -> (GlVec4, GlVec4, GlVec4, GlVec4) {
    (GlVec4::from(t.cols[0]), GlVec4::from(t.cols[1]), GlVec4::from(t.cols[2]), GlVec4::from(t.cols[3]))
}