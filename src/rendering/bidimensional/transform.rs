/// Convenience struct used in all `Scion` to specify any 2D position.
#[derive(Default, Debug, Copy, Clone)]
pub struct Position2D {
    pub x: f32,
    pub y: f32,
}

/// Component used by the renderer to know where and how to represent an object.
/// Default is position 0;0 with a scale of 1.0 and no angle.
#[derive(Debug)]
pub struct Transform2D {
    pub(crate) position: Position2D,
    pub(crate) scale: f32,
    pub(crate) angle: f32,
}

impl Default for Transform2D {
    fn default() -> Self {
        Self {
            position: Default::default(),
            scale: 1.0,
            angle: 0.0,
        }
    }
}

impl Transform2D {
    /// Creates a new transform using provided values.
    pub fn new(position: Position2D, scale: f32, angle: f32) -> Self {
        Self {
            position,
            scale,
            angle,
        }
    }

    /// Append a translation to this transform's position
    pub fn append_translation(&mut self, x: f32, y: f32) {
        self.position.x += x;
        self.position.y += y;
    }

    /// Append an angle to this transform's angle
    pub fn append_angle(&mut self, angle: f32) {
        self.angle += angle;
    }

    /// Get the transform's position
    pub fn position(&self) -> &Position2D {
        &self.position
    }

    /// Change the scale value to a new one.
    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale
    }
}
