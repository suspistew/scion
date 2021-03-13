#[repr(C)]
pub(crate) struct GlVec2 {
    pub x: f32,
    pub y: f32,
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
pub(crate) struct GlVertex {
    pub pos: GlVec2,
    pub color: GlColor,
}

#[repr(C)]
pub(crate) struct GlUniform {
    pub offset: (f32, f32),
}