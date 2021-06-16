use legion::{Entity, Resources, World};
use scion::{
    config::{scion_config::ScionConfigBuilder, window_config::WindowConfigBuilder},
    core::{
        components::{
            animations::{Animation, AnimationModifier, Animations},
            color::Color,
            material::Material,
            maths::{
                camera::Camera,
                transform::{Coordinates, Transform},
            },
            tiles::{
                sprite::Sprite,
                tilemap::{TileInfos, Tilemap, TilemapInfo},
                tileset::Tileset,
            },
            Square,
        },
        game_layer::{GameLayer, SimpleGameLayer},
        resources::{
            asset_manager::AssetManager,
            inputs::{inputs_controller::InputsController, keycode::KeyCode, InputState},
        },
    },
    utils::{file::app_base_path, maths::Dimensions},
    Scion,
};

#[derive(Default)]
struct BombermanLayer {
    entity: Option<Entity>,
}

impl SimpleGameLayer for BombermanLayer {
    fn on_start(&mut self, world: &mut World, resources: &mut Resources) {
        let asset_ref = resources
            .get_mut::<AssetManager>()
            .expect("Asset Manager is mandatory")
            .register_tileset(Tileset::new(
                app_base_path()
                    .join("examples/bomberman/assets/sokoban_tilesheet.png")
                    .get(),
                13,
                8,
                64,
            ));

        let tilemap_infos =
            TilemapInfo::new(Dimensions::new(5, 5, 2), Transform::default(), asset_ref);

        Tilemap::create(tilemap_infos, world, |_p| TileInfos::new(Some(1), None));

        world.push((Camera::new(768., 768., 10.), Transform::default()));
    }
}

fn main() {
    Scion::app_with_config(
        ScionConfigBuilder::new()
            .with_window_config(
                WindowConfigBuilder::new()
                    .with_resizable(false)
                    .with_dimensions((768, 768))
                    .with_default_background_color(Some(Color::new_rgb(150, 100, 0)))
                    .get(),
            )
            .get(),
    )
    .with_game_layer(GameLayer::strong::<BombermanLayer>("Bomberman"))
    .run();
}
