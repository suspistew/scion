use std::collections::HashMap;

use legion::{system, world::SubWorld, Entity, EntityStore, Query};

use crate::core::components::maths::{
    hierarchy::{Children, Parent},
    transform::Transform,
};

#[system]
pub(crate) fn dirty_child(
    world: &mut SubWorld,
    query_transform_with_parent: &mut Query<(
        Entity,
        &mut Transform,
        Option<&Parent>,
        Option<&Children>,
    )>,
) {
    let mut parent_to_check = Vec::new();
    query_transform_with_parent.for_each_mut(world, |(child_entity, transform, parent, _child)| {
        if transform.dirty_child {
            if let Some(parent) = parent {
                parent_to_check.push((*child_entity, parent.0));
            } else {
                transform.dirty_child = false;
            }
        }
    });

    let mut parents_transform = HashMap::new();
    for (_, parent) in parent_to_check.iter() {
        if let Ok(parent_transform) = world.entry_ref(*parent).unwrap().get_component::<Transform>()
        {
            parents_transform.insert(*parent, parent_transform.clone());
        }
    }

    while !parent_to_check.is_empty() {
        let mut tmp_vec = parent_to_check.clone();
        parent_to_check.clear();
        while let Some((child, parent)) = tmp_vec.pop() {
            let mut child_entry = world
                .entry_mut(child)
                .expect("An entity has been marked as dirty child but actually does not exist");
            let mut child_transform = child_entry
                .get_component_mut::<Transform>()
                .expect("Missing Transform component previously found on this entity");
            if parents_transform.contains_key(&parent) {
                let parent_transform = parents_transform.get(&parent).unwrap();
                if !parent_transform.dirty_child {
                    child_transform.dirty_child = false;
                    child_transform
                        .compute_global_from_parent(parent_transform.global_translation());
                    parents_transform.insert(child, child_transform.clone());
                } else {
                    parent_to_check.push((child, parent));
                }
            } else {
                child_transform.dirty_child = false;
                parents_transform.insert(child, child_transform.clone());
            }
        }
    }
}

#[system]
pub(crate) fn dirty_transform(
    world: &mut SubWorld,
    query_transform_with_childs: &mut Query<(&mut Transform, Option<&Children>)>,
) {
    let mut first_iter = true;
    let mut transform_entities: Vec<(Transform, Vec<Entity>)> = Vec::new();
    while first_iter || !transform_entities.is_empty() {
        first_iter = false;
        while let Some((transform, entities)) = transform_entities.pop() {
            for entity in entities {
                let mut child_entry = world
                    .entry_mut(entity)
                    .expect("An entity has been marked as dirty child but actually does not exist");
                let child_transform = child_entry.get_component_mut::<Transform>();
                if let Ok(child_transform) = child_transform {
                    child_transform.compute_global_from_parent(transform.global_translation())
                }
            }
        }

        query_transform_with_childs.for_each_mut(world, |(mut parent_transform, children)| {
            if let Some(children) = children {
                if parent_transform.dirty {
                    transform_entities.push((parent_transform.clone(), children.0.clone()));
                }
            }
            parent_transform.dirty = false;
        });
    }
}

#[cfg(test)]
mod tests {
    use legion::{IntoQuery, Resources, Schedule, World};

    use super::*;
    use crate::core::{components::maths::hierarchy::Parent, systems::hierarchy_system::*};

    #[test]
    fn dirty_parent_transform_test() {
        let mut world = World::default();
        let mut resources = Resources::default();
        let mut schedule = Schedule::builder()
            .add_system(children_manager_system())
            .flush()
            .add_system(dirty_child_system())
            .flush()
            .add_system(dirty_transform_system())
            .build();

        let parent_transform = Transform::from_xy(1., 1.);
        let child_transform = Transform::from_xy(1., 1.);
        let child_of_child_transform = Transform::from_xy(1., 1.);
        let parent = world.push((parent_transform,));
        let child = world.push((child_transform, Parent(parent)));
        let child_of_child = world.push((child_of_child_transform, Parent(child)));

        let mut query = <(Entity, &Transform)>::query();
        query.for_each(&world, |(_e, t)| {
            assert_eq!(false, t.dirty);
        });

        schedule.execute(&mut world, &mut resources);

        assert_eq!(
            1.,
            world
                .entry(parent)
                .unwrap()
                .get_component::<Transform>()
                .unwrap()
                .global_translation
                .x()
        );
        assert_eq!(
            2.,
            world
                .entry(child)
                .unwrap()
                .get_component::<Transform>()
                .unwrap()
                .global_translation
                .x()
        );
        assert_eq!(
            3.,
            world
                .entry(child_of_child)
                .unwrap()
                .get_component::<Transform>()
                .unwrap()
                .global_translation
                .x()
        );

        let mut query = <&mut Transform>::query();
        let res = query.get_mut(&mut world, parent).unwrap();
        res.append_translation(5.0, 1.0);

        schedule.execute(&mut world, &mut resources);

        assert_eq!(
            6.,
            world
                .entry(parent)
                .unwrap()
                .get_component::<Transform>()
                .unwrap()
                .global_translation
                .x()
        );
        assert_eq!(
            7.,
            world
                .entry(child)
                .unwrap()
                .get_component::<Transform>()
                .unwrap()
                .global_translation
                .x()
        );
        assert_eq!(
            8.,
            world
                .entry(child_of_child)
                .unwrap()
                .get_component::<Transform>()
                .unwrap()
                .global_translation
                .x()
        );
    }
}
