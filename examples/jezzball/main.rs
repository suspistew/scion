mod inputs_layer;
mod main_layer;
mod systems;
mod tilemap_update_layer;
mod utils;

use log::LevelFilter;
use scion::{
    config::{
        logger_config::LoggerConfig, scion_config::ScionConfigBuilder,
        window_config::WindowConfigBuilder,
    },
    core::{components::color::Color, game_layer::GameLayer},
    Scion,
};

use crate::{
    inputs_layer::InputLayer, main_layer::JezzBallLayer, tilemap_update_layer::TilemapUpdateLayer,
};

fn main() {
    Scion::app_with_config(
        ScionConfigBuilder::new()
            .with_app_name("Jezzball scion".to_string())
            .with_logger_config(LoggerConfig { level_filter: LevelFilter::Off })
            .with_window_config(
                WindowConfigBuilder::new()
                    .with_default_background_color(Some(Color::new_rgb(10, 10, 10)))
                    .with_resizable(false)
                    .with_dimensions((1108, 629))
                    .get(),
            )
            .get(),
    )
    .with_game_layer(GameLayer::strong::<JezzBallLayer>("MAIN"))
    .with_game_layer(GameLayer::weak::<InputLayer>("INPUTS"))
    .with_game_layer(GameLayer::weak::<TilemapUpdateLayer>("TILEMAP_LAYER"))
    .with_system(systems::ball::ball_control_system())
    .with_system(systems::lines::line_update_system())
    .run();
}
