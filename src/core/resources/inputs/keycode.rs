use serde::{Deserialize, Serialize};
use winit::event::VirtualKeyCode;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize)]
pub enum KeyCode {
    Escape,
    Left,
    Up,
    Right,
    Down,
    Any,
}

impl From<VirtualKeyCode> for KeyCode {
    fn from(vkc: VirtualKeyCode) -> Self {
        match vkc {
            VirtualKeyCode::Escape => KeyCode::Escape,
            VirtualKeyCode::Left => KeyCode::Left,
            VirtualKeyCode::Up => KeyCode::Up,
            VirtualKeyCode::Right => KeyCode::Right,
            VirtualKeyCode::Down => KeyCode::Down,
            _ => KeyCode::Any,
        }
    }
}
