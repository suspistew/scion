use std::time::Duration;

use scion::graphics::components::animations::{Animation, AnimationModifier};

pub fn explode() -> Animation {
    Animation::new(
        Duration::from_millis(2000),
        vec![AnimationModifier::sprite(
            vec![
                64, 64, 64, 64, 64, 25, 25, 25, 25, 25, 38, 38, 38, 38, 38, 104, 105, 106, 107,
                108, 109, 110, 111, 112,
            ],
            12,
        )],
    )
}
