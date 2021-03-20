pub struct Camera2D {
    pub(crate) left: f32,
    pub(crate) right: f32,
    pub(crate) top: f32,
    pub(crate) bottom: f32,
    pub(crate) near: f32,
    pub(crate) far: f32,
}

impl Camera2D {
    pub fn new(width: f32, height: f32, depth: f32) -> Self {
        Self {
            left: 0.,
            right: width,
            top: 0.,
            bottom: -1. * height,
            near: 0.0,
            far: depth,
        }
    }
}
