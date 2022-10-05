use std::collections::HashMap;
use std::time::Duration;
use std::vec;
use scion::core::components::animations::{Animation, AnimationModifier};
use scion::utils::maths::Vector;
use crate::MoveDirection;

pub(crate) fn get_case_animation() -> HashMap<String, Animation>{
    let mut animations = HashMap::new();
    animations.insert("LEFT".to_string(),move_direction(MoveDirection::Left));
    animations.insert("RIGHT".to_string(),move_direction(MoveDirection::Right));
    animations.insert("TOP".to_string(),move_direction(MoveDirection::Top));
    animations.insert("BOTTOM".to_string(),move_direction(MoveDirection::Bottom));
    animations
}

fn move_direction(direction: MoveDirection) -> Animation {
    Animation::new(Duration::from_millis(100),
                   vec![AnimationModifier::transform(8,
                                                     retrieve_vector(direction),
                                                     None,
                                                     None)])
}

fn retrieve_vector(direction: MoveDirection) -> Option<Vector> {
    match direction {
        MoveDirection::Left => Some(Vector::new( -192., 0.)),
        MoveDirection::Right => Some(Vector::new(192., 0.)),
        MoveDirection::Top => Some(Vector::new(0., -192.)),
        MoveDirection::Bottom => Some(Vector::new(0., 192.)),
        _ => None
    }
}