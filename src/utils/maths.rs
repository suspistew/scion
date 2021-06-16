/// The standard way to communicate a position in 3 dimensions in `Scion`
#[derive(Eq, PartialEq, Hash, Clone)]
pub struct Position {
    x: usize,
    y: usize,
    layer: usize,
}

impl Position {
    pub fn new(x: usize, y: usize, layer: usize) -> Self {
        Self { x, y, layer }
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }

    pub fn layer(&self) -> usize {
        self.layer
    }
}

/// The standard way to communicate 3D sizes in `Scion`
pub struct Dimensions {
    width: usize,
    height: usize,
    number_of_layers: usize,
}

impl Dimensions {
    pub fn new(width: usize, height: usize, number_of_layers: usize) -> Self {
        Self {
            width,
            height,
            number_of_layers,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn number_of_layers(&self) -> usize {
        self.number_of_layers
    }
}
