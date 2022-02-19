use log::LevelFilter;
use scion::{
    config::{
        logger_config::LoggerConfig,
        scion_config::{ScionConfig, ScionConfigBuilder},
        window_config::WindowConfigBuilder,
    },
    utils::file::{app_base_path, PathBuilder},
    Scion,
};

use crate::{
    scene::MainScene,
    systems::{
        move_system::move_piece_system, piece_system::piece_update_system,
        rotation_system::piece_rotation_system, score_system::score_system,
    },
};

mod components;
pub mod resources;
mod scene;
mod systems;

fn main() {
    Scion::app_with_config(app_config())
        .with_scene::<MainScene>()
        .with_system(piece_update_system())
        .with_system(move_piece_system())
        .with_system(piece_rotation_system())
        .with_system(score_system())
        .run();
}

fn app_config() -> ScionConfig {
    ScionConfigBuilder::new()
        .with_app_name("Tetris".to_string())
        .with_logger_config(LoggerConfig { level_filter: LevelFilter::Warn })
        .with_window_config(
            WindowConfigBuilder::new().with_dimensions((544, 704)).with_resizable(true).get(),
        )
        .get()
}

pub fn asset_path() -> PathBuilder {
    app_base_path().join("examples/tetris/assets/")
}
