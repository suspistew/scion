mod level_reader;
mod char_animations;
mod character_control_system;
mod bomb_animations;
mod bomb_system;

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
                transform::{Transform},
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

use crate::level_reader::Level;
use scion::core::resources::asset_manager::AssetRef;
use character_control_system::controller_system;
use bomb_system::exposion_system;

#[derive(Default)]
struct BombermanLayer {
    character: Option<Entity>,
}

#[derive(Default)]
pub struct BombermanRefs {
    tileset: Option<AssetRef<Material>>,
    tilemap_entity: Option<Entity>
}

pub struct Bomb{
    pub pos_x: usize,
    pub pos_y: usize
}

#[derive(Default)]
pub struct BombermanInfos {
    pub pos_x: usize,
    pub pos_y: usize
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
                9,
                64,
            ));

        let level = level_reader::read_level("examples/bomberman/assets/test_map.json");

        let tilemap_infos =
            TilemapInfo::new(Dimensions::new(level.width, level.height, level.tilemap.len()), Transform::default(), asset_ref.clone());

        let tilemap = Tilemap::create(tilemap_infos, world, |p| {
            TileInfos::new(Some(*level.tilemap.get(p.layer()).unwrap().values.get(p.y()).unwrap().get(p.x()).unwrap()), None)
        });

        self.character = Some(world.push(create_char(asset_ref.clone(), &level)));
        world.push((Camera::new(640., 640., 10.), Transform::default()));
        resources.insert(level);
        resources.insert(BombermanRefs { tileset: Some(asset_ref), tilemap_entity: Some(tilemap) })
    }
}

fn create_char(asset_ref: AssetRef<Material>, level: &Level) -> (Transform, Sprite, AssetRef<Material>, Animations, BombermanInfos) {
    (
        Transform::from_xy_with_layer((level.character_x * 64) as f32, (level.character_y * 64) as f32, level.tilemap.len() + 2),
        Sprite::new(52),
        asset_ref,
        Animations::new(char_animations::get_animations()),
        BombermanInfos { pos_x: level.character_x, pos_y: level.character_y }
    )
}

fn main() {
    Scion::app_with_config(
        ScionConfigBuilder::new()
            .with_app_name("Scion's Bomberman".to_string())
            .with_window_config(
                WindowConfigBuilder::new()
                    .with_resizable(false)
                    .with_dimensions((640, 640))
                    .get(),
            )
            .get(),
    )
    .with_game_layer(GameLayer::strong::<BombermanLayer>("Bomberman"))
        .with_system(controller_system())
        .with_system(exposion_system())
    .run();
}
