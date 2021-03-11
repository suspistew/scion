use legion::{Resources, Schedule, World};
use legion::systems::{Builder, ParallelRunnable, Runnable};
use log::info;
use miniquad::{conf, Context, EventHandlerFree, UserData};

use crate::config::scion_config::{ScionConfig, ScionConfigReader};
use crate::utils::time::Time;
use crate::utils::window::WindowDimensions;
use crate::renderer::{RendererType, ScionRenderer};

use crate::renderer::bidimensional::triangle::Triangle;

/// `Scion` is the entry point of any application made with Scion engine.
pub struct Scion {
    config: ScionConfig,
    world: World,
    resources: Resources,
    schedule: Schedule,
    context: Option<Context>,
    renderer: Box<dyn ScionRenderer>,
}

impl EventHandlerFree for Scion {
    fn update(&mut self) {
        self.next_frame();
    }

    fn draw(&mut self) {
        self.renderer.draw(
            self.context.as_mut().expect("Miniquad context is mandatory"),
            &mut self.world, &mut self.resources)
    }

    fn resize_event(&mut self, w: f32, h: f32) {
        self.resources
            .get_mut::<WindowDimensions>().expect("Missing Screen Dimension Resource. Did something deleted it ?").set(w, h);
    }
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

    fn setup(mut self, context: Context) -> Self {
        let screen_size = context.screen_size();
        self.context = Some(context);
        self.resources.insert(Time::default());

        self.resources.insert(WindowDimensions::new(screen_size));
        self.world.push((Triangle,));
        self
    }

    fn next_frame(&mut self) {
        self.resources.get_mut::<Time>().expect("Time is an internal resource and can't be missing").frame();
        self.schedule.execute(&mut self.world, &mut self.resources);
    }
}

pub struct ScionBuilder {
    config: ScionConfig,
    schedule_builder: Builder,
    renderer: RendererType
}

impl ScionBuilder {
    fn new(config: ScionConfig) -> Self {
        Self {
            config,
            schedule_builder: Default::default(),
            renderer: Default::default()
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

    pub fn with_renderer(mut self, renderer_type: RendererType) -> Self{
        self.renderer = renderer_type;
        self
    }

    /// Builds, setups and runs the Scion application
    pub fn run(mut self) {
        let scion = Scion {
            config: self.config,
            world: Default::default(),
            resources: Default::default(),
            schedule: self.schedule_builder.build(),
            context: None,
            renderer: self.renderer.into_boxed_renderer()
        };

        let mut miniquad_conf = conf::Conf::default();
        miniquad_conf.high_dpi=true;
        if let Some(window_config) = scion.config.window_config.as_ref() {
            miniquad_conf.fullscreen = window_config.fullscreen;
        }
        miniquad::start(miniquad_conf, |ctx| UserData::free(scion.setup(ctx)));
    }
}
