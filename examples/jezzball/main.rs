use log::LevelFilter;

use crate::main_scene::MainScene;
use scion::{
    config::{
        logger_config::LoggerConfig, scion_config::ScionConfigBuilder,
        window_config::WindowConfigBuilder,
    },
    core::components::color::Color,
    Scion,
};

mod main_scene;
mod systems;
mod utils;

fn main() {
    Scion::app_with_config(
        ScionConfigBuilder::new()
            .with_app_name("Jezzball scion".to_string())
            .with_logger_config(LoggerConfig { scion_level_filter: LevelFilter::Info, level_filter: LevelFilter::Info })
            .with_window_config(
                WindowConfigBuilder::new()
                    .with_default_background_color(Some(Color::new_rgb(10, 10, 10)))
                    .with_resizable(true)
                    .with_dimensions((1108, 629))
                    .get(),
            )
            .get(),
    )
    .with_scene::<MainScene>()
    .with_system(systems::ball::ball_control_system)
    .with_system(systems::lines::line_update_system)
    .run();
}
