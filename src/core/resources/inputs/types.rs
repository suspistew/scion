use serde::{Deserialize, Serialize};
use winit::event::{ElementState, VirtualKeyCode};

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize, Copy)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Other(u16),
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize, Copy)]
pub enum KeyCode {
    Escape,
    Left,
    Up,
    Right,
    Down,
    Any,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Space,
    Tab,
    LShift,
    RShift,
    Apostrophe,
    BackSpace
}

impl KeyCode{
    pub(crate) fn to_char(&self) -> Option<char>{
        match self{
            KeyCode::A => Some('a'),
            KeyCode::B => Some('b'),
            KeyCode::C => Some('c'),
            KeyCode::D => Some('d'),
            KeyCode::E => Some('e'),
            KeyCode::F => Some('f'),
            KeyCode::G => Some('g'),
            KeyCode::H => Some('h'),
            KeyCode::I => Some('i'),
            KeyCode::J => Some('j'),
            KeyCode::K => Some('k'),
            KeyCode::L => Some('l'),
            KeyCode::M => Some('m'),
            KeyCode::N => Some('n'),
            KeyCode::O => Some('o'),
            KeyCode::P => Some('p'),
            KeyCode::Q => Some('q'),
            KeyCode::R => Some('r'),
            KeyCode::S => Some('s'),
            KeyCode::T => Some('t'),
            KeyCode::U => Some('u'),
            KeyCode::V => Some('v'),
            KeyCode::W => Some('w'),
            KeyCode::X => Some('x'),
            KeyCode::Y => Some('y'),
            KeyCode::Z => Some('z'),
            KeyCode::Space => Some(' '),
            KeyCode::Apostrophe => Some('\''),
            _ => None
        }
    }
}

impl From<VirtualKeyCode> for KeyCode {
    fn from(vkc: VirtualKeyCode) -> Self {
        match vkc {
            VirtualKeyCode::Escape => KeyCode::Escape,
            VirtualKeyCode::Left => KeyCode::Left,
            VirtualKeyCode::Up => KeyCode::Up,
            VirtualKeyCode::Right => KeyCode::Right,
            VirtualKeyCode::Down => KeyCode::Down,
            VirtualKeyCode::A => KeyCode::A,
            VirtualKeyCode::B => KeyCode::B,
            VirtualKeyCode::C => KeyCode::C,
            VirtualKeyCode::D => KeyCode::D,
            VirtualKeyCode::E => KeyCode::E,
            VirtualKeyCode::F => KeyCode::F,
            VirtualKeyCode::G => KeyCode::G,
            VirtualKeyCode::H => KeyCode::H,
            VirtualKeyCode::I => KeyCode::I,
            VirtualKeyCode::J => KeyCode::J,
            VirtualKeyCode::K => KeyCode::K,
            VirtualKeyCode::L => KeyCode::L,
            VirtualKeyCode::M => KeyCode::M,
            VirtualKeyCode::N => KeyCode::N,
            VirtualKeyCode::O => KeyCode::O,
            VirtualKeyCode::P => KeyCode::P,
            VirtualKeyCode::Q => KeyCode::Q,
            VirtualKeyCode::R => KeyCode::R,
            VirtualKeyCode::S => KeyCode::S,
            VirtualKeyCode::T => KeyCode::T,
            VirtualKeyCode::U => KeyCode::U,
            VirtualKeyCode::V => KeyCode::V,
            VirtualKeyCode::W => KeyCode::W,
            VirtualKeyCode::X => KeyCode::X,
            VirtualKeyCode::Y => KeyCode::Y,
            VirtualKeyCode::Z => KeyCode::Z,
            VirtualKeyCode::Space => KeyCode::Space,
            VirtualKeyCode::LShift => KeyCode::LShift,
            VirtualKeyCode::RShift => KeyCode::RShift,
            VirtualKeyCode::Tab => KeyCode::Tab,
            VirtualKeyCode::Apostrophe => KeyCode::Apostrophe,
            VirtualKeyCode::Back => KeyCode::BackSpace,
            _ => KeyCode::Any,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize, Copy)]
pub enum InputState {
    Pressed,
    Released,
}

impl From<ElementState> for InputState {
    fn from(state: ElementState) -> Self {
        match state {
            ElementState::Pressed => InputState::Pressed,
            ElementState::Released => InputState::Released,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KeyboardEvent {
    pub keycode: KeyCode,
    pub state: InputState,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub enum Input {
    Key(KeyCode),
    Mouse(MouseButton),
}

impl Into<Input> for KeyCode {
    fn into(self) -> Input {
        Input::Key(self)
    }
}

impl Into<Input> for MouseButton {
    fn into(self) -> Input {
        Input::Mouse(self)
    }
}

pub type Shortcut = Vec<Input>;

#[cfg(test)]
mod tests {

    use crate::core::resources::inputs::inputs_controller::InputsController;

    #[test]
    fn shortcut_test() {
        let controller = InputsController::default();
        let pressed = controller.all_pressed();
        controller.shortcut_pressed(&pressed);
    }
}
