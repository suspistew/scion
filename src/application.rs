use std::path::Path;
use std::sync::Arc;

use log::info;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};
use winit::dpi::{PhysicalSize, Size};
use winit::event::StartCause;


use crate::core::package::Package;
use crate::core::resources::time::Time;
use crate::core::scene::{Scene, SceneAction, SceneMachine};
use crate::core::scheduler::Scheduler;

use crate::core::systems::InternalPackage;
use crate::core::world::GameData;
use crate::{
    config::scion_config::{ScionConfig, ScionConfigReader},
    core::event_handler::update_input_events,
    rendering::{renderer_state::RendererState, RendererType},
};
use crate::core::state::GameState;

/// `Scion` is the entry point of any application made with Scion's lib.
pub struct Scion {
    #[allow(dead_code)]
    config: ScionConfig,
    game_data: GameData,
    scheduler: Scheduler,
    layer_machine: SceneMachine,
    window: Option<Arc<Window>>,
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
        info!("Starting a Scion app, with the following configuration \n {:?}", app_config);
        ScionBuilder::new(app_config)
    }

    fn setup(&mut self) {
        self.initialize_internal_resources();
        self.layer_machine.apply_scene_action(SceneAction::Start, &mut self.game_data);
    }

    fn initialize_internal_resources(&mut self) {
        let window = self.window.as_ref().expect("No window found during setup");
        self.game_data.insert_resource(crate::core::resources::window::Window::new(
            (window.inner_size().width, window.inner_size().height),
            window.scale_factor(),
        ));
    }

    fn run(mut self, event_loop: EventLoop<()>) {
        let _result = event_loop.run(move |event,loopd| {
            loopd.set_control_flow(ControlFlow::Poll);

            match event {
                Event::WindowEvent { ref event, window_id }
                    if window_id == self.window.as_mut().unwrap().id() =>
                {
                    match event {
                        WindowEvent::CloseRequested => loopd.exit(),
                        WindowEvent::Resized(physical_size) => {
                            self.game_data
                                .window()
                                .set_dimensions(physical_size.width, physical_size.height);
                            self.game_data
                                .window()
                                .set_dpi(self.window.as_ref().expect("Missing window").scale_factor());
                            self.renderer.as_mut().unwrap().resize(
                                *physical_size,
                                self.window.as_ref().expect("Missing window").scale_factor(),
                            );
                        }
                        WindowEvent::ScaleFactorChanged { scale_factor,  .. } => {
                            self.renderer.as_mut().unwrap().resize(
                                self.window.as_ref().expect("Missing window").inner_size(),
                                *scale_factor,
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
                            self.game_data.inputs().set_mouse_position(
                                position.x / dpi_factor,
                                position.y / dpi_factor,
                            );
                        }
                        WindowEvent::RedrawRequested => {
                            self.renderer.as_mut().unwrap().update(&mut self.game_data);
                            match self.renderer.as_mut().unwrap().render(&mut self.game_data, &self.config)
                            {
                                Ok(_) => {}
                                Err(e) => log::error!("{:?}", e),
                            }
                        }
                        _ => {}
                    }
                    update_input_events(event, &mut self.game_data);
                }
                Event::AboutToWait => {
                    self.next_frame();
                    self.layer_machine
                        .apply_scene_action(SceneAction::EndFrame, &mut self.game_data);
                    self.window.as_mut().unwrap().request_redraw();
                }
                _ => (),
            }
        });
    }

    fn next_frame(&mut self) {
        let frame_duration = self
            .game_data
            .get_resource_mut::<Time>()
            .expect("Time is an internal resource and can't be missing")
            .frame();
        self.game_data.timers().add_delta_duration(frame_duration);
        self.layer_machine.apply_scene_action(SceneAction::Update, &mut self.game_data);
        self.scheduler.execute(&mut self.game_data);
        self.layer_machine.apply_scene_action(SceneAction::LateUpdate, &mut self.game_data);
        self.update_cursor();
        self.game_data.inputs().reset_inputs();
        self.game_data.events().cleanup();
    }

    fn update_cursor(&mut self) {
        {
            let mut window = self.game_data.window();
            if let Some(icon) = window.new_cursor() {
                let w = self.window.as_mut().expect("A window is mandatory to run this game !");
                w.set_cursor_icon(*icon);
            }
            if let Some(dimensions) = window.new_dimensions() {
                let w = self.window.as_mut().expect("A window is mandatory to run this game !");
                let _r = w.request_inner_size(Size::Physical(PhysicalSize::new(dimensions.0 * window.dpi() as u32,
                                                                               dimensions.1 * window.dpi() as u32)));
            }
            window.reset_future_settings()
        }
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
    world: GameData,
}

impl ScionBuilder {
    fn new(config: ScionConfig) -> Self {
        let builder = Self {
            config,
            scheduler: Default::default(),
            renderer: Default::default(),
            scene: Default::default(),
            world: Default::default(),
        };
        builder.with_package(InternalPackage)
    }

    /// Specify a system to add to the scheduler.
    pub fn with_system(mut self, system: fn(&mut GameData)) -> Self {
        self.scheduler.add_system(system);
        self
    }

    /// Specify a system to add to the scheduler with a conditional pausing flag function.
    pub fn with_pausable_system(mut self, system: fn(&mut GameData), pause_condition: fn(&GameState) -> bool) -> Self {
        self.scheduler.add_pausable_system(system, pause_condition);
        self
    }

    /// Specify which render type you want to use. Note that by default if not set, `Scion` will use [`crate::rendering::RendererType::Scion2D`].
    pub fn with_renderer(mut self, renderer_type: RendererType) -> Self {
        self.renderer = renderer_type;
        self
    }

    /// Set the scene to the given one. Only one scene can be executed at a time
    pub fn with_scene<T: Scene + Default + 'static>(mut self) -> Self {
        self.scene = Some(Box::<T>::default());
        self
    }

    ///
    pub fn with_package<P: Package>(mut self, package: P) -> Self {
        package.prepare(&mut self.world);
        package.load(self)
    }

    /// Builds, setups and runs the Scion application, must be called at the end of the building process.
    pub fn run(mut self) {
        let event_loop = EventLoop::new().expect("Event loop could not be created");
        event_loop.set_control_flow(ControlFlow::Poll);
        let window_builder: WindowBuilder = self
            .config
            .window_config
            .clone()
            .expect("The window configuration has not been found")
            .into(&self.config);
        let window = Arc::new(window_builder
            .build(&event_loop)
            .expect("An error occured while building the main game window"));

        self.add_late_internal_systems_to_schedule();

        let renderer = self.renderer.into_boxed_renderer();
        let renderer_state = futures::executor::block_on(RendererState::new(window.clone(), renderer));

        let mut scion = Scion {
            config: self.config,
            game_data: self.world,
            scheduler: self.scheduler,
            layer_machine: SceneMachine { current_scene: self.scene, current_scene_started: false },
            window: Some(window),
            renderer: Some(renderer_state),
        };

        scion.setup();
        scion.run(event_loop);
    }

    fn add_late_internal_systems_to_schedule(&mut self) {}
}
