use serde::{Deserialize, Serialize};

use crate::core::resources::inputs::InputState;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MouseEvent {
    pub button: MouseButton,
    pub state: InputState,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
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

    /// Execute the action `action` if the left mouse button is clicked, actions params are mouse position x;y
    pub fn on_left_click_pressed<Body>(&self, action: Body)
    where
        Body: FnMut(f64, f64), {
        self.on_mouse_event(MouseButton::Left, InputState::Pressed, action);
    }

    /// Execute the action `action` if the right mouse button is clicked, actions params are mouse position x;y
    pub fn on_right_click_pressed<Body>(&self, action: Body)
    where
        Body: FnMut(f64, f64), {
        self.on_mouse_event(MouseButton::Right, InputState::Pressed, action);
    }

    /// Execute the action `action` if the middle mouse button is clicked, actions params are mouse position x;y
    pub fn on_middle_click_pressed<Body>(&self, action: Body)
    where
        Body: FnMut(f64, f64), {
        self.on_mouse_event(MouseButton::Middle, InputState::Pressed, action);
    }

    /// Execute the action `action` if the left mouse button is released, actions params are mouse position x;y
    pub fn on_left_click_released<Body>(&self, action: Body)
    where
        Body: FnMut(f64, f64), {
        self.on_mouse_event(MouseButton::Left, InputState::Released, action);
    }

    /// Execute the action `action` if the right mouse button is released, actions params are mouse position x;y
    pub fn on_right_click_released<Body>(&self, action: Body)
    where
        Body: FnMut(f64, f64), {
        self.on_mouse_event(MouseButton::Right, InputState::Pressed, action);
    }

    /// Execute the action `action` if the right mouse button is released, actions params are mouse position x;y
    pub fn on_middle_click_released<Body>(&self, action: Body)
    where
        Body: FnMut(f64, f64), {
        self.on_mouse_event(MouseButton::Middle, InputState::Pressed, action);
    }

    fn on_mouse_event<Body>(
        &self,
        target_button: MouseButton,
        target_state: InputState,
        mut action: Body,
    ) where
        Body: FnMut(f64, f64), {
        if let Some(ref event) = self.click_event {
            if event.button == target_button && event.state == target_state {
                action(self.x, self.y)
            }
        }
    }

    /// Returns the current x value of the cursor
    pub fn x(&self) -> f64 { self.x }
    /// Returns the current y value of the cursor
    pub fn y(&self) -> f64 { self.y }
    /// Returns if the mouse has been clicked in the current frame
    pub fn click_event(&self) -> &Option<MouseEvent> { &self.click_event }
}
