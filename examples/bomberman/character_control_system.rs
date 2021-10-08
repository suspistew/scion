use legion::{systems::CommandBuffer, *};
use scion::core::{
    components::{animations::Animations, maths::transform::Transform, tiles::sprite::Sprite},
    resources::inputs::{inputs_controller::InputsController, types::KeyCode},
};

use crate::{bomb_animations, level_reader::Level, Bomb, BombermanInfos, BombermanRefs};

#[system(for_each)]
pub fn controller(
    cmd: &mut CommandBuffer,
    #[resource] inputs: &mut InputsController,
    #[resource] refs: &mut BombermanRefs,
    #[resource] level_data: &Level,
    character: &mut BombermanInfos,
    animations: &mut Animations,
) {
    let (posx, posy) = (character.pos_x, character.pos_y);

    if !animations.any_animation_running() {
        inputs.on_key_pressed(KeyCode::Right, || {
            if level_data.pathing.get(posy).unwrap().get(posx + 1).unwrap() == &1
                && level_data
                    .tilemap
                    .get(2)
                    .unwrap()
                    .values
                    .get(posy)
                    .unwrap()
                    .get(posx + 1)
                    .unwrap()
                    == &0
            {
                character.pos_x += 1;
                animations.run_animation("MOVE_RIGHT");
            }
        });
        inputs.on_key_pressed(KeyCode::Left, || {
            if level_data.pathing.get(posy).unwrap().get(posx - 1).unwrap() == &1
                && level_data
                    .tilemap
                    .get(2)
                    .unwrap()
                    .values
                    .get(posy)
                    .unwrap()
                    .get(posx - 1)
                    .unwrap()
                    == &0
            {
                character.pos_x -= 1;
                animations.run_animation("MOVE_LEFT");
            }
        });
        inputs.on_key_pressed(KeyCode::Up, || {
            if level_data.pathing.get(posy - 1).unwrap().get(posx).unwrap() == &1
                && level_data
                    .tilemap
                    .get(2)
                    .unwrap()
                    .values
                    .get(posy - 1)
                    .unwrap()
                    .get(posx)
                    .unwrap()
                    == &0
            {
                character.pos_y -= 1;
                animations.run_animation("MOVE_TOP");
            }
        });
        inputs.on_key_pressed(KeyCode::Down, || {
            if level_data.pathing.get(posy + 1).unwrap().get(posx).unwrap() == &1
                && level_data
                    .tilemap
                    .get(2)
                    .unwrap()
                    .values
                    .get(posy + 1)
                    .unwrap()
                    .get(posx)
                    .unwrap()
                    == &0
            {
                character.pos_y += 1;
                animations.run_animation("MOVE_BOTTOM");
            }
        });
        inputs.on_key_pressed(KeyCode::Space, || {
            let mut animations = Animations::single("EXPLODE", bomb_animations::explode());
            animations.run_animation("EXPLODE");
            cmd.push((
                Transform::from_xyz(
                    (character.pos_x * 64) as f32,
                    (character.pos_y * 64) as f32,
                    level_data.tilemap.len() + 1,
                ),
                animations,
                Sprite::new(64),
                refs.tileset.as_ref().unwrap().clone(),
                Bomb { pos_x: character.pos_x, pos_y: character.pos_y },
            ));
        });
    }
}
