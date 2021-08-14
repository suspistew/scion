/// [`WindowDimensions`] is a Resource dedicated to always have an access to the current screen dimension.
/// It's immediatly updated when the window resize event happens.
#[derive(Default, Debug, Copy, Clone)]
pub struct WindowDimensions {
    width: u32,
    height: u32,
}

impl WindowDimensions {
    pub fn new(screen_size: (u32, u32)) -> Self {
        Self { width: screen_size.0, height: screen_size.1 }
    }

    pub fn set(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    pub fn get(self) -> (u32, u32) { (self.width, self.height) }

    pub fn width(self) -> u32 { self.width }

    pub fn height(self) -> u32 { self.height }
}
