use std::collections::HashMap;
use std::time::Duration;
use hecs::Entity;
use scion::core::components::animations::{Animation, AnimationModifier, Animations};
use scion::core::components::maths::collider::{Collider, ColliderMask, ColliderType};
use scion::core::components::maths::coordinates::Coordinates;
use scion::core::components::maths::Pivot;
use scion::core::components::maths::transform::Transform;
use scion::core::components::tiles::atlas::data::{TilemapAtlas, TileObjectClass};
use scion::core::components::tiles::atlas::importer::import_tileset;
use scion::core::components::tiles::sprite::Sprite;
use scion::core::components::tiles::tileset::Tileset;
use scion::core::world::{GameData, World};
use scion::utils::file::app_base_path_join;
use crate::level_scene::GLOBAL_SCALE_MODIFIER;

// Adding entity
pub fn add_ship(data: &mut GameData, atlas: &TilemapAtlas) -> Entity {
    let ship_start = get_ship_start_pos(&atlas);
    let ship_ref = data.assets_mut().register_tileset(Tileset::new("ship".to_string(),
                                                                   app_base_path_join("examples/starlight-1961/assets/space_ship.png"),
                                                                   5, 1, 32, 64));

    let t = import_tileset(&app_base_path_join("examples/starlight-1961/assets/space_ship.scion"));
    let config = t.tile_config_for(0);
    let collider_coords: Vec<Coordinates> = {
        let o = config.objects().iter().filter(|o| o.get_class() == &TileObjectClass::Collider).next().expect("");
        o.get_polygon().iter().map(|c| Coordinates::new(c.x() + 14., c.y())).collect()
    };

    let mut animations = HashMap::new();
    animations.insert("booster".to_string(), Animation::looping(Duration::from_millis(500), vec![AnimationModifier::sprite(vec![1,2,3,2], 0)]));

    data.push((
        Transform::from_xy(ship_start.x() * GLOBAL_SCALE_MODIFIER - 16., ship_start.y() * GLOBAL_SCALE_MODIFIER - 32.),
        Sprite::new(0).pivot(Pivot::Custom(16., 16.)),
        Collider::new(ColliderMask::Character, vec![ColliderMask::Landscape], ColliderType::Polygon(collider_coords)),
        ship_ref,
        Animations::new(animations),
        Ship{ y_force: 0.0, x_force: 0.0, is_landed: true }
    ))
}

fn get_ship_start_pos(atlas: &TilemapAtlas) -> Coordinates {
    let ship = atlas.get_objects().iter().filter(|o| {
        if let TileObjectClass::Custom(a) = o.get_class() {
            a.eq_ignore_ascii_case("Ship")
        } else {
            false
        }
    }).next().expect("A ship is mandatory to start the map");
    Coordinates::new(ship.get_position().x(), ship.get_position().y())
}

// Component struct
pub struct Ship{
    pub y_force: f32,
    pub x_force : f32,
    pub is_landed: bool,
}

