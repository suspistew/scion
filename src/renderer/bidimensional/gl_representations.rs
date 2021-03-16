use crate::renderer::bidimensional::transform::Position2D;
use ultraviolet::{Mat4, Vec4};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct GlVec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl From<&Position2D> for GlVec3 {
    fn from(position: &Position2D) -> Self {
        Self {
            x: position.x,
            y: position.y,
            z: 0.,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
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
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct ColoredGlVertex {
    pub position: GlVec3,
    pub color: GlColor,
}

impl ColoredGlVertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<ColoredGlVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float3,
                },
            ],
        }
    }
}

#[repr(C)]
pub(crate) struct TexturedGlVertex {
    pub pos: GlVec3,
    pub uv: GlVec3,
}

#[repr(C)]
pub(crate) struct GlUniform {
    pub offset: (f32, f32),
    pub trans: (GlVec4, GlVec4, GlVec4, GlVec4),
    pub scale: (GlVec4, GlVec4, GlVec4, GlVec4),
}

pub(crate) fn create_glmat4(t: &mut Mat4) -> (GlVec4, GlVec4, GlVec4, GlVec4) {
    (
        GlVec4::from(t.cols[0]),
        GlVec4::from(t.cols[1]),
        GlVec4::from(t.cols[2]),
        GlVec4::from(t.cols[3]),
    )
}
