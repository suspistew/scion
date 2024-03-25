use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;
use std::sync::{Arc, mpsc};
use std::thread;
use std::time::Duration;

use log::{debug, info};
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
    graphics::rendering::{renderer_state::RendererState, RendererType},
};
use crate::core::application_builder::ScionBuilder;
use crate::core::scion_runner::ScionRunner;
use crate::core::state::GameState;
use crate::graphics::rendering::scion2d::Scion2D;
use crate::graphics::windowing::input_event_handler::update_input_events;
use crate::graphics::windowing::WindowingEvent;

/// `Scion` is the entry point of any application made with Scion's lib.
pub struct Scion {
    #[allow(dead_code)]
    pub(crate) config: ScionConfig,
    pub(crate) game_data: GameData,
    pub(crate) scheduler: Scheduler,
    pub(crate) layer_machine: SceneMachine,
    pub(crate) renderer: RendererType,
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


    // There was no technical need to have the run function inside the Scion struct, but I made it here because I wanted the
    // main window loop & game loop to be in the main application file.
    pub(crate) fn run(mut self) {
        if self.config.window_config.is_none() {
            // Running window less mode, so launching the runner in the main thread
            info!("Launching game in text mode");
            ScionRunner {
                game_data: self.game_data,
                scheduler: self.scheduler,
                layer_machine: self.layer_machine,
                renderer: None,
                window: None,
                main_thread_receiver: None,
                scion_renderer: Default::default(),
            }.launch_game_loop();
        } else {
            // Game is running in a window, it must be created & handled in the main thread, so
            // the game loop is going to another thread.
            let event_loop = EventLoop::new().expect("Event loop could not be created");
            event_loop.set_control_flow(ControlFlow::Poll);
            let window_builder: WindowBuilder = self.config.window_config
                .clone()
                .expect("The window configuration has not been found")
                .into(&self.config);
            let window = Arc::new(window_builder
                .build(&event_loop)
                .expect("An error occured while building the main game window"));
            let renderer = Box::<Scion2D>::default();
            let renderer_state = futures::executor::block_on(RendererState::new(window.clone(), renderer, self.config.window_config.as_ref().unwrap().default_background_color.clone()));
            let (event_sender, receiver) = mpsc::channel::<WindowingEvent>();
            thread::spawn(move || {
                ScionRunner {
                    game_data: self.game_data,
                    scheduler: self.scheduler,
                    layer_machine: self.layer_machine,
                    renderer: Some(renderer_state),
                    window: Some(window),
                    main_thread_receiver: Some(receiver),
                    scion_renderer: Default::default(),
                }.launch_game_loop()
            });

           let _result = event_loop.run(move |event,loopd| {
               loopd.set_control_flow(ControlFlow::Poll);
               match event {
                   Event::WindowEvent { event, window_id } => {
                        match event {
                            WindowEvent::CloseRequested => loopd.exit(),
                            WindowEvent::RedrawRequested => {
                                let _r = event_sender.send(WindowingEvent{ window_event: Some(WindowEvent::RedrawRequested), id: Some(window_id), redraw: true });
                            },
                            e => {
                                let _r = event_sender.send(WindowingEvent{ window_event: Some(e), id: Some(window_id), redraw: false });
                            }
                        }
                   },
                   Event::AboutToWait => {
                       //
                   }
                   _ => {}
               }
           });
        }
    }
}
