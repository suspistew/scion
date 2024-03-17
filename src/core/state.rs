use std::collections::HashMap;

/// `GameState` is a convenience Resource created to keep track of
/// diverse thing internally. It's also the resource used to create
/// pausable systems.
#[derive(Debug, Default)]
pub struct GameState {
    registry: HashMap<String, bool>
}

impl GameState {
    pub fn get(&self, key: &str) -> bool {
        if self.registry.contains_key(key){
            return *self.registry.get(key).unwrap()
        } else{
            false
        }
    }

    pub fn set(&mut self, key: &str, val: bool){
        self.registry.insert(key.to_string(), val);
    }
}
