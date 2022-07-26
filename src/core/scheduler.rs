use crate::core::world::GameData;
use std::collections::LinkedList;

#[derive(Default)]
pub(crate) struct Scheduler {
    systems: LinkedList<fn(&mut GameData)>,
}

impl Scheduler {
    pub(crate) fn add_system(&mut self, system: fn(&mut GameData)) {
        self.systems.push_back(system);
    }
    pub(crate) fn execute(&mut self, data: &mut GameData) {
        self.systems.iter().for_each(|s| s(data))
    }
}
