use std::collections::{HashMap, HashSet};
use hecs::Entity;
use log::info;

use crate::core::components::{color::Color, material::Material, maths::{
    collider::{Collider, ColliderDebug, ColliderMask, ColliderType, Collision},
    coordinates::Coordinates,
    hierarchy::Parent,
    transform::Transform,
}, shapes::polygon::Polygon, Square, Triangle};
use crate::core::components::maths::Pivot;
use crate::core::components::shapes::line::Line;
use crate::core::components::shapes::rectangle::Rectangle;
use crate::core::components::tiles::sprite::Sprite;
use crate::core::resources::global_storage::GlobalStorage;
use crate::core::resources::inputs::types::{Input, KeyCode, Shortcut};
use crate::core::world::{GameData, World};
use crate::rendering::Renderable2D;
use crate::utils::maths::Vector;

pub(crate) fn collider_cleaner_system(data: &mut GameData) {
    for (_, c) in data.query_mut::<&mut Collider>() {
        c.clear_collisions();
    }
}

/// System responsible to compute collision between colliders, following the mask filters
pub(crate) fn compute_collisions_system(data: &mut GameData) {
    let mut res: HashMap<Entity, Vec<Collision>> = HashMap::default();

    {
        let colliders: Vec<(Entity, Transform, Collider)> = {
            let mut res = Vec::new();
            for (e, (t, c)) in data.query::<(&Transform, &Collider)>().iter() {
                res.push((e, t.clone(), c.clone()));
            }
            res
        };

        let mut colliders_by_mask: HashMap<
            ColliderMask,
            Vec<(Entity, &Transform, &Collider)>,
        > = HashMap::default();

        colliders.iter().for_each(|(e, t, c)| {
            colliders_by_mask.entry(c.mask().clone()).or_insert_with(|| Vec::new()).push((*e, t, c))
        });

        let mut cpt = 0;
        colliders.iter().for_each(|(entity, transform, collider)| {
            collider.filters().iter().filter(|mask| colliders_by_mask.contains_key(mask)).for_each(
                |mask| {
                    colliders_by_mask
                        .get(mask)
                        .expect("Impossible to find a collider's entry")
                        .iter()
                        .map(|(_e, t, c)| {
                            cpt += 1;
                            (_e, t, c, collider.collides_with(transform, c, t))
                        })
                        .filter(|(_e, t, c, collision_area)| collision_area.is_some())
                        .for_each(|(_e, t, c, collision_area)| {
                            res.entry(*entity).or_insert_with(|| Vec::new()).push(Collision {
                                mask: c.mask().clone(),
                                entity: *entity,
                                coordinates: t.global_translation().clone(),
                                collision_area: collision_area.expect("Filtered Option is still KO"),
                            });
                        })
                },
            );
        });
    }


    res.drain().for_each(|(e, mut collisions)| {
        data.entry_mut::<&mut Collider>(e)
            .expect("Collisions on unreachable collider")
            .add_collisions(&mut collisions);
    });
}

/// System responsible to add a `ColliderDebug` component to each colliders that are in debug mode
pub(crate) fn debug_colliders_system(data: &mut GameData) {
    let global_debug_activated = handle_global_debug_colliders(data);
    let mut collider_debug = fetch_collider_debug_entities(data);
    let mut debug_lines_to_add = Vec::new();
    let mut debug_lines_to_remove = Vec::new();
    for (entity, (_, collider, square, rectangle, triangle, polygon, sprite, line))
    in data.query_mut::<(&Transform, &mut Collider, Option<&Square>, Option<&Rectangle>, Option<&Triangle>, Option<&Polygon>, Option<&Sprite>, Option<&Line>)>() {
        if (collider.debug_lines() || global_debug_activated) && !collider_debug.0.contains(&entity) {
            let (width, height) = match collider.collider_type() {
                ColliderType::Square(size) => (*size as f32, *size as f32),
                ColliderType::Rectangle(width, height) => (*width as f32, *height as f32),
            };
            let color = match collider.mask() {
                ColliderMask::None => Color::new_rgb(255, 255, 255),
                ColliderMask::Character => Color::new_rgb(255, 0, 0),
                ColliderMask::Bullet => Color::new_rgb(255, 0, 0),
                ColliderMask::Death => Color::new_rgb(255, 0, 0),
                ColliderMask::Landscape => Color::new_rgb(0, 255, 0),
                ColliderMask::Custom(_) => Color::new_rgb(0, 0, 255)
            };
            let offset = collider.offset();
            let pivot = get_pivot(square, rectangle, triangle, polygon, sprite, line);
            let mut polygon_collider = Polygon::new(vec![
                Coordinates::new(0., 0.),
                Coordinates::new(width, 0.),
                Coordinates::new(width, height),
                Coordinates::new(0., height)
            ]).pivot(pivot);
            debug_lines_to_add.push((
                Parent(entity),
                ColliderDebug,
                Transform::from_xyz(offset.x(), offset.y(), 99),
                polygon_collider,
                Material::Color(color),
            ));
        } else if !collider.debug_lines() && !global_debug_activated && collider_debug.0.contains(&entity) {
            debug_lines_to_remove.push(entity);
        }
    }

    debug_lines_to_add.drain(0..).for_each(|components| {
        data.push(components);
    });
    debug_lines_to_remove.drain(0..).for_each(|e| {
        let _r = data.remove(collider_debug.1.remove(&e).expect(""));
    });
}

