use crate::core::components::maths::transform::Coordinates;

/// Mandatory resource to add to the Resources to have anything rendered.
pub struct Camera {
    pub(crate) left: f32,
    pub(crate) right: f32,
    pub(crate) top: f32,
    pub(crate) bottom: f32,
    pub(crate) near: f32,
    pub(crate) far: f32,
    pub(crate) position: Coordinates,
}

impl Camera {
    /// Creates a camera with a viewport of size (width;height;depth). In general the same width and height of the window.
    pub fn new(width: f32, height: f32, depth: f32) -> Self {
        Self {
            left: 0.,
            right: width,
            top: 0.,
            bottom: -1. * height,
            near: 0.0,
            far: depth,
            position: Coordinates::default(),
        }
    }

    /// Creates a camera with a viewport of size (width;height;depth). In general the same width and height of the window.
    pub fn new_with_position(width: f32, height: f32, depth: f32, position: Coordinates) -> Self {
        Self {
            left: 0.,
            right: width,
            top: 0.,
            bottom: -1. * height,
            near: 0.0,
            far: depth,
            position,
        }
    }

    pub fn position(&self) -> &Coordinates {
        &self.position
    }

    pub fn set_position(&mut self, new_position: Coordinates) {
        self.position = new_position;
    }

    pub fn append_position(&mut self, x: f32, y: f32) {
        self.position.set_x(self.position.x() + x);
        self.position.set_y(self.position.y() + y);
    }
}
