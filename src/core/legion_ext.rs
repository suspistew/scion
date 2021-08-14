use atomic_refcell::AtomicRefMut;
use legion::{
    storage::ComponentTypeId,
    systems::{CommandBuffer, ResourceSet, ResourceTypeId, Runnable, SystemId, UnsafeResources},
    world::{ArchetypeAccess, WorldId},
    Read, Resources, World,
};

use crate::{
    core::{
        components::maths::camera::DefaultCamera,
        resources::{
            asset_manager::AssetManager, events::Events,
            inputs::inputs_controller::InputsController, time::Timers,
        },
        state::GameState,
    },
    legion::Entity,
};

pub(crate) struct PausableSystem<S> {
    pub(crate) system: S,
    pub(crate) decider: Box<fn(GameState) -> bool>,
    pub(crate) resource_reads: Vec<ResourceTypeId>,
}

impl<S> Runnable for PausableSystem<S>
where
    S: Runnable,
{
    fn name(&self) -> Option<&SystemId> { self.system.name() }

    fn reads(&self) -> (&[ResourceTypeId], &[ComponentTypeId]) {
        let (_, components) = self.system.reads();
        (&self.resource_reads[..], components)
    }

    fn writes(&self) -> (&[ResourceTypeId], &[ComponentTypeId]) { self.system.writes() }

    fn prepare(&mut self, world: &World) { self.system.prepare(world) }

    fn accesses_archetypes(&self) -> &ArchetypeAccess { self.system.accesses_archetypes() }

    unsafe fn run_unsafe(&mut self, world: &World, resources: &UnsafeResources) {
        let resources_static = &*(resources as *const UnsafeResources);
        let resource_to_check = Read::<GameState>::fetch_unchecked(resources_static);

        if (self.decider)(*resource_to_check) {
            return;
        } else {
        }

        self.system.run_unsafe(world, resources);
    }

    fn command_buffer_mut(&mut self, world: WorldId) -> Option<&mut CommandBuffer> {
        self.system.command_buffer_mut(world)
    }
}

pub trait ScionResourcesExtension {
    fn assets(&mut self) -> AtomicRefMut<AssetManager>;
    fn timers(&mut self) -> AtomicRefMut<Timers>;
    fn inputs(&mut self) -> AtomicRefMut<InputsController>;
    fn events(&mut self) -> AtomicRefMut<Events>;
}

impl ScionResourcesExtension for Resources {
    fn assets(&mut self) -> AtomicRefMut<AssetManager> {
        self.get_mut::<AssetManager>()
            .expect("The engine is missing the mandatory asset manager resource")
    }

    fn timers(&mut self) -> AtomicRefMut<Timers> {
        self.get_mut::<Timers>().expect("The engine is missing the mandatory timers resource")
    }

    fn inputs(&mut self) -> AtomicRefMut<InputsController> {
        self.get_mut::<InputsController>()
            .expect("The engine is missing the mandatory inputs controller resource")
    }

    fn events(&mut self) -> AtomicRefMut<Events> {
        self.get_mut::<Events>().expect("The engine is missing the mandatory events resource")
    }
}

pub trait ScionWorldExtension {
    fn add_default_camera(&mut self) -> Entity;
}

impl ScionWorldExtension for World {
    /// Adds a default camera that will be bind to the window size
    fn add_default_camera(&mut self) -> Entity { self.push((DefaultCamera,)) }
}
