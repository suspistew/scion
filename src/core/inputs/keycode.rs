use winit::event::VirtualKeyCode;

#[derive(Eq, PartialEq, Hash, Clone)]
pub enum KeyCode {
    Escape,
    Left,
    Up,
    Right,
    Down,
    Any
}

impl From<VirtualKeyCode> for KeyCode {
    fn from(vkc: VirtualKeyCode) -> Self {
        match vkc{
            VirtualKeyCode::Escape => KeyCode::Escape,
            VirtualKeyCode::Left => KeyCode::Left,
            VirtualKeyCode::Up => KeyCode::Up,
            VirtualKeyCode::Right => KeyCode::Right,
            VirtualKeyCode::Down => KeyCode::Down,
            _ => KeyCode::Any
        }
    }
}