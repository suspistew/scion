use std::collections::{HashMap, HashSet};

use crate::core::components::{
    color::Color,
    material::Material,
    maths::{
        collider::{Collider, ColliderDebug, ColliderMask, ColliderType, Collision},
        coordinates::Coordinates,
        hierarchy::Parent,
        transform::Transform,
    },
    shapes::polygon::Polygon,
};

pub(crate) fn collider_cleaner_system(world: &mut crate::core::world::World) {
    for (_, c) in world.query_mut::<&mut Collider>() {
        c.clear_collisions();
    }
}

/// System responsible to compute collision between colliders, following the mask filters
pub(crate) fn compute_collisions_system(world: &mut crate::core::world::World) {
    let mut res: HashMap<hecs::Entity, Vec<Collision>> = HashMap::default();

    {
        let colliders: Vec<(hecs::Entity, Transform, Collider)> = {
            let mut res = Vec::new();
            for (e,(t,c)) in world.query::<(&Transform, &Collider)>().iter(){
                res.push((e, t.clone(),c.clone()));
            }
            res
        };

        let mut colliders_by_mask: HashMap<ColliderMask, Vec<(hecs::Entity, &Transform, &Collider)>, > = HashMap::default();

        colliders.iter().for_each(|(e, t, c)| {
            colliders_by_mask.entry(c.mask().clone()).or_insert_with(|| Vec::new()).push((*e, t, c))
        });

        let mut cpt = 0;
        colliders.iter().for_each(|(entity, transform, collider)| {
            collider.filters().iter().filter(|mask| colliders_by_mask.contains_key(mask)).for_each(
                |mask| {
                    colliders_by_mask.get(mask).expect("Impossible to find a collider's entry")
                        .iter()
                        .filter(|(_e, t, c)| {
                            cpt += 1;
                            collider.collides_with(transform, c, t)
                        })
                        .for_each(|(_e, t, c)| {
                            res.entry(*entity).or_insert_with(|| Vec::new()).push(Collision {
                                mask: c.mask().clone(),
                                entity: *entity,
                                coordinates: t.global_translation().clone(),
                            });
                        })
                }
            );
        });
    }

    res.drain().for_each(|(e, mut collisions)| {
        world.entry_mut::<&mut Collider>(e)
            .expect("Collisions on unreachable collider")
            .add_collisions(&mut collisions);
    });
}

/// System responsible to add a `ColliderDebug` component to each colliders that are in debug mode
pub(crate) fn debug_colliders_system(world: &mut crate::core::world::World){
    let collider_debug: HashSet<hecs::Entity> = fetch_collider_debug_entities(world);
    let mut debug_lines_to_add = Vec::new();
    for (entity, (_, collider)) in world.query_mut::<(&Transform, &mut Collider)>(){
        if collider.debug_lines() && !collider_debug.contains(&entity) {
            let (width, height) = match collider.collider_type() {
                ColliderType::Square(size) => (*size as f32, *size as f32),
                ColliderType::Rectangle(width, height) => (*width as f32 as f32, *height as f32),
            };
            let offset = collider.offset();
            debug_lines_to_add.push((
                Parent(entity),
                ColliderDebug,
                Transform::from_xyz(offset.x(), offset.y(), 99),
                Polygon::new(vec![
                    Coordinates::new(0., 0.),
                    Coordinates::new(width, 0.),
                    Coordinates::new(width, height),
                    Coordinates::new(0., height),
                    Coordinates::new(0., 0.),
                ]),
                Material::Color(Color::new_rgb(0, 255, 0)),
            ));
        }
    }

    debug_lines_to_add.drain(0..).for_each(|components|{
        world.push(components);
    });
}

fn fetch_collider_debug_entities(world: &mut crate::core::world::World) -> HashSet<hecs::Entity> {
    let mut res = HashSet::new();
    for (e, _) in world.query::<&ColliderDebug>().iter() {
        res.insert(e);
    }
    res
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::core::components::maths::{
        collider::{Collider, ColliderMask, ColliderType, Collision},
        transform::Transform,
    };

    #[test]
    fn clear_collision_system_test() {
        let mut world = crate::core::world::World::default();

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
        }]);
        assert_eq!(1, entry.collisions().len());

        collider_cleaner_system(&mut world);

        assert_eq!(0, world.entry::<&Collider>(e).unwrap().get().unwrap().collisions().len());
    }

    #[test]
    fn compute_collision_system_test() {
        let mut world = crate::core::world::World::default();

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
;
        assert_eq!(1,  world.entry::<&Collider>(e2).unwrap().get().unwrap().collisions().len());
    }

    #[test]
    fn debug_colliders_system_test() {
        let mut world = crate::core::world::World::default();

        let _collider = world.push((
            Transform::default(),
            Collider::new(ColliderMask::None, vec![], ColliderType::Square(100)).with_debug_lines(),
        ));

        debug_colliders_system(&mut world);

        let res = world.query::<(&ColliderDebug, &Parent)>().iter().count();
        assert_eq!(1, res);
    }



}
