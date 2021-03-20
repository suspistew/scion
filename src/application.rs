use legion::{
    systems::{Builder, ParallelRunnable},
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
    game_layer::{GameLayer, GameLayerType, LayerAction},
    inputs::Inputs,
    rendering::{renderer_state::RendererState, RendererType},
    utils::{time::Time, window::WindowDimensions},
};

/// `Scion` is the entry point of any application made with Scion's lib.
pub struct Scion {
    #[allow(dead_code)]
    config: ScionConfig,
    world: World,
    resources: Resources,
    schedule: Schedule,
    game_layers: Vec<Box<GameLayer>>,
    window: Option<Window>,
    renderer: Option<RendererState>,
}

impl Scion {
    /// Creates a new `Scion` application.
    /// The application will check for a Scion.toml file at the root to find its configurations.
    /// If this file does not exist, it will create one with default values
    pub fn app() -> ScionBuilder {
        let app_config = ScionConfigReader::read_or_create_scion_toml().expect(
            "Fatal error when trying to retrieve and deserialize `Scion.toml` configuration file.",
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
        self.resources.insert(Inputs::default());
        self.apply_layers_action(LayerAction::Start);
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
        });
    }

    fn next_frame(&mut self) {
        self.apply_layers_action(LayerAction::Update);
        self.resources
            .get_mut::<Time>()
            .expect("Time is an internal resource and can't be missing")
            .frame();
        self.schedule.execute(&mut self.world, &mut self.resources);
        self.apply_layers_action(LayerAction::LateUpdate);
    }

    fn apply_layers_action(&mut self, action: LayerAction) {
        let layers_len = self.game_layers.len();
        if layers_len > 0 {
            for layer_index in (0..layers_len).rev() {
                let current_layer = self
                    .game_layers
                    .get_mut(layer_index)
                    .expect("We just checked the len");
                match &mut current_layer.layer {
                    GameLayerType::Strong(simple_layer) | GameLayerType::Weak(simple_layer) => {
                        match action {
                            LayerAction::Update => {
                                simple_layer.update(&mut self.world, &mut self.resources)
                            }
                            LayerAction::Start => {
                                simple_layer.on_start(&mut self.world, &mut self.resources)
                            }

                            LayerAction::_STOP => {
                                simple_layer.on_stop(&mut self.world, &mut self.resources)
                            }
                            LayerAction::LateUpdate => {
                                simple_layer.late_update(&mut self.world, &mut self.resources)
                            }
                        };
                    }
                }
                if let GameLayerType::Strong(_) = current_layer.layer {
                    break;
                }
            }
        }
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
        let mut scion = Scion {
            config: self.config,
            world: Default::default(),
            resources: Default::default(),
            schedule: self.schedule_builder.build(),
            game_layers: self.game_layers,
            window: Some(window),
            renderer: Some(renderer_state),
        };

        scion.setup();
        scion.run(event_loop);
    }
}
