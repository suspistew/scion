use scion::core::world::{GameData, World};
use scion::core::{components::maths::transform::Transform, resources::inputs::types::KeyCode};

use crate::components::NextBloc;
use crate::{
    components::{Bloc, BlocKind, PieceKind, BLOC_SIZE, BOARD_HEIGHT, BOARD_WIDTH},
    resources::{TetrisResource, TetrisState},
    systems::piece_system::initialize_bloc,
};

pub fn piece_rotation_system(data: &mut GameData) {
    let (world, resources) = data.split();
    let mut timers = resources.timers();
    let mut tetris = resources.get_resource_mut::<TetrisResource>().unwrap();
    let inputs = resources.inputs();

    let rotation = inputs.key_pressed(&KeyCode::Up);
    let movement_timer = timers
        .get_timer("action_reset_timer")
        .expect("Missing a mandatory timer in the game : action_reset_timer");
    if movement_timer.ended() && rotation {
        let rotation_offsets = {
            let next_orientation = tetris.active_piece.orientation.next_orientation();
            PieceKind::get_offsets(&tetris.active_piece.kind, &next_orientation)
        };

        if let TetrisState::MOVING(x, y) = tetris.state {
            let mut should_rotate_piece = true;
            for offset in rotation_offsets.iter() {
                if x as f32 + offset.0 == 0.
                    || x as f32 + offset.0 >= (BOARD_WIDTH + 1) as f32
                    || y as f32 + offset.1 == (BOARD_HEIGHT + 1) as f32
                {
                    should_rotate_piece = false;
                } else {
                    for (_, (bloc, transform)) in world.query_mut::<(&mut Bloc, &mut Transform)>() {
                        match bloc.kind {
                            BlocKind::Moving => {}
                            _ => {
                                let translation = transform.translation();
                                if translation.x() / BLOC_SIZE == x as f32 + offset.0
                                    && translation.y() / BLOC_SIZE == y as f32 + offset.1
                                {
                                    should_rotate_piece = false;
                                    break;
                                }
                            }
                        }
                    }
                }
            }
            let mut to_remove = Vec::new();
            let mut to_add = Vec::new();

            if should_rotate_piece {
                for (entity, (bloc, _)) in world.query_mut::<(&mut Bloc, &mut Transform)>() {
                    match bloc.kind {
                        BlocKind::Moving => {
                            to_remove.push(entity);
                        }
                        _ => {}
                    }
                }
                tetris.active_piece.rotate();
                let offsets = tetris.active_piece.get_current_offsets();
                for offset in offsets {
                    to_add.push(initialize_bloc(&offset, &mut tetris, x as f32, y as f32, false));
                }
                movement_timer.reset();
            }
            to_remove.drain(0..).for_each(|e| {
                let _r = world.remove(e);
            });
            to_add.drain(0..).for_each(|comps| {
                if comps.3 {
                    let _r = world.push((comps.0, comps.1, comps.2, NextBloc));
                } else {
                    let _r = world.push((comps.0, comps.1, comps.2, Bloc::new(BlocKind::Moving)));
                }
            });
        }
    }
}
