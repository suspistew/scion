use std::collections::LinkedList;
use crate::core::world::World;

#[derive(Default)]
pub(crate) struct Scheduler{
    systems: LinkedList<fn(&mut World)>
}

impl Scheduler{
    pub(crate) fn add_system(&mut self, system: fn(&mut World) ){
        self.systems.push_back(system);
    }
    pub(crate) fn execute(&mut self, world: &mut World){
        self.systems.iter().for_each(|s| s(world))
    }
}