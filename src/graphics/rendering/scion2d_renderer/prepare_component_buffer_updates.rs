use hecs::{Component, Entity};
use wgpu::BufferUsages;

use crate::core::components::{Square, Triangle};
use crate::core::components::material::Material;
use crate::core::components::maths::transform::Transform;
use crate::core::components::shapes::line::Line;
use crate::core::components::shapes::polygon::Polygon;
use crate::core::components::shapes::rectangle::Rectangle;
use crate::core::components::tiles::sprite::Sprite;
use crate::core::components::tiles::tilemap::{Tile, Tilemap};
use crate::core::components::ui::ui_image::UiImage;
use crate::core::components::ui::ui_text::UiTextImage;
use crate::core::world::{GameData, World};
use crate::graphics::rendering::{Renderable2D, RenderableUi, RenderingUpdate};
use crate::graphics::rendering::gl_representations::TexturedGlVertexWithLayer;
use crate::graphics::rendering::scion2d_renderer::scion_renderer::ScionRenderer2D;

pub(crate) fn call(renderer: &mut ScionRenderer2D, data: &mut GameData) -> Vec<RenderingUpdate> {
    let mut updates = vec![];
    updates.append(&mut prepare_buffer_update_for_component::<Triangle>(renderer, data));
    updates.append(&mut prepare_buffer_update_for_component::<Square>(renderer, data));
    updates.append(&mut prepare_buffer_update_for_component::<Rectangle>(renderer, data));
    updates.append(&mut prepare_buffer_update_for_component::<Sprite>(renderer, data));
    updates.append(&mut prepare_buffer_update_for_component::<Line>(renderer, data));
    updates.append(&mut prepare_buffer_update_for_component::<Polygon>(renderer, data));
    updates.append(&mut prepare_buffer_update_for_ui_component::<UiImage>(renderer, data));
    updates.append(&mut prepare_buffer_update_for_ui_component::<UiTextImage>(renderer, data));
    updates.append(&mut prepare_buffer_update_for_tilemap(renderer, data));
    updates
}

fn prepare_buffer_update_for_component<T: Component + Renderable2D>(
    renderer: &mut ScionRenderer2D,
    data: &mut GameData) -> Vec<RenderingUpdate> {
    let mut updates = vec![];
    for (entity, (component, material, _)) in data.query_mut::<(&mut T, &Material, &Transform)>() {
        if renderer.missing_vertex_buffer(&entity) || component.dirty() {
            let descriptor = component.vertex_buffer_descriptor(Some(material));
            updates.push(RenderingUpdate::VertexBuffer {
                entity,
                contents: descriptor.contents.to_vec(), // TODO
                usage: descriptor.usage,
            });
            renderer.upsert_vertex_buffer(entity);
        }

        if renderer.missing_indexes_buffer(&entity) || component.dirty() {
            let descriptor = component.indexes_buffer_descriptor();
            updates.push(RenderingUpdate::IndexBuffer {
                entity,
                contents: descriptor.contents.to_vec(), // TODO
                usage: descriptor.usage,
            });
            renderer.upsert_indexes_buffer(entity);
        }

        component.set_dirty(false);
    }
    updates
}

fn prepare_buffer_update_for_ui_component<T: Component + Renderable2D + RenderableUi>(
    renderer: &mut ScionRenderer2D,
    data: &mut GameData) -> Vec<RenderingUpdate> {
    let mut updates = vec![];
    for (entity, (component, _, m)) in data.query::<(&mut T, &Transform, Option<&Material>)>().iter() {
        if renderer.missing_vertex_buffer(&entity){
            let descriptor = component.vertex_buffer_descriptor(m);
            updates.push(RenderingUpdate::VertexBuffer {
                entity,
                contents: descriptor.contents.to_vec(), // TODO
                usage: descriptor.usage,
            });
            renderer.upsert_vertex_buffer(entity);
        }
        if renderer.missing_indexes_buffer(&entity){
            let descriptor = component.indexes_buffer_descriptor();
            updates.push(RenderingUpdate::IndexBuffer {
                entity,
                contents: descriptor.contents.to_vec(), // TODO
                usage: descriptor.usage,
            });
            renderer.upsert_indexes_buffer(entity);
        }
    }
    updates
}

fn prepare_buffer_update_for_tilemap(renderer: &mut ScionRenderer2D, data: &mut GameData) -> Vec<RenderingUpdate> {
    let mut updates = vec![];
    {
        let mut to_modify: Vec<(Entity, [TexturedGlVertexWithLayer; 4])> = Vec::new();
        for (entity, (_, material, _)) in data.query::<(&mut Tilemap, &Material, &Transform)>().iter() {
            let tile_size = Material::tile_size(material).expect("");
            let mut position = 0;
            let mut vertexes = Vec::new();
            let mut indexes = Vec::new();

            let any_tile_modified = renderer.missing_vertex_buffer(&entity) || any_dirty_sprite(data, entity);
            if any_tile_modified {
                for (e, (tile, sprite)) in data.query::<(&Tile, &Sprite)>().iter() {
                    if tile.tilemap == entity {
                        let current_vertex = sprite.compute_content(Some(material));
                        to_modify.push((e, current_vertex));
                        let mut vec = current_vertex.to_vec();
                        vec.iter_mut().for_each(|gl_vertex| {
                            gl_vertex.position[0] = gl_vertex.position[0] + tile_size as f32 * tile.position.x() as f32;
                            gl_vertex.position[1] = gl_vertex.position[1] + tile_size as f32 * tile.position.y() as f32;
                            gl_vertex.position[2] = gl_vertex.position[2] + tile.position.z() as f32 / 100.
                        });
                        vertexes.append(&mut vec);
                        let sprite_indexes = Sprite::indices();
                        let mut sprite_indexes: Vec<u16> = sprite_indexes
                            .iter()
                            .map(|indice| (*indice as usize + (position * 4)) as u16)
                            .collect();
                        indexes.append(&mut sprite_indexes);
                        position += 1;
                    }
                }
                let bytes_vertexes: &[u8] = bytemuck::cast_slice(vertexes.as_slice());
                updates.push(RenderingUpdate::VertexBuffer {
                    entity,
                    contents: bytes_vertexes.to_vec(),
                    usage: BufferUsages::VERTEX
                });
                renderer.upsert_vertex_buffer(entity);

                let bytes_indexes: &[u8] = bytemuck::cast_slice(indexes.as_slice());
                updates.push(RenderingUpdate::IndexBuffer {
                    entity,
                    contents: bytes_indexes.to_vec(), // TODO
                    usage: BufferUsages::INDEX,
                });
                renderer.upsert_indexes_buffer(entity);
            }
        }

        for (e, vertexes) in to_modify.drain(0..) {
            let sprite = data.entry_mut::<&mut Sprite>(e).expect("");
            sprite.set_dirty(false);
            sprite.set_content(vertexes);
        }
    }
    updates
}

fn any_dirty_sprite(data: &GameData, entity: Entity) -> bool {
    data
        .query::<(&Tile, &Sprite)>()
        .iter()
        .filter(|(_, (tile, sprite))| tile.tilemap == entity && sprite.dirty())
        .count()
        > 0
}
