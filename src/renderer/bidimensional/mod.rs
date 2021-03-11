pub mod renderer;
pub mod triangle;

#[repr(C)]
pub struct Vec2 {
    x: f32,
    y: f32,
}

#[repr(C)]
pub struct Vec4 {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

#[repr(C)]
pub struct Vertex {
    pos: Vec2,
    color: Vec4,
}

#[repr(C)]
pub struct Uniforms {
    pub offset: (f32, f32),
}