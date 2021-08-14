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
                let key = (transform.translation().y() / BLOC_SIZE) as usize;
                let new_val = match lines.get(&key) {
                    Some(val) => val + 1,
                    None => 1,
                };
                lines.insert(key, new_val);
            }
            _ => {}
        }
    }

    let lines2 = {
        let mut full_lines = Vec::new();
        for (key, val) in lines.iter() {
            if val == &10 {
                tetris.score += 1;
                full_lines.push(*key);
            }
        }
        full_lines.sort_unstable();
        full_lines
    };

    if !lines2.is_empty() {
        for (entity, bloc, transform) in query.iter_mut(world) {
            match bloc.kind {
                BlocKind::Static => {
                    let line = (transform.translation().y() / BLOC_SIZE) as usize;
                    if lines2.contains(&line) {
                        cmd.remove(*entity);
                    }
                }
                _ => {}
            };
        }
        for (index, line) in lines2.iter().enumerate() {
            for (_, bloc, transform) in query.iter_mut(world) {
                match bloc.kind {
                    BlocKind::Static => {
                        if (*line - index as usize)
                            > (transform.translation().y() / BLOC_SIZE) as usize
                        {
                            transform.move_down(BLOC_SIZE);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}
