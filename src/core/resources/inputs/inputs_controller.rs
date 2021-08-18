//! Everything that is relatives to the core.resources.inputs.
use crate::core::resources::inputs::{keyboard::Keyboard, mouse::Mouse};

/// A resource updated by `Scion` to keep track of the core.resources.inputs
/// Can be used in any system.
#[derive(Default)]
pub struct InputsController {
    mouse: Mouse,
    keyboard: Keyboard,
}

impl InputsController {
    /// retrieve the mouse inputs
    pub fn mouse(&self) -> &Mouse { &self.mouse }

    /// retrieve the mouse inputs, mutable.
    pub fn mouse_mut(&mut self) -> &mut Mouse { &mut self.mouse }

    /// retrieve the keyboard inputs
    pub fn keyboard(&self) -> &Keyboard { &self.keyboard }

    /// retrieve the keyboard inputs, mutable
    pub fn keyboard_mut(&mut self) -> &mut Keyboard { &mut self.keyboard }

    pub(crate) fn reset_inputs(&mut self) {
        self.mouse.set_click_event(None);
        self.keyboard.clear_events();
    }
}
