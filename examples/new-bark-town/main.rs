use scion::config::logger_config::LoggerConfig;
use scion::config::scion_config::ScionConfigBuilder;
use scion::config::window_config::WindowConfigBuilder;
use scion::core::components::color::Color;
use scion::Scion;
use crate::scene::MainScene;

mod scene;
mod level_reader;
mod animations;
mod character_control_system;

fn main() {
    Scion::app_with_config(ScionConfigBuilder::new()
        .with_app_name("Pkmn new bark town".to_string())
        .with_logger_config(LoggerConfig::default())
        .with_window_config(WindowConfigBuilder::new()
            .with_dimensions((384, 336))
            .with_resizable(false)
            .with_default_background_color(Some(Color::new_rgb(0, 0, 0)))
            .get())
        .get())
        .with_scene::<MainScene>()
        .with_system(character_control_system::controller_system())
        .run()
}