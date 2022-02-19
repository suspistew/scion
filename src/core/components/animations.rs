use std::{
    collections::HashMap,
    fmt,
    fmt::{Display, Formatter},
    ops::Div,
    time::Duration,
};

use crate::{core::components::color::Color, utils::maths::Vector};
use crate::core::components::animations::AnimationStatus::{FORCE_STOPPED, STOPPED};

pub struct Animations {
    animations: HashMap<String, Animation>,
}

impl Animations {
    /// Creates a new Animations component
    pub fn new(animations: HashMap<String, Animation>) -> Self { Animations { animations } }

    /// Create a new Animations component with a single animation provided
    pub fn single(name: &str, animation: Animation) -> Self {
        let mut animations = HashMap::new();
        animations.insert(name.to_string(), animation);
        Animations { animations }
    }

    fn run(&mut self, animation_name: &str, status: AnimationStatus) -> bool {
        if self.animations.contains_key(animation_name) {
            let mut animation = self
                .animations
                .get_mut(animation_name)
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
    pub fn run_animation(&mut self, animation_name: &str) -> bool {
        self.run(animation_name, AnimationStatus::RUNNING)
    }

    /// return whether or not an animation with the given name is running
    pub fn animation_running(&mut self, animation_name: &str) -> bool {
        self.animations.contains_key(animation_name) && vec![AnimationStatus::RUNNING, AnimationStatus::LOOPING, AnimationStatus::STOPPING].contains(&self.animations.get(animation_name).expect("Animation must be present").status)
    }

    /// Runs the animation `name`. Returns true is the animation has been started, false if it does not exist or was already running
    pub fn loop_animation(&mut self, animation_name: &str) -> bool {
        self.run(animation_name, AnimationStatus::LOOPING)
    }

    /// Stops the animation `name`. Returns true is the animation has been stopped, false if it does not exist or was already stopped
    pub fn stop_animation(&mut self, animation_name: &str, force: bool) -> bool {
        if self.animations.contains_key(animation_name) {
            let animation = self
                .animations
                .get_mut(animation_name)
                .expect("An animation has not been found after the security check");
            Animations::stop_single_animation(force, animation)
        } else {
            false
        }
    }

    /// Stops all the animations
    pub fn stop_all_animation(&mut self, force: bool) {
        self.animations.iter_mut().for_each(|(_k, v)| {
            Animations::stop_single_animation(force, v);
        });
    }

    fn stop_single_animation(force: bool, animation: &mut Animation) -> bool {
        if animation.status == AnimationStatus::LOOPING
            || animation.status == AnimationStatus::RUNNING
        {
            if force {
                animation.status = AnimationStatus::FORCE_STOPPED;
            } else {
                animation.status = AnimationStatus::STOPPING;
            }
            true
        } else {
            false
        }
    }

    /// Returns the mutable animations
    pub fn animations_mut(&mut self) -> &mut HashMap<String, Animation> { &mut self.animations }

    /// Return whether or not any animations is currently running. Useful to avoid double call
    pub fn any_animation_running(&self) -> bool {
        self.animations
            .values()
            .filter(|v| {
                v.status.eq(&AnimationStatus::RUNNING) || v.status.eq(&AnimationStatus::LOOPING) || v.status.eq(&AnimationStatus::STOPPING)
            })
            .count()
            > 0
    }
}

#[derive(Eq, PartialEq, Debug)]
pub(crate) enum AnimationStatus {
    FORCE_STOPPED,
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
        Animation::initialise_animation(duration, &mut modifiers);

        Self { _duration: duration, modifiers, status: AnimationStatus::STOPPED }
    }

    /// Creates a new animation with the status running
    pub fn running(duration: Duration, mut modifiers: Vec<AnimationModifier>) -> Self {
        Animation::initialise_animation(duration, &mut modifiers);

        Self { _duration: duration, modifiers, status: AnimationStatus::RUNNING }
    }

    ///Creates a new animation with the status looping
    pub fn looping(duration: Duration, mut modifiers: Vec<AnimationModifier>) -> Self {
        Animation::initialise_animation(duration, &mut modifiers);

        Self { _duration: duration, modifiers, status: AnimationStatus::LOOPING }
    }

    fn initialise_animation(duration: Duration, modifiers: &mut Vec<AnimationModifier>) {
        if duration.as_millis() != 0 {
            modifiers.iter_mut().for_each(|animation_modifier| {
                animation_modifier.single_keyframe_duration =
                    Some(duration.div(animation_modifier.number_of_keyframes as u32));
                compute_animation_keyframe_modifier(animation_modifier);
            });
        }
    }

    /// Will compute the status of the current animation
    pub(crate) fn try_update_status(&mut self) {
        if self.status == FORCE_STOPPED {
            self.status = STOPPED;
            return;
        }
        if self
            .modifiers
            .iter()
            .filter(|modifier| modifier.current_keyframe == modifier.number_of_keyframes)
            .count()
            == self.modifiers.len()
        {
            self.modifiers.iter_mut().for_each(|modifier| modifier.current_keyframe = 0);
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
    pub(crate) single_keyframe_modifier: Option<ComputedKeyframeModifier>,
    /// In case of a sprite modifier we need to keep track of the next index position in the vec
    pub(crate) next_sprite_index: Option<usize>,
    pub(crate) variant: bool
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
            variant: false
        }
    }

    /// Convenience function to directly create an AnimationModifier of type Transform with the needed informations
    pub fn transform(
        number_of_keyframes: usize,
        vector: Option<Vector>,
        scale: Option<f32>,
        rotation: Option<f32>,
    ) -> Self {
        AnimationModifier::new(
            number_of_keyframes,
            AnimationModifierType::TransformModifier { vector, scale, rotation },
        )
    }
    /// Convenience function to directly create an AnimationModifier of type Sprite with the needed informations
    pub fn sprite(tile_numbers: Vec<usize>, end_tile_number: usize) -> Self {
        AnimationModifier::new(
            tile_numbers.len() - 1,
            AnimationModifierType::SpriteModifier { tile_numbers, tile_numbers_variant: None, end_tile_number },
        )
    }

    /// Convenience function to directly create an AnimationModifier of type Sprite with the needed informations, with a variant animation
    pub fn sprite_with_variant(tile_numbers: Vec<usize>, tile_numbers_variant: Vec<usize>, end_tile_number: usize) -> Self {
        assert_eq!(tile_numbers_variant.len(), tile_numbers_variant.len());
        AnimationModifier::new(
            tile_numbers.len() - 1,
            AnimationModifierType::SpriteModifier { tile_numbers, tile_numbers_variant: Some(tile_numbers_variant), end_tile_number},
        )
    }

    /// Convenience function to create a color animation
    pub fn color(number_of_keyframes: usize, target_color: Color) -> Self {
        AnimationModifier::new(
            number_of_keyframes,
            AnimationModifierType::Color { target: target_color },
        )
    }

    /// Convenience function to create a blink animation.
    pub fn blink(number_of_blinks: usize) -> Self {
        AnimationModifier::new(number_of_blinks * 2, AnimationModifierType::Blink)
    }

    /// Convenience function to create a text animation.
    pub fn text(content: String) -> Self {
        AnimationModifier::new(content.len(), AnimationModifierType::Text { content })
    }

    pub(crate) fn retrieve_keyframe_modifier(&self) -> &ComputedKeyframeModifier {
        self.single_keyframe_modifier.as_ref().expect("single keyframe modifier is needed for transform animation")
    }

    pub(crate) fn retrieve_keyframe_modifier_mut(&mut self) -> &mut ComputedKeyframeModifier {
        self.single_keyframe_modifier.as_mut().expect("single keyframe modifier is needed for transform animation")
    }

    pub(crate) fn modifier_type(&self) -> &AnimationModifierType {
        &self.modifier_type
    }

    pub(crate) fn compute_keyframe_modifier_for_animation(&mut self, initial_color: &Color) {
        self.single_keyframe_modifier = match &self.modifier_type {
            AnimationModifierType::Color { target } => {
                let r = (target.red() as i16 - initial_color.red() as i16)
                    / self.number_of_keyframes as i16;
                let g = (target.green() as i16 - initial_color.green() as i16)
                    / self.number_of_keyframes as i16;
                let b = (target.blue() as i16 - initial_color.blue() as i16)
                    / self.number_of_keyframes as i16;
                let a = (target.alpha() - initial_color.alpha()) / self.number_of_keyframes as f32;
                Some(ComputedKeyframeModifier::Color { r, g, b, a })
            }
            _ => None,
        }
    }

    pub(crate) fn is_first_frame(&self) -> bool { self.current_keyframe == 0 }

    pub(crate) fn will_be_last_keyframe(&self, added_keyframes: usize) -> bool {
        self.current_keyframe + added_keyframes >= self.number_of_keyframes
    }
}

#[derive(Debug, Clone)]
pub enum AnimationModifierType {
    TransformModifier { vector: Option<Vector>, scale: Option<f32>, rotation: Option<f32> },
    SpriteModifier { tile_numbers: Vec<usize>, tile_numbers_variant: Option<Vec<usize>>, end_tile_number: usize },
    Color { target: Color },
    Text { content: String },
    Blink,
}

pub(crate) enum ComputedKeyframeModifier {
    TransformModifier { vector: Option<Vector>, scale: Option<f32>, rotation: Option<f32> },
    Color { r: i16, g: i16, b: i16, a: f32 },
    Text { cursor: usize }
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
                AnimationModifierType::Color { .. } => {
                    "Color"
                }
                AnimationModifierType::Blink => {
                    "Blink"
                }
                AnimationModifierType::Text { .. } => {
                    "Text"
                }
            }
        )
    }
}

