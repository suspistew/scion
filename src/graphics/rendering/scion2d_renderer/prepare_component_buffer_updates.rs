use hecs::{Component, QueryMut};
use log::info;
use crate::core::components::{Square, Triangle};
use crate::core::components::material::Material;
use crate::core::components::maths::transform::Transform;
use crate::core::components::shapes::line::Line;
use crate::core::components::shapes::polygon::Polygon;
use crate::core::components::shapes::rectangle::Rectangle;
use crate::core::components::tiles::sprite::Sprite;
use crate::core::world::{GameData, World};
use crate::graphics::rendering::{Renderable2D, RenderableUi, RenderingUpdate};
use crate::graphics::rendering::scion2d_renderer::scion_renderer::ScionRenderer2D;

pub(crate) fn call(renderer: &mut ScionRenderer2D, data: &mut GameData) -> Vec<RenderingUpdate> {
    let mut updates = vec![];
    updates.append(&mut prepare_buffer_update_for_component::<Triangle>(renderer, data));
    updates.append(&mut prepare_buffer_update_for_component::<Square>(renderer, data));
    updates.append(&mut prepare_buffer_update_for_component::<Rectangle>(renderer, data));
    updates.append(&mut prepare_buffer_update_for_component::<Sprite>(renderer, data));
    updates.append(&mut prepare_buffer_update_for_component::<Line>(renderer, data));
    updates.append(&mut prepare_buffer_update_for_component::<Polygon>(renderer, data));
    updates
}

fn prepare_buffer_update_for_component<T: Component + Renderable2D>(
    renderer: &mut ScionRenderer2D,
    data: &mut GameData) -> Vec<RenderingUpdate> {
    let mut updates = vec![];
    for (entity, (component, material, _)) in data.query_mut::<(&mut T, &Material, &Transform)>() {
        if renderer.missing_vertex_buffer(&entity) || component.dirty() {
            info!("Adding vertex {:?}", entity);
            let descriptor = component.vertex_buffer_descriptor(Some(material));
            updates.push(RenderingUpdate::VertexBuffer {
                entity,
                label: descriptor.label.unwrap().to_string(),
                contents: descriptor.contents.to_vec(), // TODO
                usage: descriptor.usage,
            });
            renderer.upsert_vertex_buffer(entity);
        }

        if renderer.missing_indexes_buffer(&entity) || component.dirty() {
            info!("Adding index {:?}", entity);
            let descriptor = component.indexes_buffer_descriptor();
            updates.push(RenderingUpdate::IndexBuffer {
                entity,
                label: descriptor.label.unwrap().to_string(),
                contents: descriptor.contents.to_vec(), // TODO
                usage: descriptor.usage,
            });
        }

        component.set_dirty(false);
    }
    updates
}

fn prepare_buffer_update_for_ui_component<T: Component + Renderable2D + RenderableUi>(
    renderer: &mut ScionRenderer2D,
    data: &mut GameData) -> Vec<RenderingUpdate> {
    let mut updates = vec![];
}
