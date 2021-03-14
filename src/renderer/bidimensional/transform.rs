#[derive(Default, Debug, Copy, Clone)]
pub struct Position2D {
    pub x: f32,
    pub y: f32
}

#[derive(Debug)]
pub struct Transform2D {
    pub(crate) position: Position2D,
    pub(crate) scale: f32,
    pub(crate) angle: f32
}

impl Default for Transform2D{
    fn default() -> Self {
        Self {
            position: Default::default(),
            scale: 1.0,
            angle: 0.0
        }
    }
}

impl Transform2D {

    pub fn new(position: Position2D, scale: f32, angle: f32) -> Self{
        Self {
            position,
            scale,
            angle
        }
    }

    pub fn append_translation(&mut self, x: f32, y: f32) {
        self.position.x += x;
        self.position.y += y;
    }

    pub fn append_angle(&mut self, angle: f32) {
        self.angle += angle;
    }
}


