use std::path::Path;

use legion::{
    systems::{Builder, ParallelRunnable, ResourceTypeId},
    Resources, Schedule, World,
};
use log::info;
use winit::{
    event::{ElementState, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use crate::{
    config::scion_config::{ScionConfig, ScionConfigReader},
    core::{
        game_layer::{GameLayer, GameLayerController, GameLayerMachine, LayerAction},
        legion_ext::PausableSystem,
        resources::{
            inputs::Inputs,
            time::{Time, Timers},
            window::WindowDimensions,
        },
        state::GameState,
        systems::{hierarchy_system::children_manager_system, ui_text_system::ui_text_bitmap_update_system},
        inputs::keycode::KeyCode
    },
    rendering::{renderer_state::RendererState, RendererType},
};

/// `Scion` is the entry point of any application made with Scion's lib.
pub struct Scion {
    #[allow(dead_code)]
    config: ScionConfig,
    world: World,
    resources: Resources,
    schedule: Schedule,
    layer_machine: GameLayerMachine,
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
        info!(
            "Launching Scion application with the following configuration: {:?}",
            app_config
        );
        ScionBuilder::new(app_config)
    }

    fn setup(&mut self) {
        let inner_size = self
            .window
            .as_ref()
            .expect("No window found during setup")
            .inner_size();
        self.resources
            .insert(WindowDimensions::new((inner_size.width, inner_size.height)));

        self.resources.insert(Time::default());
        self.resources.insert(Timers::default());
        self.resources.insert(Inputs::default());
        self.resources.insert(GameState::default());
        self.resources.insert(GameLayerController::default());

        self.layer_machine.apply_layers_action(
            LayerAction::Start,
            &mut self.world,
            &mut self.resources,
        );
    }

    fn run(mut self, event_loop: EventLoop<()>) {
        event_loop.run(move |event, _, control_flow| {
            let mut click_event = false; // WIP while we don't use events internally
            *control_flow = ControlFlow::Poll;
            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id
                    == self
                        .window
                        .as_ref()
                        .expect("A window is mandatory to run this game !")
                        .id() =>
                {
                    match event {
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => {
                            self.resources
                                .get_mut::<WindowDimensions>()
                                .expect("Missing mandatory ressource : WindowDimension")
                                .set(physical_size.width, physical_size.height);
                            self.renderer
                                .as_mut()
                                .expect("A renderer is mandatory to run this game !")
                                .resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            self.renderer
                                .as_mut()
                                .expect("A renderer is mandatory to run this game !")
                                .resize(**new_inner_size);
                        }
                        WindowEvent::CursorMoved {
                            device_id: _,
                            position,
                            ..
                        } => {
                            let dpi_factor = self
                                .window
                                .as_ref()
                                .expect("Missing the window")
                                .current_monitor()
                                .expect("Missing the monitor")
                                .scale_factor();
                            self.resources
                                .get_mut::<Inputs>()
                                .expect("Missing mandatory ressource : Inputs")
                                .mouse_mut()
                                .set_position(position.x / dpi_factor, position.y / dpi_factor);
                        }
                        WindowEvent::MouseInput {
                            state, button: _, ..
                        } => {
                            match state {
                                ElementState::Pressed => {
                                    click_event = true; // WIP while we don't use events internally
                                }
                                ElementState::Released => {}
                            }
                        }
                        WindowEvent::KeyboardInput {
                            input, ..
                        } => {
                            if let Some(keycode)= input.virtual_keycode {
                                match input.state {
                                    ElementState::Pressed => {
                                        self.resources
                                            .get_mut::<Inputs>()
                                            .expect("Missing mandatory ressource : Inputs")
                                            .keyboard_mut()
                                            .press(KeyCode::from(keycode));
                                    }
                                    ElementState::Released => {
                                        self.resources
                                            .get_mut::<Inputs>()
                                            .expect("Missing mandatory ressource : Inputs")
                                            .keyboard_mut()
                                            .release(KeyCode::from(keycode));
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
                Event::MainEventsCleared => {
                    self.window.as_ref().unwrap().request_redraw();
                }
                Event::RedrawRequested(_) => {
                    self.renderer
                        .as_mut()
                        .expect("A renderer is mandatory to run this game !")
                        .update(&mut self.world, &mut self.resources);
                    match self
                        .renderer
                        .as_mut()
                        .expect("A renderer is mandatory to run this game !")
                        .render(&mut self.world, &mut self.resources)
                    {
                        Ok(_) => {}
                        Err(e) => log::error!("{:?}", e),
                    }
                }
                _ => (),
            }
            self.resources
                .get_mut::<Inputs>()
                .expect("Missing mandatory ressource : Inputs")
                .mouse_mut()
                .set_click_event(click_event);
            self.next_frame();
            self.layer_machine.apply_layers_action(
                LayerAction::EndFrame,
                &mut self.world,
                &mut self.resources,
            );
        });
    }

    fn next_frame(&mut self) {
        let frame_duration = self
            .resources
            .get_mut::<Time>()
            .expect("Time is an internal resource and can't be missing")
            .frame();
        self.resources
            .get_mut::<Timers>()
            .expect("Missing mandatory ressource : Timers")
            .add_delta_duration(frame_duration);
        self.layer_machine.apply_layers_action(
            LayerAction::Update,
            &mut self.world,
            &mut self.resources,
        );
        self.schedule.execute(&mut self.world, &mut self.resources);
        self.layer_machine.apply_layers_action(
            LayerAction::LateUpdate,
            &mut self.world,
            &mut self.resources,
        );
        self.resources
            .get_mut::<Inputs>()
            .expect("Missing mandatory ressource : Inputs")
            .keyboard_mut()
            .clear_events();
    }
}

/// Builder providing convenience functions to build the `Scion` application.
/// This builder is returned when calling [`Scion::app()`] of [`Scion::app_with_config()`]
/// and can't be obtained otherwise.
pub struct ScionBuilder {
    config: ScionConfig,
    schedule_builder: Builder,
    renderer: RendererType,
    game_layers: Vec<Box<GameLayer>>,
}

impl ScionBuilder {
    fn new(config: ScionConfig) -> Self {
        Self {
            config,
            schedule_builder: Default::default(),
            renderer: Default::default(),
            game_layers: Default::default(),
        }
    }

    /// Add a [legion](https://docs.rs/legion/latest/legion/) system to the scheduler.
    ///
    /// To create a system, you have two choices :
    ///
    /// 1. Using macros (Note that you need to add `legion as dependency to your project`
    /// ```no_run
    /// #[system]
    /// fn hello() {
    ///     println!("Heullo world from a system");
    /// }
    /// ```
    /// This will create a function `hello_system()` that you can use on this function
    ///
    /// 2. Using complex system builder , see [legion system builder documentation](https://docs.rs/legion/latest/legion/struct.SystemBuilder.html)
    /// for more precisions.
    pub fn with_system<S: ParallelRunnable + 'static>(mut self, system: S) -> Self {
        self.schedule_builder.add_system(system);
        self
    }

    pub fn with_pausable_system<S: ParallelRunnable + 'static>(
        mut self,
        system: S,
        condition: fn(GameState) -> bool,
    ) -> Self {
        let (resource_reads, _) = system.reads();
        let resource_reads = resource_reads
            .iter()
            .copied()
            .chain(std::iter::once(ResourceTypeId::of::<GameState>()))
            .collect::<Vec<_>>();
        let boxed_condition = Box::new(condition);
        let pausable_system = PausableSystem {
            system,
            decider: boxed_condition,
            resource_reads,
        };
        self.schedule_builder.add_system(pausable_system);
        self
    }

    /// Specify which render type you want to use. Note that by default if not set, `Scion` will use [`crate::rendering::RendererType::Scion2D`].
    pub fn with_renderer(mut self, renderer_type: RendererType) -> Self {
        self.renderer = renderer_type;
        self
    }

    /// Add a game layer to the pile. Not that the order in which you add them is important if you use the concept of Strong layers.
    pub fn with_game_layer(mut self, game_layer: Box<GameLayer>) -> Self {
        self.game_layers.push(game_layer);
        self
    }

    /// Builds, setups and runs the Scion application, must be called at the end of the building process.
    pub fn run(mut self) {
        let event_loop = EventLoop::new();
        let window_builder: WindowBuilder = self
            .config
            .window_config
            .clone()
            .expect("The window configuration has not been found")
            .into();
        let window = window_builder.build(&event_loop).expect("");
        let renderer = self.renderer.into_boxed_renderer();
        let renderer_state = futures::executor::block_on(
            crate::rendering::renderer_state::RendererState::new(&window, renderer),
        );

        self.schedule_builder.add_system(children_manager_system());
        self.schedule_builder.add_system(ui_text_bitmap_update_system());

        let mut scion = Scion {
            config: self.config,
            world: Default::default(),
            resources: Default::default(),
            schedule: self.schedule_builder.build(),
            layer_machine: GameLayerMachine {
                game_layers: self.game_layers,
            },
            window: Some(window),
            renderer: Some(renderer_state),
        };

        scion.setup();
        scion.run(event_loop);
    }
}
