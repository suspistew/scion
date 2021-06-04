use scion::{
    core::{
        components::maths::transform::Transform,
        resources::{
            inputs::{inputs_controller::InputsController, keycode::KeyCode},
            time::Timers,
        },
    },
    legion::{system, systems::CommandBuffer, world::SubWorld, Entity, Query},
};

use crate::{
    components::{Bloc, BlocKind, PieceKind, BLOC_SIZE, BOARD_HEIGHT, BOARD_WIDTH},
    resources::{TetrisResource, TetrisState},
    systems::piece_system::initialize_bloc,
};

#[system]
pub fn piece_rotation(
    cmd: &mut CommandBuffer,
    #[resource] inputs: &InputsController,
    #[resource] timers: &mut Timers,
    #[resource] tetris: &mut TetrisResource,
    world: &mut SubWorld,
    query: &mut Query<(Entity, &mut Bloc, &mut Transform)>,
) {
    let rotation = inputs.keyboard().key_pressed(&KeyCode::Up);
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
                    || x as f32 + offset.0 == (BOARD_WIDTH + 1) as f32
                    || y as f32 + offset.1 == (BOARD_HEIGHT + 1) as f32
                {
                    should_rotate_piece = false;
                } else {
                    for (_, bloc, transform) in query.iter_mut(world) {
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
            if should_rotate_piece {
                for (entity, bloc, _transform) in query.iter_mut(world) {
                    match bloc.kind {
                        BlocKind::Moving => {
                            cmd.remove(*entity);
                        }
                        _ => {}
                    }
                }
                tetris.active_piece.rotate();
                let offsets = tetris.active_piece.get_current_offsets();
                for offset in offsets {
                    initialize_bloc(&offset, cmd, tetris, x as f32, y as f32, false);
                }
                movement_timer.reset();
            }
        }
    }
}
