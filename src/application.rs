use crate::legion::{World, Resources, Schedule};
use crate::legion::systems::{ParallelRunnable, Runnable, Builder};
use std::thread;
use std::time::Duration;
use crate::config::scion_config::{ScionConfig, ScionConfigReader};

/// `Scion` is the entry point of any application made with Scion engine.
pub struct Scion{
    config: ScionConfig,
    world: World,
    resources: Resources,
    schedule: Schedule
}

impl Scion {
    /// Creates a new `Scion` application.
    /// The application need to have a Scion.toml file at root to find its mandatory configurations
    pub fn app() -> ScionBuilder {
        let app_config = ScionConfigReader::read_or_create_scion_toml()
                .expect("Fatal error when trying to retrieve and deserialize `Scion.toml` configuration file.");
        println!("Launching Scion application with the following configuration: {:?}", app_config);
        ScionBuilder::new(app_config)
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
    config: ScionConfig,
    schedule_builder: Builder,
}

impl ScionBuilder{
    fn new(config: ScionConfig) -> Self {
        Self {
            config,
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
            config: self.config,
            world: Default::default(),
            resources: Default::default(),
            schedule: self.schedule_builder.build()
        };
        scion.setup();
        scion.run();
    }
}

