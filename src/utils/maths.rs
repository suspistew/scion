/// The standard way to communicate a position in 3 dimensions in `Scion`
#[derive(Eq, PartialEq, Hash, Clone)]
pub struct Position {
    x: usize,
    y: usize,
    z: usize,
}

impl Position {
    pub fn new(x: usize, y: usize, z: usize) -> Self { Self { x, y, z } }

    pub fn x(&self) -> usize { self.x }

    pub fn y(&self) -> usize { self.y }

    pub fn z(&self) -> usize { self.z }
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

    pub fn width(&self) -> usize { self.width }

    pub fn height(&self) -> usize { self.height }

    pub fn depth(&self) -> usize { self.depth }
}
