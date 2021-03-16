use legion::systems::{Builder, ParallelRunnable, Runnable};
use legion::{Resources, Schedule, World};
use log::info;

use crate::config::scion_config::{ScionConfig, ScionConfigReader};
use crate::utils::time::Time;

use crate::renderer::{RendererType, ScionRenderer};

use crate::game_layer::{GameLayer, GameLayerType, LayerAction, SimpleGameLayer};
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

use crate::renderer::renderer_state::RendererState;

/// `Scion` is the entry point of any application made with Scion engine.
pub struct Scion {
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
    /// The application will check for a Scion.toml file at root to find its configurations
    pub fn app() -> ScionBuilder {
        let app_config = ScionConfigReader::read_or_create_scion_toml().expect(
            "Fatal error when trying to retrieve and deserialize `Scion.toml` configuration file.",
        );
        Scion::app_with_config(app_config)
    }

    pub fn app_with_config(app_config: ScionConfig) -> ScionBuilder {
        crate::utils::logger::Logger::init_logging(app_config.logger_config.clone());
        info!(
            "Launching Scion application with the following configuration: {:?}",
            app_config
        );
        ScionBuilder::new(app_config)
    }

    pub fn run(mut self, event_loop: EventLoop<()>) {
        event_loop.run(move |event, _, control_flow| {
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
                        .update();
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
            self.next_frame();
        });
    }

    fn setup(&mut self) {
        //let screen_size = context.screen_size();
        self.resources.insert(Time::default());
        // self.resources.insert(WindowDimensions::new(screen_size));
        self.apply_layers_action(LayerAction::START);
    }

    fn next_frame(&mut self) {
        self.apply_layers_action(LayerAction::UPDATE);
        self.resources
            .get_mut::<Time>()
            .expect("Time is an internal resource and can't be missing")
            .frame();
        self.schedule.execute(&mut self.world, &mut self.resources);
        self.apply_layers_action(LayerAction::LATE_UPDATE);
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
                            LayerAction::UPDATE => {
                                simple_layer.update(&mut self.world, &mut self.resources)
                            }
                            LayerAction::START => {
                                simple_layer.on_start(&mut self.world, &mut self.resources)
                            }
                            LayerAction::STOP => {
                                simple_layer.on_stop(&mut self.world, &mut self.resources)
                            }
                            LayerAction::LATE_UPDATE => {
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

    pub fn with_system<S: ParallelRunnable + 'static>(mut self, system: S) -> Self {
        self.schedule_builder.add_system(system);
        self
    }

    pub fn with_thread_local_system<S: Runnable + 'static>(mut self, system: S) -> Self {
        self.schedule_builder.add_thread_local(system);
        self
    }

    pub fn with_thread_local_fn<F: FnMut(&mut World, &mut Resources) + 'static>(
        mut self,
        function: F,
    ) -> Self {
        self.schedule_builder.add_thread_local_fn(function);
        self
    }

    pub fn with_renderer(mut self, renderer_type: RendererType) -> Self {
        self.renderer = renderer_type;
        self
    }

    pub fn with_game_layer(mut self, game_layer: Box<GameLayer>) -> Self {
        self.game_layers.push(game_layer);
        self
    }

    /// Builds, setups and runs the Scion application
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
            crate::renderer::renderer_state::RendererState::new(&window, renderer),
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
