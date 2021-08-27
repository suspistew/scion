use crate::{
    core::{
        components::maths::{
            camera::{Camera, DefaultCamera},
            transform::Transform,
        },
        resources::window::Window,
    },
    legion::{systems::CommandBuffer, *},
};

/// System responsible of adding a Camera on each entity with a DefaultCamera component
#[system(for_each)]
pub(crate) fn default_camera(
    cmd: &mut CommandBuffer,
    #[resource] window_dimension: &Window,
    _c: &DefaultCamera,
    entity: &Entity,
) {
    cmd.remove_component::<DefaultCamera>(*entity);
    cmd.add_component(
        *entity,
        Camera::new(window_dimension.width() as f32, window_dimension.height() as f32),
    );
    cmd.add_component(*entity, Transform::default());
}
