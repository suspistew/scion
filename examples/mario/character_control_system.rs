use scion::{
    core::{
        components::maths::transform::Transform,
        resources::{
            inputs::{inputs_controller::InputsController, keycode::KeyCode, InputState},
            time::{TimerType, Timers},
        },
    },
    legion::{world::SubWorld, *},
};

use crate::{Hero, MAX_VELOCITY};

#[system]
pub fn move_char(
    #[resource] inputs: &InputsController,
    #[resource] timers: &mut Timers,
    world: &mut SubWorld,
    query: &mut Query<(&mut Hero, &mut Transform)>,
) {
    if !timers.exists("input") {
        timers.add_timer("input", TimerType::Manual, 0.05);
    }
    if !timers.exists("gravity") {
        timers.add_timer("gravity", TimerType::Manual, 0.005);
    }

    let input_velocity = read_velocity(inputs);
    let jump = inputs
        .keyboard()
        .keyboard_events()
        .iter()
        .filter(|e| e.state.eq(&InputState::Pressed) && e.keycode.eq(&KeyCode::Up))
        .count()
        > 0;

    if input_velocity != 0 {
        let timer = timers.get_timer("input").expect("Missing timer : input");
        query.for_each_mut(world, |(hero, _t)| {
            if timer.ended()
                && ((input_velocity > 0 && hero.velocity < MAX_VELOCITY)
                    || (input_velocity < 0 && hero.velocity > -1 * MAX_VELOCITY))
            {
                hero.velocity += input_velocity * 12;
                hero.landed = false;
                timer.reset();
            }
        });
    } else {
        let timer = timers.get_timer("input").expect("Missing timer : input");
        query.for_each_mut(world, |(hero, _t)| {
            if timer.ended() && hero.velocity != 0 {
                hero.landed = false;
                hero.velocity += {
                    if hero.velocity > 0 {
                        -12
                    } else {
                        12
                    }
                };
                timer.reset();
            }
        });
    }

    if jump {
        query.for_each_mut(world, |(hero, _t)| {
            if hero.landed {
                hero.gravity -= 50;
                hero.landed = false;
            }
        });
    } else {
        let timer = timers.get_timer("gravity").expect("Missing timer : gravity");
        query.for_each_mut(world, |(hero, _t)| {
            if !hero.landed {
                if timer.ended() {
                    timer.reset();
                    hero.gravity += 4;
                }
            } else {
                hero.gravity = 0;
            }
        });
    }
}

fn read_velocity(inputs: &InputsController) -> i32 {
    ({
        if inputs.keyboard().key_pressed(&KeyCode::Left) {
            -1
        } else {
            0
        }
    }) + ({
        if inputs.keyboard().key_pressed(&KeyCode::Right) {
            1
        } else {
            0
        }
    })
}
