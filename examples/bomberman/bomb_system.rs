use legion::*;

use crate::level_reader::Level;
use crate::{Bomb, BombermanRefs};
use scion::core::components::animations::Animations;
use scion::core::components::tiles::tilemap::Tilemap;
use legion::world::SubWorld;
use scion::utils::maths::Position;


#[system(for_each)]
#[write_component(scion::core::components::tiles::tilemap::Tilemap)]
#[write_component(scion::core::components::tiles::sprite::Sprite)]
pub fn exposion(
    #[resource] level_data: &mut Level,
    #[resource] refs: &mut BombermanRefs,
    world: &mut SubWorld,
    animations: &mut Animations,
    bomb: &Bomb,
) {
    if !animations.any_animation_running() {
        let (mut tilemap_world, mut other_world) = world.split::<&mut Tilemap>();
        let mut tilemap_entry = tilemap_world.entry_mut(refs.tilemap_entity.unwrap()).unwrap();
        let tilemap = tilemap_entry.get_component_mut::<Tilemap>().expect("Unreachable Tilemap in the world");
        {
            let left = level_data.tilemap.get_mut(2).unwrap().values.get_mut(bomb.pos_y).unwrap().get_mut(bomb.pos_x - 1).unwrap();
            if *left == 4 {
                let _r = std::mem::replace(left,0);
                tilemap.modify_sprite_tile(Position::new(bomb.pos_x - 1, bomb.pos_y, 2), 0, &mut other_world);
            }
        }
        {
            let right = level_data.tilemap.get_mut(2).unwrap().values.get_mut(bomb.pos_y).unwrap().get_mut(bomb.pos_x + 1).unwrap();
            if *right == 4 {
                let _r = std::mem::replace(right,0);
                tilemap.modify_sprite_tile(Position::new(bomb.pos_x + 1, bomb.pos_y, 2), 0, &mut other_world);
            }
        }
        {
            let top = level_data.tilemap.get_mut(2).unwrap().values.get_mut(bomb.pos_y - 1).unwrap().get_mut(bomb.pos_x).unwrap();
            if *top == 4 {
                let _r = std::mem::replace(top,0);
                tilemap.modify_sprite_tile(Position::new(bomb.pos_x, bomb.pos_y - 1, 2), 0, &mut other_world);
            }
        }
        {
            let bottom = level_data.tilemap.get_mut(2).unwrap().values.get_mut(bomb.pos_y + 1).unwrap().get_mut(bomb.pos_x).unwrap();
            if *bottom == 4 {
                let _r = std::mem::replace(bottom,0);
                tilemap.modify_sprite_tile(Position::new(bomb.pos_x, bomb.pos_y + 1, 2), 0, &mut other_world);
            }
        }
    }
}