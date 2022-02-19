use serde::{Deserialize, Serialize};

/// The standard way to communicate a position in 3 dimensions in `Scion`
#[derive(Eq, PartialEq, Hash, Clone, Debug, Serialize, Deserialize)]
pub struct Position {
    x: usize,
    y: usize,
    z: usize,
}

impl Position {
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        Self { x, y, z }
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }

    pub fn z(&self) -> usize {
        self.z
    }
}

/// The standard way to communicate 3D sizes in `Scion`
pub struct Dimensions {
    width: usize,
    height: usize,
    depth: usize,
}

impl Dimensions {
    pub fn new(width: usize, height: usize, depth: usize) -> Self {
        Self { width, height, depth }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn depth(&self) -> usize {
        self.depth
    }
}

/// Struct used in all `Scion` to specify any 2D movement.
#[derive(Default, Debug, Copy, Clone)]
pub struct Vector {
    pub(crate) x: f32,
    pub(crate) y: f32,
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
