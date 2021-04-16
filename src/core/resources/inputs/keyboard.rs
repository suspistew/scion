use std::collections::HashSet;

use crate::core::resources::inputs::{keycode::KeyCode, InputState, KeyboardEvent};

#[derive(Default)]
pub struct Keyboard {
    pressed_keys: HashSet<KeyCode>,
    keyboard_events: Vec<KeyboardEvent>,
}

impl Keyboard {
    pub fn key_pressed(&self, key: &KeyCode) -> bool {
        self.pressed_keys.contains(key)
    }

    pub fn keyboard_events(&self) -> &Vec<KeyboardEvent> {
        &self.keyboard_events
    }

    pub fn clear_events(&mut self) {
        self.keyboard_events.clear();
    }

    pub fn add_keyboard_event(&mut self, keyboard_event: KeyboardEvent) {
        match &keyboard_event {
            KeyboardEvent { keycode, state } => {
                match state {
                    InputState::Pressed => self.press(keycode),
                    InputState::Released => self.release(keycode),
                }
            }
        }
        self.keyboard_events.push(keyboard_event);
    }

    pub(crate) fn press(&mut self, key: &KeyCode) {
        self.pressed_keys.insert(key.clone());
    }

    pub(crate) fn release(&mut self, key: &KeyCode) {
        self.pressed_keys.remove(&key);
    }
}
