use hecs::Entity;
use log::debug;
use scion::core::components::color::Color;
use scion::core::components::material::Material;
use scion::core::components::maths::camera::Camera;
use scion::core::components::maths::collider::Collider;
use scion::core::components::maths::hierarchy::Parent;
use scion::core::components::maths::transform::Transform;
use scion::core::components::Square;
use scion::core::components::tiles::sprite::Sprite;
use scion::core::components::tiles::tilemap::{TileInfos, Tilemap, TilemapInfo};
use scion::core::components::tiles::tileset::Tileset;
use scion::core::scene::Scene;
use scion::core::world::{GameData, World};
use scion::utils::file::app_base_path;
use scion::utils::maths::Dimensions;
use crate::level_reader::read_level;
use crate::model::Hero;

#[derive(Default)]
pub struct MainScene{
    pub current_tilemap: Option<Entity>,
    pub mario: Option<Entity>
}

impl Scene for MainScene{
    fn on_start(&mut self, data: &mut GameData) {
        debug!("on_start on main scene started");
        self.current_tilemap = Some(create_level(data));
        self.mario = Some(add_character(data));
        let camera_transform = Transform::from_xy(-64., -384.);
        data.push((Camera::new(512., 480.), camera_transform, Parent(self.mario.unwrap())));

        debug!("on_start on main scene ended");
    }

    fn late_update(&mut self, data: &mut GameData) {
        // Update the hero position acording to its velocity
        let hero = data.entry_mut::<(&mut Hero, &mut Transform)>(self.mario.unwrap()).unwrap();
        if hero.0.velocity != 0 {
            hero.1.append_x(hero.0.velocity as f32 / 30 as f32 * 2.5);
        }
    }
}

fn create_level(data: &mut GameData) -> Entity{
    let asset_ref = data.assets_mut().register_tileset(
        Tileset::from_atlas("examples/mario-bis/assets/tileset_atlas.json").unwrap(),
    );

    let mut level = read_level("examples/mario-bis/assets/level1.json");
    let mut scale = Transform::default();

    scale.set_scale(2.0);

    let tilemap_infos = TilemapInfo::new(
        Dimensions::new(level.width, level.height, level.layers.len()),
        scale,
        asset_ref.clone(),
    );

    Tilemap::create(tilemap_infos, data, |p| {
        let tile_val = level.tile_at(p);
        let mut tile = None;
        if tile_val >= 0 {
            tile = Some(tile_val as usize);
        }
        TileInfos::new(tile, None)
    })
}

fn add_character(data: &mut GameData) -> Entity {
    let asset_ref = data.assets_mut().register_tileset(Tileset::new(
        app_base_path().join("examples/mario-bis/assets/mario.png").get(),
        4,
        3,
        16,
    ));
    let mut t = Transform::from_xyz(160., 480., 2);
    t.set_scale(2.0);
    data.push((
        Hero { velocity: 0, gravity: 1, landed: false },
        Sprite::new(0),
        asset_ref,
        t,
    ))
}