fn get_pivot(square: Option<&Square>,
             rectangle: Option<&Rectangle>,
             triangle: Option<&Triangle>,
             polygon: Option<&Polygon>,
             sprite: Option<&Sprite>,
             line: Option<&Line>) -> Pivot {
    if sprite.is_some() { sprite.unwrap().get_pivot() }
    else if square.is_some() { square.unwrap().get_pivot() }
    else if rectangle.is_some() { rectangle.unwrap().get_pivot() }
    else if polygon.is_some() { polygon.unwrap().get_pivot() }
    else if triangle.is_some() { triangle.unwrap().get_pivot() }
    else if line.is_some() { line.unwrap().get_pivot() }
    else { Pivot::TopLeft }
}

fn handle_global_debug_colliders(game_data: &mut GameData) -> bool {
    let shortcut_event = game_data.inputs().shortcut_pressed_event(&vec![Input::Key(KeyCode::LShift), Input::Key(KeyCode::D)]);

    let mut resources = game_data.get_resource_mut::<GlobalStorage>().expect("Missing Global Storage resource");
    let mut current_val = resources.flags.get("debug_colliders").get_or_insert(&false).clone();

    if shortcut_event {
        current_val = !current_val;
        resources.flags.insert("debug_colliders".to_string(), current_val);
    }

    current_val
}

fn fetch_collider_debug_entities(data: &mut GameData) -> (HashSet<Entity>, HashMap<Entity, Entity>) {
    let mut parents = HashSet::new();
    let mut debug_line: HashMap<Entity, Entity> = HashMap::new();
    for (e, (_, parent)) in data.query::<(&ColliderDebug, &Parent)>().iter() {
        parents.insert(parent.0);
        debug_line.insert(parent.0, e);
    }
    (parents, debug_line)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::components::maths::{
        collider::{Collider, ColliderMask, ColliderType, Collision},
        transform::Transform,
    };
    use crate::core::components::maths::collider::CollisionArea;
    use crate::core::resources::inputs::inputs_controller::InputsController;
    use crate::core::world::GameData;

    #[test]
    fn clear_collision_system_test() {
        let mut world = GameData::default();

        let mut t = Transform::default();
        t.append_x(1.0);
        let e = world.push((
            1,
            t,
            Collider::new(ColliderMask::Bullet, vec![], ColliderType::Square(5)),
        ));

        let entry = world.entry_mut::<&mut Collider>(e).unwrap();
        entry.add_collisions(&mut vec![Collision {
            mask: ColliderMask::Character,
            entity: e,
            coordinates: Default::default(),
            collision_area: CollisionArea { start_point: Default::default(), end_point: Default::default() },
        }]);
        assert_eq!(1, entry.collisions().len());

        collider_cleaner_system(&mut world);

        assert_eq!(0, world.entry::<&Collider>(e).unwrap().get().unwrap().collisions().len());
    }

    #[test]
    fn compute_collision_system_test() {
        let mut world = GameData::default();

        let mut t = Transform::default();
        t.append_x(1.0);
        let mut t2 = Transform::default();
        t2.append_x(2.0);

        let e = world.push((
            1,
            t,
            Collider::new(ColliderMask::Bullet, vec![], ColliderType::Square(5)),
        ));
        let e2 = world.push((
            2,
            t2,
            Collider::new(
                ColliderMask::Landscape,
                vec![ColliderMask::Bullet],
                ColliderType::Square(5),
            ),
        ));

        compute_collisions_system(&mut world);

        assert_eq!(0, world.entry::<&Collider>(e).unwrap().get().unwrap().collisions().len());
        assert_eq!(1, world.entry::<&Collider>(e2).unwrap().get().unwrap().collisions().len());
    }

    #[test]
    fn debug_colliders_system_test() {
        let mut world = GameData::default();
        world.insert_resource(InputsController::default());
        world.insert_resource(GlobalStorage::default());

        let _collider = world.push((
            Transform::default(),
            Collider::new(ColliderMask::None, vec![], ColliderType::Square(100)).with_debug_lines(),
        ));

        debug_colliders_system(&mut world);

        let res = world.query::<(&ColliderDebug, &Parent)>().iter().count();
        assert_eq!(1, res);
    }
}
