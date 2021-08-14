/// Convenience struct used in all `Scion` to specify any 2D position.
#[derive(Default, Debug, Copy, Clone)]
pub struct Coordinates {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) layer: usize,
}

impl Coordinates {
    pub fn new(x: f32, y: f32) -> Self { Self { x, y, layer: 0 } }

    pub fn new_with_layer(x: f32, y: f32, layer: usize) -> Self { Self { x, y, layer } }

    pub fn x(&self) -> f32 { self.x }

    pub fn y(&self) -> f32 { self.y }

    pub fn layer(&self) -> usize { self.layer }

    pub fn set_x(&mut self, x: f32) { self.x = x }

    pub fn set_y(&mut self, y: f32) { self.y = y; }

    pub fn set_layer(&mut self, layer: usize) { self.layer = layer; }
}
