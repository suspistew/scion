use std::{
    collections::HashMap,
    fmt,
    fmt::{Display, Formatter},
    ops::Div,
    time::Duration,
};

use crate::core::components::maths::transform::Coordinates;

pub struct Animations {
    animations: HashMap<String, Animation>,
}

impl Animations {
    /// Creates a new Animations component
    pub fn new(animations: HashMap<String, Animation>) -> Self {
        Animations { animations }
    }

    fn run(&mut self, animation_name: String, status: AnimationStatus) -> bool {
        if self.animations.contains_key(animation_name.as_str()) {
            let mut animation = self
                .animations
                .get_mut(animation_name.as_str())
                .expect("An animation has not been found after the security check");
            if AnimationStatus::STOPPED == animation.status {
                animation.status = status;
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Runs the animation `name`. Returns true is the animation has been started, false if it does not exist or was already running
    pub fn run_animation(&mut self, animation_name: String) -> bool {
        self.run(animation_name, AnimationStatus::RUNNING)
    }

    /// Runs the animation `name`. Returns true is the animation has been started, false if it does not exist or was already running
    pub fn loop_animation(&mut self, animation_name: String) -> bool {
        self.run(animation_name, AnimationStatus::LOOPING)
    }

    /// Stops the animation `name`. Returns true is the animation has been stopped, false if it does not exist or was already stopped
    pub fn stop_animation(&mut self, animation_name: String, force: bool) -> bool {
        if self.animations.contains_key(animation_name.as_str()) {
            let mut animation = self
                .animations
                .get_mut(animation_name.as_str())
                .expect("An animation has not been found after the security check");
            if animation.status == AnimationStatus::LOOPING
                || animation.status == AnimationStatus::RUNNING
            {
                if force {
                    animation.status = AnimationStatus::STOPPED;
                } else {
                    animation.status = AnimationStatus::STOPPING;
                }
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Returns the mutable animations
    pub fn animations_mut(&mut self) -> &mut HashMap<String, Animation> {
        &mut self.animations
    }
}

#[derive(Eq, PartialEq)]
pub(crate) enum AnimationStatus {
    STOPPED,
    RUNNING,
    LOOPING,
    STOPPING,
}

pub struct Animation {
    pub(crate) _duration: Duration,
    pub(crate) modifiers: Vec<AnimationModifier>,
    pub(crate) status: AnimationStatus,
}

impl Animation {
    /// Creates a new animation based on a duration and a list of modifiers
    pub fn new(duration: Duration, mut modifiers: Vec<AnimationModifier>) -> Self {
        if duration.as_millis() != 0 {
            modifiers.iter_mut().for_each(|animation_modifier| {
                animation_modifier.single_keyframe_duration =
                    Some(duration.div(animation_modifier.number_of_keyframes as u32));
                compute_animation_keyframe_modifier(animation_modifier);
            });
        }

        Self {
            _duration: duration,
            modifiers,
            status: AnimationStatus::STOPPED,
        }
    }

    /// Will compute the status of the current animation
    pub(crate) fn try_update_status(&mut self) {
        if self
            .modifiers
            .iter()
            .filter(|modifier| modifier.current_keyframe == modifier.number_of_keyframes)
            .count()
            == self.modifiers.len()
        {
            self.modifiers
                .iter_mut()
                .for_each(|modifier| modifier.current_keyframe = 0);
            if self.status == AnimationStatus::RUNNING || self.status == AnimationStatus::STOPPING {
                self.status = AnimationStatus::STOPPED;
            }
        }
    }
}

pub struct AnimationModifier {
    pub(crate) number_of_keyframes: usize,
    pub(crate) current_keyframe: usize,
    pub(crate) modifier_type: AnimationModifierType,
    pub(crate) single_keyframe_duration: Option<Duration>,
    pub(crate) single_keyframe_modifier: Option<AnimationModifierType>,
    /// In case of a sprite modifier we need to keep track of the next index position in the vec
    pub(crate) next_sprite_index: Option<usize>,
}

impl AnimationModifier {
    /// Creates a new AnimationModifier using a number of keyframes and a type.
    fn new(number_of_keyframes: usize, modifier_type: AnimationModifierType) -> Self {
        Self {
            number_of_keyframes,
            current_keyframe: 0,
            modifier_type,
            single_keyframe_duration: None,
            single_keyframe_modifier: None,
            next_sprite_index: None,
        }
    }

    /// Convenience function to directly create an AnimationModifier of type Transform with the needed informations
    pub fn transform(
        number_of_keyframes: usize,
        vector: Option<Coordinates>,
        scale: Option<f32>,
        rotation: Option<f32>,
    ) -> Self {
        AnimationModifier::new(
            number_of_keyframes,
            AnimationModifierType::TransformModifier {
                vector,
                scale,
                rotation,
            },
        )
    }
    /// Convenience function to directly create an AnimationModifier of type Sprite with the needed informations
    pub fn sprite(tile_numbers: Vec<usize>, end_tile_number: usize) -> Self {
        AnimationModifier::new(
            tile_numbers.len() - 1,
            AnimationModifierType::SpriteModifier {
                tile_numbers,
                end_tile_number,
            },
        )
    }
}

#[derive(Debug)]
pub enum AnimationModifierType {
    TransformModifier {
        vector: Option<Coordinates>,
        scale: Option<f32>,
        rotation: Option<f32>,
    },
    SpriteModifier {
        tile_numbers: Vec<usize>,
        end_tile_number: usize,
    },
}

impl Display for AnimationModifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AnimationModifier-{}",
            match self.modifier_type {
                AnimationModifierType::TransformModifier { .. } => {
                    "TransformModifier"
                }
                AnimationModifierType::SpriteModifier { .. } => {
                    "SpriteModifier"
                }
            }
        )
    }
}

fn compute_animation_keyframe_modifier(modifier: &mut AnimationModifier) {
    let keyframe_nb = modifier.number_of_keyframes as f32;
    modifier.single_keyframe_modifier = match modifier.modifier_type {
        AnimationModifierType::TransformModifier {
            vector: coordinates,
            scale,
            rotation,
        } => {
            Some(AnimationModifierType::TransformModifier {
                vector: coordinates.map_or(None, |coordinates| {
                    Some(Coordinates::new(
                        coordinates.x() / keyframe_nb,
                        coordinates.y() / keyframe_nb,
                    ))
                }),
                scale: scale.map_or(None, |scale| Some(scale / keyframe_nb)),
                rotation: rotation.map_or(None, |rotation| Some(rotation / keyframe_nb)),
            })
        }
        AnimationModifierType::SpriteModifier { .. } => None,
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duration_divider_test() {
        let animation = Animation::new(
            Duration::from_secs(1),
            vec![AnimationModifier::new(
                2,
                AnimationModifierType::TransformModifier {
                    vector: Some(Coordinates::new(2., 4.)),
                    scale: Some(4.),
                    rotation: Some(1.),
                },
            )],
        );

        let anim_modifier = animation.modifiers.iter().next().unwrap();
        assert_eq!(
            500,
            anim_modifier.single_keyframe_duration.unwrap().as_millis()
        );
        if let AnimationModifierType::TransformModifier {
            vector: coordinates,
            scale,
            rotation,
        } = anim_modifier.single_keyframe_modifier.as_ref().unwrap()
        {
            assert_eq!(1.0, coordinates.unwrap().x());
            assert_eq!(2.0, coordinates.unwrap().y());
            assert_eq!(2.0, scale.unwrap());
            assert_eq!(0.5, rotation.unwrap());
        } else {
            panic!();
        }
    }
}
