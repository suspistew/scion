mod scene;
mod character;

use scion::config::scion_config::{ScionConfig, ScionConfigBuilder};
use scion::config::window_config::WindowConfigBuilder;
use scion::Scion;
use crate::scene::MainScene;

fn main(){
    Scion::app_with_config(get_config())
        .with_scene::<MainScene>()
        .run();
}

fn get_config() -> ScionConfig {
    ScionConfigBuilder::new()
        .with_app_name("Pixel Adventures - Scion".to_string())
        .with_window_config(WindowConfigBuilder::new().with_resizable(false).with_dimensions((1024, 768)).get())
        .get()
}