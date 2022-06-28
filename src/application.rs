use std::path::Path;

use log::info;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use crate::core::package::Package;
use crate::core::resources::time::Time;
use crate::core::scene::{Scene, SceneAction, SceneMachine};
use crate::core::systems::InternalPackage;
use crate::{
    config::scion_config::{ScionConfig, ScionConfigReader},
    core::{
        event_handler::update_input_events,
    },
    rendering::{renderer_state::RendererState, RendererType},
};
use crate::core::scheduler::Scheduler;
use crate::core::systems::collider_systems::collider_cleaner_system;

/// `Scion` is the entry point of any application made with Scion's lib.
pub struct Scion {
    #[allow(dead_code)]
    config: ScionConfig,
    internal_world: crate::core::world::World,
    scheduler: Scheduler,
    layer_machine: SceneMachine,
    window: Option<Window>,
    renderer: Option<RendererState>,
}

impl Scion {
    /// Creates a new `Scion` application.
    /// The application will check for a scion.json file at the root to find its configurations.
    /// If this file does not exist, it will create one with default values
    pub fn app() -> ScionBuilder {
        let app_config = ScionConfigReader::read_or_create_default_scion_json().expect(
            "Fatal error when trying to retrieve and deserialize `scion.json` configuration file.",
        );
        Scion::app_with_config(app_config)
    }

    /// Creates a new `Scion` application.
    /// The application will try to read a json file using the provided path.
    pub fn app_with_config_path(config_path: &Path) -> ScionBuilder {
        let app_config = ScionConfigReader::read_scion_json(config_path).expect(
            "Fatal error when trying to retrieve and deserialize `scion.json` configuration file.",
        );
        Scion::app_with_config(app_config)
    }

    /// Creates a new `Scion` application.
    /// The application will use the provided configuration.
    pub fn app_with_config(app_config: ScionConfig) -> ScionBuilder {
        crate::utils::logger::Logger::init_logging(app_config.logger_config.clone());
        info!("Launching Scion application with the following configuration: {:?}", app_config);
        ScionBuilder::new(app_config)
    }

    fn setup(&mut self) {
        self.initialize_internal_resources();
        self.layer_machine.apply_scene_action(
            SceneAction::Start,
            &mut self.internal_world,
        );
    }

    fn initialize_internal_resources(&mut self) {
        let window = self.window.as_ref().expect("No window found during setup");
        self.internal_world.insert_resource(crate::core::resources::window::Window::new(
            (window.inner_size().width, window.inner_size().height),
            window.scale_factor(),
        ));
    }

    fn run(mut self, event_loop: EventLoop<()>) {
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            match event {
                Event::WindowEvent { ref event, window_id }
                    if window_id == self.window.as_mut().unwrap().id() =>
                {
                    match event {
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => {
                            self.internal_world
                                .window()
                                .set_dimensions(physical_size.width, physical_size.height);
                            self.renderer.as_mut().unwrap().resize(
                                *physical_size,
                                self.window.as_ref().expect("Missing window").scale_factor(),
                            );
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            self.renderer.as_mut().unwrap().resize(
                                **new_inner_size,
                                self.window.as_ref().expect("Missing window").scale_factor(),
                            );
                        }
                        WindowEvent::CursorMoved { device_id: _, position, .. } => {
                            let dpi_factor = self
                                .window
                                .as_mut()
                                .unwrap()
                                .current_monitor()
                                .expect("Missing the monitor")
                                .scale_factor();
                            self.internal_world.inputs().set_mouse_position(
                                position.x / dpi_factor,
                                position.y / dpi_factor,
                            );
                        }
                        _ => {}
                    }

                    update_input_events(event, &mut self.internal_world);
                }
                Event::MainEventsCleared => {
                    self.next_frame();
                    self.layer_machine.apply_scene_action(
                        SceneAction::EndFrame,
                        &mut self.internal_world,
                    );
                    self.window.as_mut().unwrap().request_redraw();
                }
                Event::RedrawRequested(_) => {
                    self.renderer.as_mut().unwrap().update(&mut self.internal_world);
                    match self.renderer.as_mut().unwrap().render(&mut self.internal_world, &self.config) {
                        Ok(_) => {}
                        Err(e) => log::error!("{:?}", e),
                    }
                }
                _ => (),
            }
        });
    }

    fn next_frame(&mut self) {
        let frame_duration = self
            .internal_world
            .get_resource_mut::<Time>()
            .expect("Time is an internal resource and can't be missing")
            .frame();
        self.internal_world.timers().add_delta_duration(frame_duration);
        self.layer_machine.apply_scene_action(
            SceneAction::Update,
            &mut self.internal_world,
        );
        self.scheduler.execute(&mut self.internal_world);
        self.layer_machine.apply_scene_action(
            SceneAction::LateUpdate,
            &mut self.internal_world,
        );
        {
            let mut window = self.internal_world.window();
            if let Some(icon) = window.new_cursor() {
                let w = self.window.as_mut().expect("A window is mandatory to run this game !");
                w.set_cursor_icon(*icon);
                window.reset_new_cursor();
            }
        }
        self.internal_world.inputs().reset_inputs();
        self.internal_world.events().cleanup();
    }
}

