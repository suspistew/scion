/// Convenience struct used in all `Scion` to specify any 2D position.
#[derive(Default, Debug, Copy, Clone)]
pub struct Coordinates {
    x: f32,
    y: f32,
    layer: usize,
}

impl Coordinates {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y, layer: 0 }
    }

    pub fn new_with_layer(x: f32, y: f32, layer: usize) -> Self {
        Self { x, y, layer }
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn layer(&self) -> usize {
        self.layer
    }

    pub fn set_x(&mut self, x: f32) {
        self.x = x
    }

    pub fn set_y(&mut self, y: f32) {
        self.y = y;
    }

    pub fn set_layer(&mut self, layer: usize) {
        self.layer = layer;
    }
}

/// Component used by the renderer to know where and how to represent an object.
/// Default is position 0;0 with a scale of 1.0 and no angle.
#[derive(Debug)]
pub struct Transform {
    pub(crate) translation: Coordinates,
    pub(crate) scale: f32,
    pub(crate) angle: f32,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            coords: Default::default(),
            scale: 1.0,
            angle: 0.0,
        }
    }
}

impl Transform {
    /// Creates a new transform using provided values.
    pub fn new(coords: Coordinates, scale: f32, angle: f32) -> Self {
        Self {
            coords,
            scale,
            angle,
        }
    }

    /// Append a translation to this transform's position
    pub fn append_translation(&mut self, x: f32, y: f32) {
        self.coords.x += x;
        self.coords.y += y;
    }

    /// Move this transform down
    pub fn move_down(&mut self, y: f32) {
        self.coords.y += y;
    }

    /// Append an angle to this transform's angle
    pub fn append_angle(&mut self, angle: f32) {
        self.angle += angle;
    }

    /// Get the transform's coordinates
    pub fn translation(&self) -> &Coordinates {
        &self.coords
    }

    /// Change the scale value to a new one.
    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale
    }

    /// Change the layer value in the coordinates.
    pub fn set_layer(&mut self, layer: usize) {
        self.coords.layer = layer
    }
}
