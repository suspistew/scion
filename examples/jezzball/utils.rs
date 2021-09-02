use std::{collections::HashMap, time::Duration};

use scion::{
    core::components::{
        animations::{Animation, AnimationModifier, Animations},
        maths::vector::Vector,
    },
    utils::file::app_base_path,
};

const ANIMATION_DURATION: Duration = Duration::from_millis(400);
const BALL_SPEED: f32 = 75.;

pub fn ball_asset() -> String { app_base_path().join("examples/jezzball/assets/ball.png").get() }

pub fn cases_asset() -> String { app_base_path().join("examples/jezzball/assets/cases.png").get() }

pub fn ball_bounce_effect() -> String {
    app_base_path().join("examples/jezzball/assets/bounce.ogg").get()
}

pub fn ball_animations() -> Animations {
    let mut animations = HashMap::new();
    animations.insert(
        "TOP_LEFT".to_string(),
        Animation::new(
            ANIMATION_DURATION,
            vec![
                AnimationModifier::sprite(vec![10,9,8,3], 3),
                AnimationModifier::transform(
                    15,
                    Some(Vector::new(-BALL_SPEED, -BALL_SPEED)),
                    None,
                    None,
                ),
            ],
        ),
    );
    animations.insert(
        "BOTTOM_LEFT".to_string(),
        Animation::new(
            ANIMATION_DURATION,
            vec![
                AnimationModifier::sprite(vec![12,13,14,3], 3),
                AnimationModifier::transform(
                    15,
                    Some(Vector::new(-BALL_SPEED, BALL_SPEED)),
                    None,
                    None,
                ),
            ],
        ),
    );
    animations.insert(
        "TOP_RIGHT".to_string(),
        Animation::new(
            ANIMATION_DURATION,
            vec![
                AnimationModifier::sprite(vec![14,13,12,3], 3),
                AnimationModifier::transform(
                    15,
                    Some(Vector::new(BALL_SPEED, -BALL_SPEED)),
                    None,
                    None,
                ),
            ],
        ),
    );
    animations.insert(
        "BOTTOM_RIGHT".to_string(),
        Animation::new(
            ANIMATION_DURATION,
            vec![
                AnimationModifier::sprite(vec![8,9,10,3], 3),
                AnimationModifier::transform(
                    15,
                    Some(Vector::new(BALL_SPEED, BALL_SPEED)),
                    None,
                    None,
                ),
            ],
        ),
    );
    Animations::new(animations)
}
