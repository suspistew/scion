use std::collections::HashMap;

use scion::{
    core::components::maths::transform::Transform,
    legion::{system, systems::CommandBuffer, world::SubWorld, Entity, Query},
};

use crate::{
    components::{Bloc, BlocKind, BLOC_SIZE, BOARD_HEIGHT},
    resources::TetrisResource,
};

#[system]
pub fn score(
    cmd: &mut CommandBuffer,
    #[resource] tetris: &mut TetrisResource,
    world: &mut SubWorld,
    query: &mut Query<(Entity, &Bloc, &mut Transform)>,
) {
    let mut lines = HashMap::new();
    for i in 1..=BOARD_HEIGHT {
        lines.insert(i as usize, 0);
    }
    for (_, bloc, transform) in query.iter_mut(world) {
        match bloc.kind {
            BlocKind::Static => {
                let line_idx = (transform.translation().y() / BLOC_SIZE) as usize;
                let bloc_counter = match lines.get(&line_idx) {
                    Some(val) => val + 1,
                    None => 1,
                };
                lines.insert(line_idx, bloc_counter);
            }
            _ => {}
        }
    }

    let full_lines = {
        let mut full_lines = Vec::new();
        for (line_idx, bloc_counter) in lines.iter() {
            if bloc_counter == &10 {
                tetris.score += 1;
                full_lines.push(*line_idx);
            }
        }
        full_lines.sort_unstable();
        full_lines
    };

    if !full_lines.is_empty() {
        for (entity, bloc, transform) in query.iter_mut(world) {
            match bloc.kind {
                BlocKind::Static => {
                    let line_idx = (transform.translation().y() / BLOC_SIZE) as usize;
                    if full_lines.contains(&line_idx) {
                        cmd.remove(*entity);
                    }
                }
                _ => {}
            };
        }
        for full_line_idx in full_lines.iter() {
            for (_, bloc, transform) in query.iter_mut(world) {
                match bloc.kind {
                    BlocKind::Static => {
                        let line_idx = (transform.translation().y() / BLOC_SIZE) as usize;
                        if *full_line_idx > line_idx {
                            transform.move_down(BLOC_SIZE);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}
