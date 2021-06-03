use legion::{
    component, storage::Component, system, systems::CommandBuffer, world::SubWorld, Entity,
    IntoQuery,
};

use crate::core::components::ui::UiComponent;

/// System responsible to add the UiComponent to any T missing its uiComponent
#[system]
#[write_component(T)]
pub(crate) fn missing_ui_component<T: Component>(world: &mut SubWorld, cmd: &mut CommandBuffer) {
    <(Entity, &T)>::query()
        .filter(!component::<UiComponent>())
        .for_each(world, |(e, _t)| {
            cmd.add_component(*e, UiComponent);
        });
}

#[cfg(test)]
mod tests {
    use legion::{Resources, Schedule, World};

    use super::*;
    use crate::core::components::ui::{ui_image::UiImage, UiComponent};

    #[test]
    fn missing_ui_comp_system_test() {
        let mut world = World::default();
        let mut resources = Resources::default();
        let mut schedule = Schedule::builder()
            .add_system(missing_ui_component_system::<UiImage>())
            .build();

        let e = world.push((UiImage::new(1., 1., "".to_string()),));

        let entry = world.entry(e).unwrap();
        let res = entry.get_component::<UiComponent>();
        assert_eq!(true, res.is_err());

        schedule.execute(&mut world, &mut resources);

        let entry = world.entry(e).unwrap();
        let res = entry.get_component::<UiComponent>();
        assert_eq!(true, res.is_ok());
    }
}
