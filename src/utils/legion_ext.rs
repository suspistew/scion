use legion::systems::{Runnable, ResourceTypeId, SystemId, UnsafeResources, ResourceSet, CommandBuffer};
use legion::storage::ComponentTypeId;
use legion::{World, Read};
use legion::world::{ArchetypeAccess, WorldId};
use crate::state::GameState;

pub(crate) struct PausableSystem<S> {
    pub(crate) system: S,
    pub(crate) decider: Box<fn(GameState) -> bool>,
    pub(crate) resource_reads: Vec<ResourceTypeId>,
}

impl<S> Runnable for PausableSystem<S>
    where
        S: Runnable
{
    fn name(&self) -> Option<&SystemId> {
        self.system.name()
    }

    fn reads(&self) -> (&[ResourceTypeId], &[ComponentTypeId]) {
        let (_, components) = self.system.reads();
        (&self.resource_reads[..], components)
    }

    fn writes(&self) -> (&[ResourceTypeId], &[ComponentTypeId]) {
        self.system.writes()
    }

    fn prepare(&mut self, world: &World) {
        self.system.prepare(world)
    }

    fn accesses_archetypes(&self) -> &ArchetypeAccess {
        self.system.accesses_archetypes()
    }

    unsafe fn run_unsafe(&mut self, world: &World, resources: &UnsafeResources) {
        let resources_static = &*(resources as *const UnsafeResources);
        let resource_to_check = Read::<GameState>::fetch_unchecked(resources_static);

        if (self.decider)(*resource_to_check) {
            return;
        } else {}

        self.system.run_unsafe(world, resources);
    }

    fn command_buffer_mut(&mut self, world: WorldId) -> Option<&mut CommandBuffer> {
        self.system.command_buffer_mut(world)
    }
}