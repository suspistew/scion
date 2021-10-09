use legion::{systems::CommandBuffer, *};
use scion::core::{
    components::{
        maths::{
            collider::{Collider, ColliderMask, ColliderType},
            transform::Transform,
        },
        shapes::rectangle::Rectangle,
    },
    resources::{
        events::Events,
        time::{TimerType, Timers},
    },
};

use crate::{inputs_layer::Line, main_layer::LineDirection};

const LINE_SPEED: f32 = 4.;

#[system(for_each)]
pub fn line_update(
    cmd: &mut CommandBuffer,
    entity: &Entity,
    line: &Line,
    transform: &mut Transform,
    rectangle: &mut Rectangle,
    collider: &Collider,
    #[resource] timers: &mut Timers,
    #[resource] events: &mut Events,
) {
    let timer_name = format!("{:?}_line", entity);
    let timer_str = timer_name.as_str();
    if !timers.exists(timer_str) {
        let _r = timers.add_timer(timer_str, TimerType::Cyclic, 0.02);
    }
    let timer = timers.get_timer(timer_str).unwrap();

    if timer.cycle() > 0 {
        match line.direction {
            LineDirection::LEFT => {
                rectangle.set_width(rectangle.width() + LINE_SPEED);
                transform.append_x(-LINE_SPEED);
            }
            LineDirection::RIGHT => {
                rectangle.set_width(rectangle.width() + LINE_SPEED);
            }
            LineDirection::TOP => {
                transform.append_y(-LINE_SPEED);
                rectangle.set_height(rectangle.height() + LINE_SPEED)
            }
            LineDirection::BOTTOM => rectangle.set_height(rectangle.height() + LINE_SPEED),
        }

        if collider.is_colliding() {
            if collider.collisions().iter().filter(|e| e.mask() == &ColliderMask::Bullet).count()
                > 0
            {
                cmd.remove(*entity);
            } else {
                match line.direction {
                    LineDirection::LEFT | LineDirection::RIGHT => {
                        let y = (transform.translation().y() - 10.) as usize / 16;
                        let _r = events.publish(
                            "TILEMAP_UPDATE",
                            (
                                ((transform.translation().x()) - 10.) as usize / 16,
                                (((transform.translation().x() + rectangle.width()) - 10.)
                                    as usize
                                    / 16)
                                    - 1,
                                y,
                                y,
                            ),
                        );
                        cmd.add_component(
                            *entity,
                            Collider::new(
                                ColliderMask::Custom("BORDER_CUSTOM_HORIZONTAL".to_string()),
                                vec![],
                                ColliderType::Rectangle(
                                    rectangle.width() as usize,
                                    rectangle.height() as usize,
                                ),
                            ),
                        );
                    }
                    LineDirection::TOP | LineDirection::BOTTOM => {
                        let x = (transform.translation().x() - 10.) as usize / 16;
                        let _r = events.publish(
                            "TILEMAP_UPDATE",
                            (
                                x,
                                x,
                                (transform.translation().y() - 10.) as usize / 16,
                                (((transform.translation().y() + rectangle.height()) - 10.)
                                    as usize
                                    / 16)
                                    - 1,
                            ),
                        );
                        cmd.add_component(
                            *entity,
                            Collider::new(
                                ColliderMask::Custom("BORDER_CUSTOM_VERTICAL".to_string()),
                                vec![],
                                ColliderType::Rectangle(
                                    rectangle.width() as usize,
                                    rectangle.height() as usize,
                                ),
                            ),
                        );
                    }
                }

                cmd.remove_component::<Rectangle>(*entity);
            }
        } else {
            cmd.add_component(
                *entity,
                Collider::new(
                    ColliderMask::Landscape,
                    vec![
                        ColliderMask::Bullet,
                        ColliderMask::Custom("BORDER_CUSTOM_HORIZONTAL".to_string()),
                        ColliderMask::Custom("BORDER_CUSTOM_VERTICAL".to_string()),
                        ColliderMask::Custom("BORDER_TOP".to_string()),
                        ColliderMask::Custom("BORDER_BOTTOM".to_string()),
                        ColliderMask::Custom("BORDER_LEFT".to_string()),
                        ColliderMask::Custom("BORDER_RIGHT".to_string()),
                    ],
                    ColliderType::Rectangle(
                        rectangle.width() as usize,
                        rectangle.height() as usize,
                    ),
                ),
            );
        }
    }
}
