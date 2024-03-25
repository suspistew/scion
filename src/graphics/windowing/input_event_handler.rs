use log::{debug, info};
use winit::event::WindowEvent;
use winit::keyboard::Key;

use crate::core::resources::inputs::{
    mouse::MouseEvent,
    types::{InputState, KeyCode, KeyboardEvent},
};
use crate::core::world::GameData;

pub fn update_input_events(window_event: WindowEvent, data: &mut GameData) {
    match window_event {
        WindowEvent::KeyboardInput { event,.. } => {
            if let &Key::Dead(_) = &event.logical_key {
            }else{
                let k_event = KeyboardEvent {
                    keycode: KeyCode::from(&event.logical_key),
                    state: InputState::from(event.state),
                };
                data.inputs().add_keyboard_event(k_event);
            }
        }
        WindowEvent::MouseInput { state, button, .. } => {
            let m_event = MouseEvent {
                button: crate::core::resources::inputs::types::MouseButton::from(button),
                state: InputState::from(state),
            };
            data.inputs().add_click_event(m_event);
        }
        _ => {}
    };
}
