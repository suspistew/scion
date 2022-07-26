use winit::event::WindowEvent;

use crate::core::resources::inputs::{
    mouse::MouseEvent,
    types::{InputState, KeyCode, KeyboardEvent},
};
use crate::core::world::GameData;

pub fn update_input_events(window_event: &WindowEvent, data: &mut GameData) {
    match window_event {
        WindowEvent::KeyboardInput { input, .. } => {
            if let Some(keycode) = input.virtual_keycode {
                let k_event = KeyboardEvent {
                    keycode: KeyCode::from(keycode),
                    state: InputState::from(input.state),
                };
                data.inputs().add_keyboard_event(k_event.clone());
            }
        }
        WindowEvent::MouseInput { state, button, .. } => {
            let m_event = MouseEvent {
                button: crate::core::resources::inputs::types::MouseButton::from(*button),
                state: InputState::from(*state),
            };
            data.inputs().add_click_event(m_event);
        }
        _ => {}
    };
}
