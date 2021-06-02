use legion::{world::SubWorld, *};

use crate::core::components::maths::{
    collider::{Collider, Collision},
    transform::Transform,
};

#[system(for_each)]
pub(crate) fn colliders_cleaner(collider: &mut Collider) {
    collider.clear_collisions()
}

#[system]
pub(crate) fn compute_collisions(
    world: &mut SubWorld,
    query_colliders: &mut Query<(Entity, &Transform, &mut Collider)>,
) {
    let mut colliders: Vec<(&Entity, &Transform, &mut Collider)> =
        query_colliders.iter_mut(world).collect();
    let len = colliders.len();
    for i in 0..len {
        let col = colliders.get(i).expect("");
        if !col.2.passive() {
            let mut collisions = (0..len)
                .filter(|index| *index != i)
                .filter(|index| {
                    let col2 = colliders.get(*index).expect("");
                    col.2.collides_with(col.1, col2.2, col2.1)
                })
                .map(|index| {
                    let col = colliders.get(index).expect("");
                    let col2 = colliders.get(index).expect("");
                    Collision {
                        mask: col.2.mask().clone(),
                        entity: *col.0,
                        coordinates: col2.1.global_translation().clone(),
                    }
                })
                .collect();
            colliders
                .get_mut(i)
                .expect("")
                .2
                .add_collisions(&mut collisions);
        }
    }
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
        let mut schedule = Schedule::builder()
            .add_system(colliders_cleaner_system())
            .build();

        let mut t = Transform::default();
        t.append_x(1.0);
        let e = world.push((
            1,
            t,
            Collider::new(ColliderMask::Bullet, vec![], ColliderType::Square(5)),
        ));
        let mut entry = world.entry_mut(e).expect("");
        let res = entry.get_component_mut::<Collider>().expect("");
        res.add_collisions(&mut vec![Collision {
            mask: ColliderMask::Character,
            entity: e,
            coordinates: Default::default(),
        }]);
        assert_eq!(1, res.collisions().len());

        schedule.execute(&mut world, &mut resources);

        let entry = world.entry(e).expect("");
        let res = entry.get_component::<Collider>().expect("");
        assert_eq!(0, res.collisions().len());
    }

    #[test]
    fn compute_collision_system_test() {
        let mut world = World::default();
        let mut resources = Resources::default();
        let mut schedule = Schedule::builder()
            .add_system(compute_collisions_system())
            .build();
        let mut t = Transform::default();
        t.append_x(1.0);
        let mut t2 = Transform::default();
        t2.append_x(2.0);

        let e = world.push((
            1,
            t,
            Collider::new(
                ColliderMask::Bullet,
                vec![ColliderMask::Landscape],
                ColliderType::Square(5),
            ),
        ));
        let e2 = world.push((
            2,
            t2,
            Collider::new(ColliderMask::Bullet, vec![], ColliderType::Square(5)),
        ));

        schedule.execute(&mut world, &mut resources);

        let entry = world.entry(e).expect("");
        let res = entry.get_component::<Collider>().expect("");
        assert_eq!(0, res.collisions().len());

        let entry = world.entry(e2).expect("");
        let res = entry.get_component::<Collider>().expect("");
        assert_eq!(1, res.collisions().len());
    }
}
