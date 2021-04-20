use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Other(u16),
}

impl From<winit::event::MouseButton> for MouseButton {
    fn from(button: winit::event::MouseButton) -> Self {
        match button {
            winit::event::MouseButton::Left => MouseButton::Left,
            winit::event::MouseButton::Right => MouseButton::Right,
            winit::event::MouseButton::Middle => MouseButton::Middle,
            winit::event::MouseButton::Other(v) => MouseButton::Other(v),
        }
    }
}

use crate::core::resources::inputs::MouseEvent;

/// Contains some data about the mouse, updated at each event received from the window.
/// Can be used in any system.
#[derive(Default, Debug)]
pub struct Mouse {
    x: f64,
    y: f64,
    click_event: Option<MouseEvent>,
}

impl Mouse {
    pub(crate) fn set_position(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }
    pub(crate) fn set_click_event(&mut self, event: Option<MouseEvent>) {
        self.click_event = event;
    }

    /// Returns the current x value of the cursor
    pub fn x(&self) -> f64 {
        self.x
    }
    /// Returns the current y value of the cursor
    pub fn y(&self) -> f64 {
        self.y
    }
    /// Returns if the mouse has been clicked in the current frame
    pub fn click_event(&self) -> &Option<MouseEvent> {
        &self.click_event
    }
}
