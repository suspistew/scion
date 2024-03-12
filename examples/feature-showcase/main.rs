use geo_clipper::Clipper;

use scion::config::scion_config::{ScionConfig, ScionConfigBuilder};
use scion::config::window_config::WindowConfigBuilder;
use scion::core::components::color::Color;
use scion::Scion;

use crate::scene::ShowCaseScene;

mod scene;

fn main() {
    Scion::app_with_config(create_config())
        .with_scene::<ShowCaseScene>()
        .run();
}

fn create_config() -> ScionConfig {
    ScionConfigBuilder::new()
        .with_app_name("Showcase - Scion".to_string())
        .with_window_config(
            WindowConfigBuilder::new()
                .with_resizable(false)
                .with_dimensions((1024, 768))
                .with_default_background_color(Some(Color::new_rgb(0,0,0)))
                .get())
        .get()
}