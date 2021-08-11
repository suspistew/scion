use legion::*;

use crate::core::{
    components::{
        animations::{AnimationModifierType, AnimationStatus, Animations},
        maths::transform::Transform,
        tiles::sprite::Sprite,
    },
    resources::time::{TimerType, Timers},
};
use crate::core::components::animations::{AnimationModifier, ComputedKeyframeModifier};
use crate::core::components::material::Material;
use crate::core::components::color::Color;

/// System responsible of applying modifiers data to the dedicated components
/// It will use timers to keep track of the animation and will merge keyframes in case
/// of long frames.
#[system(for_each)]
pub(crate) fn animation_executer(
    #[resource] timers: &mut Timers,
    entity: &Entity,
    animations: &mut Animations,
    mut transform: Option<&mut Transform>,
    mut sprite: Option<&mut Sprite>,
    mut material: Option<&mut Material>,
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
                    match modifier.modifier_type.clone() {
                        AnimationModifierType::TransformModifier { .. } => {
                            apply_transform_modifier(transform.as_mut(), modifier, timer_cycle)
                        }
                        AnimationModifierType::SpriteModifier {
                            tile_numbers,
                            end_tile_number,
                        } => {
                            apply_sprite_modifier(sprite.as_mut(), &animation.status, modifier, &tile_numbers, end_tile_number)
                        }
                        AnimationModifierType::Color { .. } => {
                            apply_color_modifier(material.as_mut(), modifier, timer_cycle)
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

fn apply_transform_modifier(mut transform: Option<&mut &mut Transform>, modifier: &mut AnimationModifier, timer_cycle: usize) {
    match modifier.single_keyframe_modifier.as_ref().expect(
        "single keyframe modifier is needed for transform animation",
    ) {
        ComputedKeyframeModifier::TransformModifier {
            vector: coordinates,
            scale,
            rotation,
        } => {
            if let Some(ref mut transform) = transform {
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
        }
        _ => {}
    }
}

fn apply_sprite_modifier(mut sprite: Option<&mut &mut Sprite>,
                         status: &AnimationStatus,
                         modifier: &mut AnimationModifier,
                         tile_numbers: &Vec<usize>,
                         end_tile_number: usize) {
    if let Some(ref mut animation_sprite) = sprite {
        if modifier.next_sprite_index.is_none() {
            modifier.next_sprite_index = Some(0);
        }
        if modifier.current_keyframe == (modifier.number_of_keyframes - 1)
            && status != &AnimationStatus::LOOPING
        {
            animation_sprite.set_tile_nb(end_tile_number);
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

fn apply_color_modifier(material: Option<&mut &mut Material>, modifier: &mut AnimationModifier, timer_cycle: usize) {
    if let Some(Material::Color(ref mut color)) = material{
        if modifier.is_first_frame(){
            modifier.compute_keyframe_modifier_for_animation(color);
        }
        if let Some(ComputedKeyframeModifier::Color{ r, g, b, a }) = &modifier.single_keyframe_modifier {
            for i in 0..timer_cycle {
                if modifier.will_be_last_keyframe(i){
                    if let AnimationModifierType::Color{ target } = &modifier.modifier_type{
                        color.replace(target.clone())
                    }
                }else{
                    let new_color  = Color::new((r + color.red() as i16) as u8,(g + color.green() as i16) as u8,(b + color.blue() as i16) as u8,a + color.alpha());
                    color.replace(new_color);
                }

            }
        }

    }
}
