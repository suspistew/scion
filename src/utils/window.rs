/// `WindowDimensions` is a Resource dedicated to always have an access to the current screen dimension.
/// It's immediatly updated when the window resize event happens.
#[derive(Default, Debug)]
pub struct WindowDimensions {
    width: f32,
    height: f32,
}

impl WindowDimensions {
    pub fn new(screen_size: (f32, f32)) -> Self {
        Self {
            width: screen_size.0,
            height: screen_size.1,
        }
    }

    pub fn set(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
    }

    pub fn get(self) -> (f32, f32) {
        (self.width, self.height)
    }

    pub fn width(self) -> f32 {
        self.width
    }

    pub fn height(self) -> f32 {
        self.height
    }
}
