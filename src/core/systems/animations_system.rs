use legion::*;

use crate::core::{
    components::{
        animations::{AnimationModifierType, AnimationStatus, Animations},
        maths::transform::Transform,
        tiles::sprite::Sprite,
    },
    resources::time::{TimerType, Timers},
};

/// System responsible of applying modifiers data to the dedicated components
/// It will use timers to keep track of the animation and will merge keyframes in case
/// of long frames.
#[system(for_each)]
pub(crate) fn animation_executer(
    #[resource] timers: &mut Timers,
    entity: &Entity,
    animations: &mut Animations,
    transform: &mut Transform,
    mut sprite: Option<&mut Sprite>,
) {
    animations
        .animations_mut()
        .iter_mut()
        .filter(|(_, v)| v.status != AnimationStatus::STOPPED)
        .for_each(|(k, animation)| {
            for mut modifier in animation.modifiers.iter_mut() {
                let mut timer_created = false;
                let timer_id = format!("{:?}-{}-{}", *entity, k, modifier.to_string());
                let timer_id = timer_id.as_str();
                if let Ok(timer) = timers.add_timer(
                    timer_id,
                    TimerType::Cyclic,
                    modifier
                        .single_keyframe_duration
                        .expect("Single keyframe duration is missing for animations")
                        .as_secs_f32(),
                ) {
                    timer.reset();
                    timer_created = true;
                }

                let timer_cycle = {
                    let cycles = timers
                        .get_timer(timer_id)
                        .expect("Timer must exist")
                        .cycle();
                    let keyframes_left = modifier.number_of_keyframes - modifier.current_keyframe;
                    if cycles > keyframes_left {
                        keyframes_left
                    } else {
                        cycles
                    }
                };
                if timer_cycle > 0 || timer_created {
                    match &modifier.modifier_type {
                        AnimationModifierType::TransformModifier { .. } => {
                            match modifier.single_keyframe_modifier.as_ref().expect(
                                "single keyframe modifier is needed for transform animation",
                            ) {
                                AnimationModifierType::TransformModifier {
                                    vector: coordinates,
                                    scale,
                                    rotation,
                                } => {
                                    for _i in 0..timer_cycle {
                                        if let Some(coordinates) = coordinates {
                                            transform.append_translation(
                                                coordinates.x(),
                                                coordinates.y(),
                                            );
                                        }
                                        if let Some(scale) = scale {
                                            transform.set_scale(transform.scale + scale);
                                        }
                                        if let Some(rotation) = rotation {
                                            transform.append_angle(*rotation);
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                        AnimationModifierType::SpriteModifier {
                            tile_numbers,
                            end_tile_number,
                        } => {
                            if let Some(ref mut animation_sprite) = sprite {
                                if modifier.next_sprite_index.is_none() {
                                    modifier.next_sprite_index = Some(0);
                                }
                                if modifier.current_keyframe == (modifier.number_of_keyframes - 1)
                                    && animation.status != AnimationStatus::LOOPING
                                {
                                    animation_sprite.set_tile_nb(*end_tile_number);
                                    modifier.next_sprite_index = None;
                                } else {
                                    animation_sprite.set_tile_nb(
                                        *tile_numbers
                                            .get(modifier.next_sprite_index.unwrap())
                                            .unwrap(),
                                    );
                                    modifier
                                        .next_sprite_index
                                        .replace(modifier.next_sprite_index.unwrap() + 1);
                                }
                            }
                        }
                    }
                    modifier.current_keyframe += timer_cycle;
                    if modifier.current_keyframe >= modifier.number_of_keyframes {
                        modifier.next_sprite_index = None;
                        let _r = timers.delete_timer(timer_id);
                    }
                }
            }
            animation.try_update_status();
        });
}
