use hecs::QueryOneError;
use std::collections::HashMap;

use crate::core::components::maths::hierarchy::{Children, Parent};
use crate::core::world::{GameData, World};

/// System responsible to add/modify Children components to the entities referenced by a Parent component
/// If the parent component referenced by the Children one is not found, then it deletes the entity
/// If an entity has a Children component referencing non existing children, then this system will remove these references
pub(crate) fn children_manager_system(data: &mut GameData) {
    let mut parents = fetch_parent_entities(data);
    let mut component_to_add = HashMap::new();
    let mut entities_to_remove = Vec::new();

    parents.drain(0..).for_each(|(child_entity, parent_entity)| {
        match data.entry_mut::<&mut Children>(parent_entity) {
            Ok(children) => {
                if !children.0.contains(&child_entity) {
                    children.0.push(child_entity);
                }
            }
            Err(e) => {
                match e {
                    QueryOneError::NoSuchEntity => {
                        // If the parent has been removed, we delete any child linked to it
                        entities_to_remove.push(child_entity);
                    }
                    QueryOneError::Unsatisfied => {
                        // If the parent hasn't the Children component, we add it to the buffer
                        component_to_add
                            .entry(parent_entity)
                            .or_insert(Vec::new())
                            .push(child_entity);
                    }
                }
            }
        }
    });
    component_to_add.drain().for_each(|(e, children)| {
        let _r = data.add_components(e, (Children(children),));
    });
    entities_to_remove.drain(0..).for_each(|e| {
        let _r = data.remove(e);
    });

    let entities = data.entities();
    for (_, c) in data.query_mut::<&mut Children>() {
        c.0.retain(|e| entities.contains(e));
    }
}

fn fetch_parent_entities(data: &mut GameData) -> Vec<(hecs::Entity, hecs::Entity)> {
    let mut res = Vec::new();
    for (e, p) in data.query::<&Parent>().iter() {
        res.push((e, p.0));
    }
    res
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::core::components::maths::hierarchy::{Children, Parent};
    use crate::core::world::World;

    #[test]
    fn children_manager_system_test_children_delete() {
        let mut world = GameData::default();

        let parent = world.push((1,));
        let child = world.push((Parent(parent),));

        // First we test that the parent has no Children component
        assert_eq!(true, world.entry::<&Children>(parent).unwrap().get().is_none());

        // Then we execute the system and test that we have the good result
        children_manager_system(&mut world);

        assert_eq!(true, world.entry::<&Children>(parent).unwrap().get().is_some());

        // Finally we delete the parent entity and check that after a schedule, the child entity is also deleted
        let _r = world.remove(parent);
        children_manager_system(&mut world);

        assert_eq!(true, world.entry::<&Parent>(child).is_err());
    }

    #[test]
    fn children_manager_system_test_parent_clean() {
        let mut world = GameData::default();

        let child = world.push((1,));
        let parent = world.push((2, Children(vec![child])));

        // We check that we have the child
        assert_eq!(
            true,
            world.entry::<&Children>(parent).unwrap().get().unwrap().0.contains(&child)
        );
        let _r = world.remove(child);

        // we delete the child and then we execute the schedule and test that we have the good result
        children_manager_system(&mut world);

        assert_eq!(0, world.entry::<&Children>(parent).unwrap().get().unwrap().0.len());
    }
}
