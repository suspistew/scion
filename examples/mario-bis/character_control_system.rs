use scion::core::world::{GameData, World};
use scion::core::{
    components::maths::transform::Transform,
    resources::{
        inputs::{inputs_controller::InputsController, types::KeyCode},
        time::TimerType,
    },
};

use crate::model::Hero;

pub fn move_char_system(data: &mut GameData) {
    let (world, resources) = data.split();
    let mut timers = resources.timers();
    let inputs = resources.inputs();

    if !timers.exists("input") {
        let _r = timers.add_timer("input", TimerType::Manual, 0.05);
    }
    if !timers.exists("gravity") {
        let _r = timers.add_timer("gravity", TimerType::Manual, 0.005);
    }

    let input_velocity = read_velocity(&inputs);
    let jump = inputs.key_pressed(&KeyCode::Up);

    if input_velocity != 0 {
        let timer = timers.get_timer("input").expect("Missing timer : input");
        for (_, (hero, _)) in world.query_mut::<(&mut Hero, &mut Transform)>() {
            if timer.ended()
                && ((input_velocity > 0 && hero.velocity < 28)
                    || (input_velocity < 0 && hero.velocity > -1 * 28))
            {
                hero.velocity += input_velocity * 40;
                hero.landed = false;
                timer.reset();
            }
        }
    } else {
        let timer = timers.get_timer("input").expect("Missing timer : input");
        for (_, (hero, _)) in world.query_mut::<(&mut Hero, &mut Transform)>() {
            if timer.ended() && hero.velocity != 0 {
                hero.landed = false;
                if hero.velocity.abs() < 12{
                    hero.velocity = 0;
                }else{
                    hero.velocity += {
                        if hero.velocity > 0 {
                            -12
                        } else {
                            12
                        }
                    };
                }
                timer.reset();
            }
        }
    }

    if jump {
        for (_, (hero, _)) in world.query_mut::<(&mut Hero, &mut Transform)>() {
            if hero.landed {
                hero.gravity -= 30;
                hero.landed = false;
            }
        }
    } else {
        let timer = timers.get_timer("gravity").expect("Missing timer : gravity");
        for (_, (hero, _)) in world.query_mut::<(&mut Hero, &mut Transform)>() {
            if !hero.landed {
                if timer.ended() {
                    timer.reset();
                    hero.gravity += 4;
                }
            } else {
                hero.gravity = 0;
            }
        }
    }
}

fn read_velocity(inputs: &InputsController) -> i32 {
    ({
        if inputs.key_pressed(&KeyCode::Left) {
            -1
        } else {
            0
        }
    }) + ({
        if inputs.key_pressed(&KeyCode::Right) {
            1
        } else {
            0
        }
    })
}
