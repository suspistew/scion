use legion::{system, systems::CommandBuffer, world::{EntityAccessError, SubWorld}, Entity, Query};

use crate::core::components::maths::hierarchy::{Children, Parent};

/// System responsible to add/modify Children components to the entities referenced by a Parent component
#[system]
pub(crate) fn children_manager(
    world: &mut SubWorld,
    cmd: &mut CommandBuffer,
    query_parent: &mut Query<(Entity, &mut Parent)>,
    query_children: &mut Query<(Entity, Option<&mut Children>)>,
) {
    let (mut w1, mut w2) = world.split_for_query(query_parent);
    query_parent.for_each_mut(&mut w1, |(entity, parent)|{
        match query_children.get_mut(&mut w2, parent.0) {
            Ok((_, children_component)) => {
                if let Some(children_component) = children_component{
                    if !children_component.0.contains(entity) {
                        children_component.0.push(*entity);
                    }
                }else{
                    cmd.add_component(parent.0, Children(vec![*entity]))
                }
            }
            Err(e) => {
                match e {
                    EntityAccessError::AccessDenied => {
                        cmd.add_component(parent.0, Children(vec![*entity]))
                    }
                    EntityAccessError::EntityNotFound => {
                        cmd.remove(*entity);
                    }
                };
            }
        }
    });

}

#[cfg(test)]
mod tests {
    use legion::{Entity, IntoQuery, Resources, Schedule, World};

    use super::*;
    use crate::core::components::maths::hierarchy::{Children, Parent};

    #[test]
    fn children_manager_system_test() {
        let mut world = World::default();
        let mut resources = Resources::default();
        let mut schedule = Schedule::builder()
            .add_system(children_manager_system())
            .build();

        let parent = world.push((1,));
        let child = world.push((Parent(parent),));
        let mut query = <(Entity, &Children)>::query();

        // First we test that the parent has no Children component
        assert_eq!(true, query.get(&world, parent).is_err());

        // Then we execute the schedule and test that we have the good result
        schedule.execute(&mut world, &mut resources);
        assert_eq!(true, query.get(&mut world, parent).is_ok());

        // Finally we delete the parent entity and check that after a schedule, the child entity is also deleted
        world.remove(parent);
        schedule.execute(&mut world, &mut resources);
        assert_eq!(true, world.entry(child).is_none());
    }
}
