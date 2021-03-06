use crate::legion::{World, Resources, Schedule};
use crate::legion::systems::{ParallelRunnable, Runnable, Builder};
use std::thread;
use std::time::Duration;

/// `Scion` is the entry point of any application made with Scion engine.
pub struct Scion{
    world: World,
    resources: Resources,
    schedule: Schedule
}

impl Scion {
    /// Creates a new `Scion` application
    pub fn app() -> ScionBuilder {
        ScionBuilder::default()
    }

    fn setup(&mut self) {
        // TODO : Add needed resources
    }

    fn run (mut self) {
        loop {
            self.schedule.execute(&mut self.world, &mut self.resources);
            thread::sleep(Duration::from_secs(1));
        }
    }
}

pub struct ScionBuilder{
    schedule_builder: Builder,
}

impl ScionBuilder{
    fn default() -> Self {
        Self {
            schedule_builder: Default::default(),
        }
    }

    pub fn with_system<S:ParallelRunnable + 'static>(mut self, system: S) -> Self{
        self.schedule_builder.add_system(system);
        self
    }

    pub fn with_thread_local_system<S: Runnable + 'static>(mut self, system: S) -> Self {
        self.schedule_builder.add_thread_local(system);
        self
    }

    fn with_thread_local_fn<F: FnMut(&mut World, &mut Resources) + 'static>(mut self, function: F) -> Self{
        self.schedule_builder.add_thread_local_fn(function);
        self
    }

    /// Builds, setups and runs the Scion application
    pub fn run(mut self) {
        let mut scion = Scion {
            world: Default::default(),
            resources: Default::default(),
            schedule: self.schedule_builder.build()
        };
        scion.setup();
        scion.run();
    }
}

