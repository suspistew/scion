use scion::{
    core::components::{animations::Animations, tiles::tilemap::Tilemap},
    utils::maths::Position,
};
use scion::core::world::World;

use crate::{level_reader::Level, Bomb, BombermanRefs};

pub fn explosion_system(world: &mut World) {
    let (world, resources) = world.split();

    let mut level_data = resources.get_resource_mut::<Level>().unwrap();
    let refs = resources.get_resource_mut::<BombermanRefs>().unwrap();

    let mut to_modify = Vec::new();

    for (_, (bomb, animations)) in world.query_mut::<(&Bomb, &mut Animations)>(){
        if !animations.any_animation_running() {
            {
                let left = level_data
                    .tilemap
                    .get_mut(2)
                    .unwrap()
                    .values
                    .get_mut(bomb.pos_y)
                    .unwrap()
                    .get_mut(bomb.pos_x - 1)
                    .unwrap();
                if *left == 4 {
                    let _r = std::mem::replace(left, 0);
                    to_modify.push((
                        Position::new(bomb.pos_x - 1, bomb.pos_y, 2),
                        0)
                    );
                }
            }
            {
                let right = level_data
                    .tilemap
                    .get_mut(2)
                    .unwrap()
                    .values
                    .get_mut(bomb.pos_y)
                    .unwrap()
                    .get_mut(bomb.pos_x + 1)
                    .unwrap();
                if *right == 4 {
                    let _r = std::mem::replace(right, 0);
                    to_modify.push((
                        Position::new(bomb.pos_x + 1, bomb.pos_y, 2),
                        0)
                    );
                }
            }
            {
                let top = level_data
                    .tilemap
                    .get_mut(2)
                    .unwrap()
                    .values
                    .get_mut(bomb.pos_y - 1)
                    .unwrap()
                    .get_mut(bomb.pos_x)
                    .unwrap();
                if *top == 4 {
                    let _r = std::mem::replace(top, 0);
                    to_modify.push((
                        Position::new(bomb.pos_x, bomb.pos_y - 1, 2),
                        0)
                    );
                }
            }
            {
                let bottom = level_data
                    .tilemap
                    .get_mut(2)
                    .unwrap()
                    .values
                    .get_mut(bomb.pos_y + 1)
                    .unwrap()
                    .get_mut(bomb.pos_x)
                    .unwrap();
                if *bottom == 4 {
                    let _r = std::mem::replace(bottom, 0);
                    to_modify.push((
                        Position::new(bomb.pos_x, bomb.pos_y + 1, 2),
                        0)
                    );
                }
            }
        }
    }

    to_modify.drain(0..).for_each(|e| {
        Tilemap::modify_sprite_tile(world,refs.tilemap_entity.unwrap(), e.0, e.1);
    })
}
