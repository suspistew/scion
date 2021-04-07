//! Everything that is relatives to the core.inputs.
use crate::core::resources::{keyboard::Keyboard, mouse::Mouse};

/// A resource updated by `Scion` to keep track of the core.inputs
/// Can be used in any system.
#[derive(Default)]
pub struct Inputs {
    mouse: Mouse,
    keyboard: Keyboard,
}

impl Inputs {
    pub fn mouse(&self) -> &Mouse {
        &self.mouse
    }

    pub fn mouse_mut(&mut self) -> &mut Mouse {
        &mut self.mouse
    }

    pub fn keyboard(&self) -> &Keyboard {
        &self.keyboard
    }

    pub fn keyboard_mut(&mut self) -> &mut Keyboard {
        &mut self.keyboard
    }
}
