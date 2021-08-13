
/// Struct used in all `Scion` to specify any 2D movement.
#[derive(Default, Debug, Copy, Clone)]
pub struct Vector {
    pub(crate) x: f32,
    pub(crate) y: f32
}

impl Vector {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn set_x(&mut self, x: f32) {
        self.x = x
    }

    pub fn set_y(&mut self, y: f32) {
        self.y = y;
    }
}