fn compute_animation_keyframe_modifier(modifier: &mut AnimationModifier) {
    let keyframe_nb = modifier.number_of_keyframes as f32;
    modifier.single_keyframe_modifier = match modifier.modifier_type {
        AnimationModifierType::TransformModifier { vector, scale, rotation } => {
            Some(ComputedKeyframeModifier::TransformModifier {
                vector: vector.map_or(None, |vector| {
                    Some(Vector::new(vector.x() / keyframe_nb, vector.y() / keyframe_nb))
                }),
                scale: scale.map_or(None, |scale| Some(scale / keyframe_nb)),
                rotation: rotation.map_or(None, |rotation| Some(rotation / keyframe_nb)),
            })
        },
        AnimationModifierType::Text { .. } => Some(ComputedKeyframeModifier::Text { cursor: 0}),
        _ => None
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
                    vector: Some(Vector::new(2., 4.)),
                    scale: Some(4.),
                    rotation: Some(1.),
                },
            )],
        );

        let anim_modifier = animation.modifiers.iter().next().unwrap();
        assert_eq!(500, anim_modifier.single_keyframe_duration.unwrap().as_millis());
        if let ComputedKeyframeModifier::TransformModifier { vector, scale, rotation } =
            anim_modifier.single_keyframe_modifier.as_ref().unwrap()
        {
            assert_eq!(1.0, vector.unwrap().x());
            assert_eq!(2.0, vector.unwrap().y());
            assert_eq!(2.0, scale.unwrap());
            assert_eq!(0.5, rotation.unwrap());
        } else {
            panic!();
        }
    }

    #[test]
    fn any_animation_running_test() {
        let mut h = HashMap::new();
        h.insert(
            "d".to_string(),
            Animation {
                _duration: Default::default(),
                modifiers: vec![],
                status: AnimationStatus::RUNNING,
            },
        );
        let a = Animations::new(h);
        assert_eq!(true, a.any_animation_running());

        let mut h = HashMap::new();
        h.insert(
            "d".to_string(),
            Animation {
                _duration: Default::default(),
                modifiers: vec![],
                status: AnimationStatus::STOPPED,
            },
        );
        let a = Animations::new(h);
        assert_eq!(false, a.any_animation_running());
    }
}
