use std::{collections::HashMap, time::Duration};

use scion::core::components::{
    animations::{Animation, AnimationModifier},
    maths::vector::Vector,
};

const MOVE_DURATION: Duration = Duration::from_millis(500);

pub fn get_animations() -> HashMap<String, Animation> {
    let mut animations = HashMap::default();
    animations.insert("MOVE_RIGHT".to_string(), move_right());
    animations.insert("MOVE_LEFT".to_string(), move_left());
    animations.insert("MOVE_TOP".to_string(), move_top());
    animations.insert("MOVE_BOTTOM".to_string(), move_bottom());
    animations
}

fn move_right() -> Animation {
    Animation::new(
        MOVE_DURATION,
        vec![
            AnimationModifier::transform(30, Some(Vector::new(64., 0.)), None, None),
            AnimationModifier::sprite(vec![78, 79, 80, 79, 78, 79, 80, 79], 78),
        ],
        false,
    )
}

fn move_left() -> Animation {
    Animation::new(
        MOVE_DURATION,
        vec![
            AnimationModifier::transform(30, Some(Vector::new(-64., 0.)), None, None),
            AnimationModifier::sprite(vec![81, 82, 83, 82, 81, 82, 83, 82], 81),
        ],
        false,
    )
}

fn move_top() -> Animation {
    Animation::new(
        MOVE_DURATION,
        vec![
            AnimationModifier::transform(30, Some(Vector::new(0., -64.)), None, None),
            AnimationModifier::sprite(vec![55, 56, 57, 56, 55, 56, 57, 56], 55),
        ],
        false,
    )
}

fn move_bottom() -> Animation {
    Animation::new(
        MOVE_DURATION,
        vec![
            AnimationModifier::transform(30, Some(Vector::new(0., 64.)), None, None),
            AnimationModifier::sprite(vec![52, 53, 54, 53, 52, 53, 54, 53], 52),
        ],
        false,
    )
}
