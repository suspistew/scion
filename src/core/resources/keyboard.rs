use std::collections::HashSet;

use crate::core::inputs::keycode::KeyCode;

#[derive(Default)]
pub struct Keyboard {
    pressed_keys: HashSet<KeyCode>,
    event_keys: HashSet<KeyCode>,
}

impl Keyboard {
    pub fn key_pressed(&self, key: &KeyCode) -> bool {
        self.pressed_keys.contains(key)
    }

    pub fn key_event(&self, key: &KeyCode) -> bool {
        self.event_keys.contains(key)
    }

    pub(crate) fn clear_events(&mut self) {
        self.event_keys.clear()
    }

    pub(crate) fn press(&mut self, key: KeyCode) {
        self.pressed_keys.insert(key.clone());
        self.event_keys.insert(key);
    }

    pub(crate) fn release(&mut self, key: KeyCode) {
        self.pressed_keys.remove(&key);
    }
}
