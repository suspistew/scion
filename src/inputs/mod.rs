use crate::rendering::bidimensional::transform::Position2D;
use crate::inputs::mouse::Mouse;

pub mod mouse;

#[derive(Default)]
pub struct Inputs {
    mouse: Mouse
}

impl Inputs{
    pub fn mouse(&self) -> &Mouse {
        &self.mouse
    }

    pub fn mouse_mut(&mut self) -> &mut Mouse {
        &mut self.mouse
    }
}