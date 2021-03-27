//! Everything that is relatives to the core.inputs.
use crate::core::resources::mouse::Mouse;

/// A resource updated by `Scion` to keep track of the core.inputs
/// Can be used in any system.
#[derive(Default)]
pub struct Inputs {
    mouse: Mouse,
}

impl Inputs {
    pub fn mouse(&self) -> &Mouse {
        &self.mouse
    }

    pub fn mouse_mut(&mut self) -> &mut Mouse {
        &mut self.mouse
    }
}