/// Builder providing convenience functions to build the `Scion` application.
/// This builder is returned when calling [`Scion::app()`] of [`Scion::app_with_config()`]
/// and can't be obtained otherwise.
pub struct ScionBuilder {
    config: ScionConfig,
    scheduler: Scheduler,
    renderer: RendererType,
    scene: Option<Box<dyn Scene>>,
    world: crate::core::world::World,
    internal_world: crate::core::world::World,
}

impl ScionBuilder {
    fn new(config: ScionConfig) -> Self {
        let builder = Self {
            config,
            scheduler: Default::default(),
            renderer: Default::default(),
            scene: Default::default(),
            world: Default::default(),
            internal_world: Default::default(),
        };
        builder.with_package(InternalPackage)
    }

    /// Add a [legion](https://docs.rs/legion/latest/legion/) system to the scheduler.
    ///
    /// To create a system, you have two choices :
    ///
    /// 1. Using macros (Note that you need to add `legion as dependency to your project`
    /// ```no_run
    /// use legion::*;
    /// #[system]
    /// fn hello() {
    ///     println!("Hello world from a system");
    /// }
    /// ```
    /// This will create a function `hello_system()` that you can use on this function
    ///
    /// 2. Using complex system builder , see [legion system builder documentation](https://docs.rs/legion/latest/legion/struct.SystemBuilder.html)
    /// for more precisions.
    pub fn with_system(mut self, system: fn(&mut crate::core::world::World)) -> Self {
        self.scheduler.add_system(system);
        self
    }

    /// Specify which render type you want to use. Note that by default if not set, `Scion` will use [`crate::rendering::RendererType::Scion2D`].
    pub fn with_renderer(mut self, renderer_type: RendererType) -> Self {
        self.renderer = renderer_type;
        self
    }

    /// Add a normal game layer to the pile. Every layer added before in the pile will be called
    pub fn with_scene<T: Scene + Default + 'static>(mut self) -> Self {
        self.scene = Some(Box::new(T::default()));
        self
    }

    ///
    pub fn with_package<P: Package>(mut self, package: P) -> Self {
        package.prepare(&mut self.internal_world);
        package.load(self)
    }

    /// Builds, setups and runs the Scion application, must be called at the end of the building process.
    pub fn run(mut self) {
        let event_loop = EventLoop::new();
        let window_builder: WindowBuilder = self
            .config
            .window_config
            .clone()
            .expect("The window configuration has not been found")
            .into(&self.config);
        let window = window_builder
            .build(&event_loop)
            .expect("An error occured while building the main game window");

        self.add_late_internal_systems_to_schedule();

        let renderer = self.renderer.into_boxed_renderer();
        let renderer_state = futures::executor::block_on(
            RendererState::new(&window, renderer),
        );

        let mut scion = Scion {
            config: self.config,
            internal_world: self.internal_world,
            scheduler: self.scheduler,
            layer_machine: SceneMachine { current_scene: self.scene },
            window: Some(window),
            renderer: Some(renderer_state),
        };

        scion.setup();
        scion.run(event_loop);
    }

    fn add_late_internal_systems_to_schedule(&mut self) {
        self.scheduler.add_system(collider_cleaner_system);
    }
}
