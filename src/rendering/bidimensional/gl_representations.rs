use crate::rendering::bidimensional::components::camera::Camera2D;
use crate::rendering::bidimensional::transform::{Position2D, Transform2D};
use ultraviolet::{Mat4, Rotor3, Similarity3, Vec3, Vec4};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct GlVec2 {
    pub x: f32,
    pub y: f32,
}

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
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
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
    pub fn _desc<'a>() -> wgpu::VertexBufferLayout<'a> {
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
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct TexturedGlVertex {
    pub position: GlVec3,
    pub tex_coords: GlVec2,
}

impl TexturedGlVertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<TexturedGlVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float2,
                },
            ],
        }
    }
}

impl From<(&Position2D, &Position2D)> for TexturedGlVertex {
    fn from(positions: (&Position2D, &Position2D)) -> Self {
        TexturedGlVertex {
            position: GlVec3 {
                x: positions.0.x,
                y: positions.0.y,
                z: 0.0,
            },
            tex_coords: GlVec2 {
                x: positions.1.x,
                y: positions.1.y,
            },
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct GlUniform {
    pub model_trans: [[f32; 4]; 4],
    pub camera_view: [[f32; 4]; 4],
}

impl GlUniform {
    pub(crate) fn replace_with(&mut self, other: GlUniform) {
        self.model_trans = other.model_trans;
        self.camera_view = other.camera_view;
    }
}

pub(crate) fn create_glmat4(t: &mut Mat4) -> [[f32; 4]; 4] {
    let result = [
        create_glmat(&t.cols[0]),
        create_glmat(&t.cols[1]),
        create_glmat(&t.cols[2]),
        create_glmat(&t.cols[3]),
    ];
    result
}

pub(crate) fn create_glmat(t: &Vec4) -> [f32; 4] {
    [t.x, t.y, t.z, t.w]
}

impl From<(&Transform2D, &Camera2D)> for GlUniform {
    fn from((transform, camera): (&Transform2D, &Camera2D)) -> Self {
        let mut model_trans = Similarity3::identity();
        model_trans.prepend_scaling(transform.scale);
        model_trans.append_translation(Vec3 {
            x: transform.position.x,
            y: transform.position.y,
            z: 1.0,
        });
        model_trans.prepend_rotation(Rotor3::from_rotation_xy(transform.angle).normalized());

        let mut test = Similarity3::identity();
        test.append_translation(Vec3::new(-384., 384., 0.));

        let mut model_trans = model_trans.into_homogeneous_matrix();
        let mut camera_view = ultraviolet::projection::lh_ydown::orthographic_wgpu_dx(
            camera.left,
            camera.right,
            camera.bottom,
            camera.top,
            camera.near,
            camera.far,
        );
        GlUniform {
            model_trans: create_glmat4(&mut model_trans),
            camera_view: create_glmat4(&mut camera_view),
        }
    }
}
