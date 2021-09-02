use legion::*;
use rand::{thread_rng, Rng};
use scion::core::{
    components::{
        animations::Animations,
        maths::{
            collider::{Collider, ColliderMask},
            transform::Transform,
        },
    },
    resources::sound::{AudioPlayer, PlayConfig},
};

use crate::main_layer::{Ball, BallDirection};

#[system(for_each)]
pub fn ball_control(
    ball: &mut Ball,
    transform: &mut Transform,
    collider: &Collider,
    animations: &mut Animations,
    #[resource] audio_player: &mut AudioPlayer,
) {
    // Init the ball direction
    if ball.direction.is_none() {
        let direction = random_direction();
        animations.loop_animation(direction.to_string().as_str());
        ball.direction = Some(direction);
    }

    collider.collisions().iter().for_each(|collision| {
        if let ColliderMask::Custom(name) = collision.mask() {
            let new_direction = match (
                name.as_str(),
                &ball.direction.as_ref().expect("A ball has collided without any direction WTF ?!"),
            ) {
                ("BORDER_LEFT", BallDirection::TOP_LEFT) => Some(BallDirection::TOP_RIGHT),
                ("BORDER_LEFT", BallDirection::BOTTOM_LEFT) => Some(BallDirection::BOTTOM_RIGHT),
                ("BORDER_RIGHT", BallDirection::TOP_RIGHT) => Some(BallDirection::TOP_LEFT),
                ("BORDER_RIGHT", BallDirection::BOTTOM_RIGHT) => Some(BallDirection::BOTTOM_LEFT),
                ("BORDER_TOP", BallDirection::TOP_RIGHT) => Some(BallDirection::BOTTOM_RIGHT),
                ("BORDER_TOP", BallDirection::TOP_LEFT) => Some(BallDirection::BOTTOM_LEFT),
                ("BORDER_BOTTOM", BallDirection::BOTTOM_RIGHT) => Some(BallDirection::TOP_RIGHT),
                ("BORDER_BOTTOM", BallDirection::BOTTOM_LEFT) => Some(BallDirection::TOP_LEFT),
                ("BORDER_CUSTOM_VERTICAL", e) => {
                    match e {
                        BallDirection::TOP_LEFT => {
                            if collision.coordinates().x() + 8. <= transform.translation().x() {
                                Some(BallDirection::TOP_RIGHT)
                            } else {
                                None
                            }
                        }
                        BallDirection::TOP_RIGHT => {
                            if collision.coordinates().x() > transform.translation().x() {
                                Some(BallDirection::TOP_LEFT)
                            } else {
                                None
                            }
                        }
                        BallDirection::BOTTOM_LEFT => {
                            if collision.coordinates().x() + 8. <= transform.translation().x() {
                                Some(BallDirection::BOTTOM_RIGHT)
                            } else {
                                None
                            }
                        }
                        BallDirection::BOTTOM_RIGHT => {
                            if collision.coordinates().x() > transform.translation().x() {
                                Some(BallDirection::BOTTOM_LEFT)
                            } else {
                                None
                            }
                        }
                    }
                }
                ("BORDER_CUSTOM_HORIZONTAL", e) => {
                    match e {
                        BallDirection::TOP_LEFT => {
                            if collision.coordinates().y() + 8. <= transform.translation().y() {
                                Some(BallDirection::BOTTOM_LEFT)
                            } else {
                                None
                            }
                        }
                        BallDirection::TOP_RIGHT => {
                            if collision.coordinates().y() + 8. <= transform.translation().y() {
                                Some(BallDirection::BOTTOM_RIGHT)
                            } else {
                                None
                            }
                        }
                        BallDirection::BOTTOM_LEFT => {
                            if collision.coordinates().y() > transform.translation().y() {
                                Some(BallDirection::TOP_LEFT)
                            } else {
                                None
                            }
                        }
                        BallDirection::BOTTOM_RIGHT => {
                            if collision.coordinates().y() > transform.translation().y() {
                                Some(BallDirection::TOP_RIGHT)
                            } else {
                                None
                            }
                        }
                    }
                }
                (_a, _b) => None,
            };

            if let Some(direction) = new_direction {
                animations.stop_all_animation(true);
                animations.loop_animation(direction.to_string().as_str());
                ball.direction = Some(direction);
                //audio_player.play("BOUNCE", PlayConfig::default());
            }
        }
    })
}

fn random_direction() -> BallDirection {
    match thread_rng().gen_range(0..4) {
        0 => BallDirection::TOP_RIGHT,
        1 => BallDirection::TOP_LEFT,
        2 => BallDirection::BOTTOM_LEFT,
        _ => BallDirection::BOTTOM_RIGHT,
    }
}
