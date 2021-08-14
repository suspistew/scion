use legion::systems::CommandBuffer;

use crate::{
    core::components::{
        maths::hierarchy::{Children, Parent},
        Hide, HidePropagated,
    },
    legion::{world::SubWorld, Entity, EntityStore, Query, *},
};

#[system(for_each)]
#[read_component(Entity)]
#[write_component(HidePropagated)]
pub(crate) fn hide_propagation(
    cmd: &mut CommandBuffer,
    world: &mut SubWorld,
    _h: &Hide,
    children: &Children,
) {
    children.0.iter().for_each(|child| {
        let child_entry =
            world.entry_ref(*child).expect("Unreachable child during hide propagation");
        if child_entry.get_component::<HidePropagated>().is_err() {
            cmd.add_component(*child, HidePropagated);
        }
    });
}

#[system(for_each)]
#[read_component(Entity)]
#[read_component(Hide)]
pub(crate) fn hide_propagated_deletion(
    cmd: &mut CommandBuffer,
    world: &mut SubWorld,
    entity: &Entity,
    _h: &HidePropagated,
    parent: &Parent,
) {
    let parent_entry =
        world.entry_ref(parent.0).expect("Unreachable parent during hide propagated deletion");
    if parent_entry.get_component::<Hide>().is_err() {
        cmd.remove_component::<HidePropagated>(*entity);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        core::components::{
            maths::hierarchy::{Children, Parent},
            Hide, HidePropagated,
        },
        legion::{EntityStore, Resources, Schedule, World},
        *,
    };

    #[test]
    fn hide_propagation_test() {
        let mut world = World::default();
        let mut resources = Resources::default();
        let mut schedule = Schedule::builder().add_system(hide_propagation_system()).build();

        let child = world.push((2,));
        let parent = world.push((1, Hide, Children(vec![child])));

        assert_eq!(true, world.entry(child).unwrap().get_component::<HidePropagated>().is_err());

        schedule.execute(&mut world, &mut resources);

        assert_eq!(true, world.entry(child).unwrap().get_component::<HidePropagated>().is_ok());
    }

    #[test]
    fn hide_propagated_deletion_test() {
        let mut world = World::default();
        let mut resources = Resources::default();
        let mut schedule =
            Schedule::builder().add_system(hide_propagated_deletion_system()).build();

        let parent = world.push((1,));
        let child = world.push((2, HidePropagated, Parent(parent)));

        assert_eq!(true, world.entry(child).unwrap().get_component::<HidePropagated>().is_ok());

        schedule.execute(&mut world, &mut resources);

        assert_eq!(true, world.entry(child).unwrap().get_component::<HidePropagated>().is_err());
    }
}
