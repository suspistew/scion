use std::collections::{HashMap, HashSet};

use legion::{systems::CommandBuffer, world::SubWorld, *};

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

#[system(for_each)]
pub(crate) fn colliders_cleaner(collider: &mut Collider) { collider.clear_collisions() }

/// System responsible to compute collision between colliders, following the mask filters
#[system]
pub(crate) fn compute_collisions(
    world: &mut SubWorld,
    query_colliders: &mut Query<(Entity, &Transform, &mut Collider)>,
) {
    let mut res: HashMap<Entity, Vec<Collision>> = HashMap::default();
    let mut colliders: Vec<(&Entity, &Transform, &mut Collider)> =
        query_colliders.iter_mut(world).collect();
    {
        let mut colliders_by_mask: HashMap<
            ColliderMask,
            Vec<(&&Entity, &&Transform, &&mut Collider)>,
        > = HashMap::default();
        colliders.iter().for_each(|(e, t, c)| {
            colliders_by_mask.entry(c.mask().clone()).or_insert_with(|| Vec::new()).push((e, t, c))
        });

        let mut cpt = 0;
        colliders.iter().for_each(|(entity, transform, collider)| {
            collider.filters().iter().filter(|mask| colliders_by_mask.contains_key(mask)).for_each(
                |mask| {
                    colliders_by_mask
                        .get(mask)
                        .unwrap()
                        .iter()
                        .filter(|(_e, t, c)| {
                            cpt += 1;
                            collider.collides_with(transform, c, t)
                        })
                        .for_each(|(_e, t, c)| {
                            res.entry(**entity).or_insert_with(|| Vec::new()).push(Collision {
                                mask: c.mask().clone(),
                                entity: **entity,
                                coordinates: t.global_translation().clone(),
                            });
                        })
                },
            );
        });
    }

    colliders.iter_mut().for_each(|(e, _, c)| {
        if res.contains_key(*e) {
            c.add_collisions(&mut (res.remove(*e).unwrap()));
        }
    });
}

/// System responsible to add a `ColliderDebug` component to each colliders that are in debug mode
#[system]
pub(crate) fn debug_colliders(
    cmd: &mut CommandBuffer,
    world: &mut SubWorld,
    query_colliders: &mut Query<(Entity, &Transform, &mut Collider)>,
    query_collider_debug: &mut Query<(&ColliderDebug, &Parent)>,
) {
    let (mut collider_world, debug_world) = world.split_for_query(query_colliders);

    let collider_debug: HashSet<Entity> =
        query_collider_debug.iter(&debug_world).map(|(_, p)| p.0).collect();

    query_colliders.for_each_mut(&mut collider_world, |(entity, _, collider)| {
        if collider.debug_lines() && !collider_debug.contains(entity) {
            let (width, height) = match collider.collider_type() {
                ColliderType::Square(size) => (*size as f32, *size as f32),
                ColliderType::Rectangle(width, height) => (*width as f32 as f32, *height as f32),
            };
            cmd.push((
                Parent(*entity),
                ColliderDebug,
                Transform::from_xyz(0., 0., 99),
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
    });
}

#[cfg(test)]
mod tests {
    use legion::{EntityStore, Resources, Schedule, World};

    use super::*;
    use crate::core::components::maths::{
        collider::{Collider, ColliderMask, ColliderType, Collision},
        transform::Transform,
    };

    #[test]
    fn clear_collision_system_test() {
        let mut world = World::default();
        let mut resources = Resources::default();
        let mut schedule = Schedule::builder().add_system(colliders_cleaner_system()).build();

        let mut t = Transform::default();
        t.append_x(1.0);
        let e = world.push((
            1,
            t,
            Collider::new(ColliderMask::Bullet, vec![], ColliderType::Square(5)),
        ));
        let mut entry = world.entry_mut(e).unwrap();
        let res = entry.get_component_mut::<Collider>().unwrap();
        res.add_collisions(&mut vec![Collision {
            mask: ColliderMask::Character,
            entity: e,
            coordinates: Default::default(),
        }]);
        assert_eq!(1, res.collisions().len());

        schedule.execute(&mut world, &mut resources);

        let entry = world.entry(e).unwrap();
        let res = entry.get_component::<Collider>().unwrap();
        assert_eq!(0, res.collisions().len());
    }

    #[test]
    fn compute_collision_system_test() {
        let mut world = World::default();
        let mut resources = Resources::default();
        let mut schedule = Schedule::builder().add_system(compute_collisions_system()).build();
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

        schedule.execute(&mut world, &mut resources);

        let entry = world.entry(e).unwrap();
        let res = entry.get_component::<Collider>().unwrap();
        assert_eq!(0, res.collisions().len());

        let entry = world.entry(e2).unwrap();
        let res = entry.get_component::<Collider>().unwrap();
        assert_eq!(1, res.collisions().len());
    }

    #[test]
    fn debug_colliders_system_test() {
        let mut world = World::default();
        let mut resources = Resources::default();
        let mut schedule = Schedule::builder().add_system(debug_colliders_system()).build();

        let _collider = world.push((
            Transform::default(),
            Collider::new(ColliderMask::None, vec![], ColliderType::Square(100)).with_debug_lines(),
        ));
        schedule.execute(&mut world, &mut resources);

        let res = <(&ColliderDebug, &Parent)>::query().iter(&world).count();
        assert_eq!(1, res);
    }
}
