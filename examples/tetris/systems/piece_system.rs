use scion::{
    core::{
        components::{
            material::Material2D,
            maths::{
                transform::{Coordinates, Transform},
            },
            Square,
        },
        resources::time::Timers,
    },
    legion::{system, systems::CommandBuffer, world::SubWorld, Entity, Query},
};

use crate::{
    asset_path,
    components::{Bloc, BlocKind, NextBloc, BLOC_SIZE, BOARD_HEIGHT, BOARD_OFFSET},
    resources::{TetrisResource, TetrisState},
};

#[system]
pub fn piece_update(
    cmd: &mut CommandBuffer,
    #[resource] timers: &mut Timers,
    #[resource] tetris: &mut TetrisResource,
    world: &mut SubWorld,
    query: &mut Query<(&mut Bloc, &mut Transform)>,
    query2: &mut Query<(Entity, &NextBloc)>,
) {
    let timer = timers
        .get_timer("piece")
        .expect("Missing a mandatory timer in the game : piece timer");
    if timer.cycle() {
        match tetris.state {
            TetrisState::WAITING => {
                tetris.switch_to_next_piece();
                query2.for_each(world, |(e, _)| {
                    cmd.remove(*e);
                });
                let offsets = tetris.next_piece.get_current_offsets();
                for offset in offsets {
                    initialize_next_bloc(&offset, cmd, tetris.next_piece.color, 12., 2.);
                }

                let offsets = tetris.active_piece.get_current_offsets();
                for offset in offsets {
                    initialize_bloc(&offset, cmd, tetris.active_piece.color, 4., 0.);
                }
            }
            TetrisState::MOVING(x, y) => {
                let mut static_values: Vec<(u32, u32)> = Vec::new();
                let mut piece_values: Vec<(u32, u32)> = Vec::new();
                for (bloc, transform) in query.iter_mut(world) {
                    let t = (
                        ((transform.translation().x() - BOARD_OFFSET.0) / BLOC_SIZE) as u32,
                        ((transform.translation().y() - BOARD_OFFSET.1) / BLOC_SIZE) as u32,
                    );
                    match bloc.kind {
                        BlocKind::Moving => piece_values.push(t),
                        _ => static_values.push(t),
                    };
                }
                let should_move_piece = {
                    let mut res = true;
                    for (x, y) in piece_values.iter() {
                        for (xx, yy) in static_values.iter() {
                            if x == xx && y == &(yy - 1) {
                                res = false;
                            }
                        }
                        if y == &(BOARD_HEIGHT - 1) {
                            res = false;
                        }
                    }
                    res
                };
                if should_move_piece {
                    for (bloc, transform) in query.iter_mut(world) {
                        match bloc.kind {
                            BlocKind::Moving => {
                                transform.move_down(BLOC_SIZE);
                                tetris.state = TetrisState::MOVING(x, y + 1);
                            }
                            _ => {}
                        };
                    }
                } else {
                    for (mut bloc, _) in query.iter_mut(world) {
                        match bloc.kind {
                            BlocKind::Moving => {
                                bloc.kind = BlocKind::Static;
                                tetris.state = TetrisState::WAITING;
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
}

pub fn initialize_bloc(
    offset: &(f32, f32),
    cmd: &mut CommandBuffer,
    color: usize,
    coord_x: f32,
    coord_y: f32,
) {
    let mut bloc_transform = Transform::default();
    bloc_transform.append_translation(
        coord_x * BLOC_SIZE + offset.0 * BLOC_SIZE,
        coord_y * BLOC_SIZE + offset.1 * BLOC_SIZE,
    );
    bloc_transform.set_layer(1);
    cmd.push((
        Bloc::new(BlocKind::Moving),
        bloc_transform,
        Square::new(
            32.,
            Some([
                Coordinates::new(0., 0.),
                Coordinates::new(0., 1.),
                Coordinates::new(1., 1.),
                Coordinates::new(1., 0.),
            ]),
        ),
        Material2D::Texture(
            asset_path()
                .join(get_color_skin(color).as_str())
                .as_path()
                .to_str()
                .unwrap()
                .to_string(),
        ),
    ));
}

pub fn initialize_next_bloc(
    offset: &(f32, f32),
    cmd: &mut CommandBuffer,
    color: usize,
    coord_x: f32,
    coord_y: f32,
) {
    let mut bloc_transform = Transform::default();
    bloc_transform.append_translation(
        coord_x * BLOC_SIZE + offset.0 * BLOC_SIZE,
        coord_y * BLOC_SIZE + offset.1 * BLOC_SIZE,
    );
    bloc_transform.set_layer(1);
    cmd.push((
        NextBloc,
        bloc_transform,
        Square::new(
            32.,
            Some([
                Coordinates::new(0., 0.),
                Coordinates::new(0., 1.),
                Coordinates::new(1., 1.),
                Coordinates::new(1., 0.),
            ]),
        ),
        Material2D::Texture(
            asset_path()
                .join(get_color_skin(color).as_str())
                .as_path()
                .to_str()
                .unwrap()
                .to_string(),
        ),
    ));
}

fn get_color_skin(color: usize) -> String {
    let color = match color {
        0 => "blue.png",
        1 => "brown.png",
        2 => "cyan.png",
        3 => "green.png",
        4 => "pink.png",
        5 => "red.png",
        _ => "yellow.png",
    };
    color.to_string()
}
