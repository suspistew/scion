use legion::*;
use scion::core::resources::inputs::inputs_controller::InputsController;
use crate::{BombermanInfos, bomb_animations, BombermanRefs, Bomb};
use scion::core::components::animations::Animations;
use crate::level_reader::Level;
use scion::core::resources::inputs::keycode::KeyCode;
use legion::systems::CommandBuffer;
use scion::core::components::maths::transform::{Transform, Coordinates};
use scion::core::components::tiles::sprite::Sprite;

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
        inputs.keyboard_mut().on_key_pressed(KeyCode::Right, || {
            if level_data.pathing.get(posy).unwrap().get(posx + 1).unwrap() == &1
                && level_data.tilemap.get(2).unwrap().values.get(posy).unwrap().get(posx + 1).unwrap() == &0 {
                character.pos_x += 1;
                animations.run_animation("MOVE_RIGHT".to_string());
            }
        });
        inputs.keyboard_mut().on_key_pressed(KeyCode::Left, || {
            if level_data.pathing.get(posy).unwrap().get(posx - 1).unwrap() == &1
                && level_data.tilemap.get(2).unwrap().values.get(posy).unwrap().get(posx - 1).unwrap() == &0 {
                character.pos_x -= 1;
                animations.run_animation("MOVE_LEFT".to_string());
            }
        });
        inputs.keyboard_mut().on_key_pressed(KeyCode::Up, || {
            if level_data.pathing.get(posy - 1).unwrap().get(posx).unwrap() == &1
                && level_data.tilemap.get(2).unwrap().values.get(posy - 1).unwrap().get(posx).unwrap() == &0 {
                character.pos_y -= 1;
                animations.run_animation("MOVE_TOP".to_string());
            }
        });
        inputs.keyboard_mut().on_key_pressed(KeyCode::Down, || {
            if level_data.pathing.get(posy + 1).unwrap().get(posx).unwrap() == &1
                && level_data.tilemap.get(2).unwrap().values.get(posy + 1).unwrap().get(posx).unwrap() == &0 {
                character.pos_y += 1;
                animations.run_animation("MOVE_BOTTOM".to_string());
            }
        });
        inputs.keyboard_mut().on_key_pressed(KeyCode::Space, || {
            let mut animations = Animations::single("EXPLODE".to_string(), bomb_animations::explode());
            animations.run_animation("EXPLODE".to_string());
            cmd.push(
                (Transform::new(Coordinates::new_with_layer((character.pos_x * 64) as f32, (character.pos_y * 64) as f32, level_data.tilemap.len() + 1), 1., 0.),
                 animations,
                 Sprite::new(64),
                 refs.tileset.as_ref().unwrap().clone(),
                 Bomb{ pos_x: character.pos_x, pos_y: character.pos_y }
                )
            );
        });
    }
}