use std::env;
use scion::config::scion_config::ScionConfigBuilder;
use scion::config::window_config::WindowConfigBuilder;
use scion::core::components::color::Color;
use scion::Scion;
use crate::character_control_system::move_char_system;
use crate::scene::MainScene;

mod scene;
mod level_reader;
mod model;
mod character_control_system;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    Scion::app_with_config(
        ScionConfigBuilder::new()
            .with_window_config(WindowConfigBuilder::new()
                .with_dimensions((512, 480))
                .with_resizable(false)
                .with_default_background_color(Some(Color::new_hex("#23a4e2")))
                .get())
            .get(),
    )
        .with_scene::<MainScene>()
        .with_system(move_char_system)
        .run();
}
