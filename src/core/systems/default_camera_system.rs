use crate::legion::*;
use crate::core::components::maths::camera::{DefaultCamera, Camera};
use crate::core::resources::window::WindowDimensions;
use crate::legion::systems::CommandBuffer;
use crate::core::components::maths::transform::Transform;

#[system(for_each)]
pub(crate) fn default_camera(
    cmd: &mut CommandBuffer,
    #[resource] window_dimension: &WindowDimensions,
    _c: &DefaultCamera,
    entity: &Entity,
) {
    cmd.remove_component::<DefaultCamera>(*entity);
    cmd.add_component(*entity, Camera::new(window_dimension.width() as f32, window_dimension.height() as f32, 10.));
    cmd.add_component(*entity, Transform::default());
}