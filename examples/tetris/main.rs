use scion::{
    core::{
        game_layer::GameLayer,
    },
    utils::file::app_base_path,
    Scion,
};




use scion::config::scion_config::{ScionConfig, ScionConfigBuilder};
use scion::config::window_config::WindowConfigBuilder;
use crate::layer::TetrisLayer;
use std::path::PathBuf;
use crate::systems::piece_system::piece_update_system;
use crate::systems::move_system::move_piece_system;
use crate::systems::rotation_system::piece_rotation_system;
use crate::systems::score_system::score_system;

mod layer;
mod components;
mod systems;
pub mod resources;

fn main() {
    Scion::app_with_config(app_config())
        .with_game_layer(GameLayer::weak::<TetrisLayer>())
        .with_system(piece_update_system())
        .with_system(move_piece_system())
        .with_system(piece_rotation_system())
        .with_system(score_system())
        .run();
}

fn app_config() -> ScionConfig {
    ScionConfigBuilder::new()
        .with_app_name("Tetris".to_string())
        .with_window_config(
            WindowConfigBuilder::new()
                .with_dimensions((544, 704))
                .with_resizable(true)
                .get()
        )
        .get()
}

pub fn asset_path() -> PathBuf {
    app_base_path()
        .expect("base_path is mandatory to run the game")
        .join("assets").join("tetris")
}
