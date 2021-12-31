use scion::config::scion_config::ScionConfigBuilder;
use scion::config::window_config::WindowConfigBuilder;
use scion::core::components::color::Color;
use scion::Scion;
use crate::scene::MainScene;

mod scene;
mod level_reader;

fn main() {
    Scion::app_with_config(ScionConfigBuilder::new()
        .with_app_name("Pkmn new bark town".to_string())
        .with_window_config(WindowConfigBuilder::new()
            .with_dimensions((384, 336))
            .with_resizable(false)
            .with_default_background_color(Some(Color::new_rgb(0, 0, 0)))
            .get())
        .get())
        .with_scene::<MainScene>()
        .run()
}