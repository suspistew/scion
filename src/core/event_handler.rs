use legion::Resources;
use winit::{
    event::WindowEvent
};

use crate::{
    core::{
        legion_ext::ScionResourcesExtension,
        resources::inputs::{
            InputState,
            KeyboardEvent,
            keycode::KeyCode, mouse::{MouseButton, MouseEvent},
        },
    },
};

pub fn update_input_events(window_event: &WindowEvent, resources: &mut Resources) {
    match window_event {
        WindowEvent::KeyboardInput { input, .. } => {
            if let Some(keycode) = input.virtual_keycode {
                let k_event = KeyboardEvent {
                    keycode: KeyCode::from(keycode),
                    state: InputState::from(input.state),
                };
                resources.inputs().keyboard_mut().add_keyboard_event(k_event.clone());
            }
        }
        WindowEvent::MouseInput { state, button, .. } => {
            let m_event =
                MouseEvent { button: MouseButton::from(*button), state: InputState::from(*state) };
            resources.inputs().mouse_mut().set_click_event(Some(m_event.clone()));
        }
        _ => {}
    };
}
