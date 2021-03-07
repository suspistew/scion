use crate::legion::{World, Resources, Schedule};
use crate::legion::systems::{ParallelRunnable, Runnable, Builder};
use std::thread;
use std::time::Duration;
use crate::config::scion_config::{ScionConfig, ScionConfigReader};
use winit::window::{Window, WindowBuilder};
use winit::event_loop::{EventLoop, ControlFlow};
use winit::event::{Event, WindowEvent};
use crate::utils::time::Time;
use crate::utils::frame_limiter::{FRAME_LOCKED, FrameLimiter, FrameLimiterStrategy};

/// `Scion` is the entry point of any application made with Scion engine.
pub struct Scion{
    config: ScionConfig,
    world: World,
    resources: Resources,
    schedule: Schedule,
    window: Option<Window>
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
        self.resources.insert(Time::default());
        self.resources.insert(FrameLimiter::new(
            self.config.frame_limiter.clone().unwrap_or(Default::default())
        ));
    }

    fn run (mut self) {
        let event_loop = EventLoop::new();
        let window_builder: WindowBuilder = self.config.window_config.clone()
            .expect("The window configuration has not been found").into();
        self.window = Some(window_builder.build(&event_loop).expect(""));

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;
            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id,
                } if window_id == self.window.as_ref().unwrap().id() => {
                    *control_flow = ControlFlow::Exit;
                },
                Event::MainEventsCleared => {
                    self.window.as_ref().unwrap().request_redraw();
                }
                _ => (),
            }
            self.next_frame();
        });
    }

    fn next_frame(&mut self) {
        let locked_ecs = unsafe { FRAME_LOCKED };
        if !locked_ecs {
            self.resources.get_mut::<Time>()
                .expect("Time is an internal resource and can't be missing")
                .frame();
            self.schedule.execute(&mut self.world, &mut self.resources);
            self.resources.get_mut::<FrameLimiter>().unwrap().end_frame();
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
            schedule: self.schedule_builder.build(),
            window: None
        };
        scion.setup();
        scion.run();
    }
}

