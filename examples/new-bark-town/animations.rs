use std::{collections::HashMap, time::Duration};

use scion::{
    core::components::animations::{Animation, AnimationModifier},
    utils::maths::Vector,
};
use scion::core::components::color::Color;

pub const MOVE_DURATION: Duration = Duration::from_millis(250);
pub const FADE_DURATION: Duration = Duration::from_millis(100);

pub fn char_animations() -> HashMap<String, Animation> {
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
            AnimationModifier::transform(12, Some(Vector::new(48., 0.)), None, None),
            AnimationModifier::sprite(vec![9, 8, 9], 8),
        ],
    )
}

fn move_left() -> Animation {
    Animation::new(
        MOVE_DURATION,
        vec![
            AnimationModifier::transform(12, Some(Vector::new(-48., 0.)), None, None),
            AnimationModifier::sprite(vec![7,6,7], 6),
        ],
    )
}

fn move_top() -> Animation {
    Animation::new(
        MOVE_DURATION,
        vec![
            AnimationModifier::transform(12, Some(Vector::new(0., -48.)), None, None),
            AnimationModifier::sprite_with_variant(vec![3,4,5],vec![5,4, 3], 4),
        ],
    )
}

fn move_bottom() -> Animation {
    Animation::new(
        MOVE_DURATION,
        vec![
            AnimationModifier::transform(12, Some(Vector::new(0., 48.)), None, None),
            AnimationModifier::sprite_with_variant(vec![0,1,2],vec![2,1,0], 1),
        ],
    )
}

pub fn switch_scene_animation() -> HashMap<String, Animation> {
    let mut animations = HashMap::default();
    animations.insert("FADE_IN".to_string(), fade_in());
    animations.insert("FADE_OUT".to_string(), fade_out());
    animations
}


fn fade_in() -> Animation {
    Animation::new(
        FADE_DURATION,
        vec![
            AnimationModifier::color(10, Color::new(255,255,255,1.)),
        ],
    )
}

fn fade_out() -> Animation {
    Animation::running(
        FADE_DURATION,
        vec![
            AnimationModifier::color(10, Color::new(255,255,255,0.)),
        ],
    )
}