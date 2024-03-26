use hecs::{Component, Entity};
use std::{cfg, collections::HashMap, ops::Range, path::Path, time::SystemTime};
use std::num::NonZeroU64;
use log::info;

use wgpu::{BindGroup, BindGroupLayout, Buffer, CommandEncoder, Device, Queue, RenderPassColorAttachment, RenderPipeline, SamplerBindingType, StoreOp, SurfaceConfiguration, TextureFormat, TextureView, util::DeviceExt};
use wgpu::util::BufferInitDescriptor;

use crate::core::world::{GameData, World};
use crate::graphics::rendering::gl_representations::{TexturedGlVertex, TexturedGlVertexWithLayer};
use crate::{
    config::scion_config::ScionConfig,
    core::components::{
        color::Color,
        Hide,
        HidePropagated,
        material::{Material, Texture},
        maths::{camera::Camera, transform::Transform},
        shapes::{
            line::Line, polygon::Polygon, rectangle::Rectangle, square::Square, triangle::Triangle,
        },
        tiles::{
            sprite::Sprite,
            tilemap::{Tile, Tilemap},
        }, ui::{ui_image::UiImage, ui_text::UiTextImage, UiComponent},
    },
    graphics::rendering::{
        gl_representations::{GlUniform, UniformData},
        Renderable2D,
        RenderableUi, ScionRenderer, shaders::pipeline::pipeline,
    },
    utils::file::{FileReaderError, read_file_modification_time},
};
use crate::core::components::material::TextureArray;
use crate::graphics::rendering::rendering_texture_management::load_texture_array_to_queue;
use crate::graphics::rendering::{DiffuseBindGroupUpdate, RendererType, RenderingInfos, RenderingUpdate};
use crate::graphics::rendering::shaders::pipeline::pipeline_sprite;
use crate::utils::maths::Vector;

#[derive(Default)]
pub(crate) struct Scion2D {
    vertex_buffers: HashMap<Entity, Buffer>,
    index_buffers: HashMap<Entity, Buffer>,
    render_pipelines: HashMap<String, RenderPipeline>,
    texture_bind_group_layout: Option<BindGroupLayout>,
    texture_array_bind_group_layout: Option<BindGroupLayout>,
    transform_bind_group_layout: Option<BindGroupLayout>,
    diffuse_bind_groups: HashMap<String, (BindGroup, wgpu::Texture)>,
    transform_uniform_bind_groups: HashMap<Entity, (GlUniform, Buffer, BindGroup)>,
    assets_timestamps: HashMap<String, SystemTime>,
    first_tick_passed: bool,
}

impl ScionRenderer for Scion2D {
    fn start(&mut self, device: &Device, surface_config: &SurfaceConfiguration) {
        self.transform_bind_group_layout = Some(Self::create_uniform_bind_group_layout(device));
        self.texture_bind_group_layout = Some(Self::create_texture_bind_group_layout(device));
        self.texture_array_bind_group_layout = Some(Self::create_texture_array_bind_group_layout(device));
        self.insert_components_pipelines::<Triangle>(&device, &surface_config);
        self.insert_components_pipelines::<Square>(&device, &surface_config);
        self.insert_components_pipelines::<Rectangle>(&device, &surface_config);
        self.insert_components_pipelines::<Sprite>(&device, &surface_config);
        self.insert_components_pipelines::<Line>(&device, &surface_config);
        self.insert_components_pipelines::<Polygon>(&device, &surface_config);
        self.insert_components_pipelines::<UiImage>(&device, &surface_config);
        self.insert_components_pipelines::<UiTextImage>(&device, &surface_config);
        self.insert_components_pipelines::<Tilemap>(&device, &surface_config);
    }

    fn update(
        &mut self,
        mut data: Vec<RenderingUpdate>,
        device: &Device,
        surface_config: &SurfaceConfiguration,
        queue: &mut Queue,
    ) {
        for update in data.drain(0..data.len()){
            match update{
                RenderingUpdate::DiffuseBindGroup { data, path } => {
                    self.update_material(device, queue, data, path);
                }
                RenderingUpdate::TransformUniform { entity, uniform } => {
                    self.update_transform_uniform(device, queue, entity, uniform);
                }
                RenderingUpdate::VertexBuffer { entity, label, contents, usage } => {
                    info!("vertex buffer to render {:?}", entity);
                    let vertex_buffer =
                        device.create_buffer_init(&BufferInitDescriptor{
                            label: Some("Vertex buffer"),
                            contents: contents.as_slice(),
                            usage,
                        });
                    self.vertex_buffers.insert(entity, vertex_buffer);
                }
                RenderingUpdate::IndexBuffer { entity, label, contents, usage } => {
                    info!("index buffer to render {:?}", entity);
                    let index_buffer =
                        device.create_buffer_init(&BufferInitDescriptor{
                            label: Some("Index buffer"),
                            contents: contents.as_slice(),
                            usage,
                        });
                    self.index_buffers.insert(entity, index_buffer);
                }
                RenderingUpdate::TilemapBuffer => {}
                RenderingUpdate::UiComponentBuffer => {}
            }
        }

        // FIXME : self.clean_buffers(data);
    }

    fn render(
        &mut self,
        data: Vec<RenderingInfos>,
        default_background: &Option<Color>,
        texture_view: TextureView,
        encoder: &mut CommandEncoder,
    ) {
        self.render_component(default_background, texture_view, encoder, data);
    }
}

impl Scion2D {
    fn insert_components_pipelines<T: Component + Renderable2D>(
        &mut self,
        device: &&Device,
        surface_config: &&SurfaceConfiguration,
    ) {
        self.insert_pipeline_if_not_finded::<T>(device, surface_config);
    }

    fn upsert_component_buffers<T: Component + Renderable2D>(
        &mut self,
        data: &mut GameData,
        device: &&Device,
    ) {
        for (entity, (component, material, _)) in
        data.query_mut::<(&mut T, &Material, &Transform)>()
        {
            if !self.vertex_buffers.contains_key(&entity) || component.dirty() {
                let vertex_buffer =
                    device.create_buffer_init(&component.vertex_buffer_descriptor(Some(material)));
                self.vertex_buffers.insert(entity, vertex_buffer);
            }

            if !self.index_buffers.contains_key(&entity) || component.dirty() {
                let index_buffer =
                    device.create_buffer_init(&component.indexes_buffer_descriptor());
                self.index_buffers.insert(entity, index_buffer);
            }

            component.set_dirty(false);
        }
    }

    fn upsert_tilemaps_buffers(&mut self, data: &mut GameData, device: &&Device) {
        let mut to_modify: Vec<(Entity, [TexturedGlVertexWithLayer; 4])> = Vec::new();

        for (entity, (_, material, _)) in
        data.query::<(&mut Tilemap, &Material, &Transform)>().iter()
        {
            let tile_size = Material::tile_size(material).expect("");
            let mut vertexes = Vec::new();
            let mut position = 0;
            let mut indexes = Vec::new();
            let any_tile_modified = !self.vertex_buffers.contains_key(&entity)
                || data
                .query::<(&Tile, &Sprite)>()
                .iter()
                .filter(|(_, (tile, sprite))| tile.tilemap == entity && sprite.dirty())
                .count()
                > 0;

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
                let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("TileMap Vertex Buffer"),
                    contents: bytemuck::cast_slice(vertexes.as_slice()),
                    usage: wgpu::BufferUsages::VERTEX,
                });

                self.vertex_buffers.insert(entity, buffer);

                let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("TileMap Index Buffer"),
                    contents: bytemuck::cast_slice(&indexes),
                    usage: wgpu::BufferUsages::INDEX,
                });
                self.index_buffers.insert(entity, index_buffer);
            }
        }

        for (e, vertexes) in to_modify.drain(0..) {
            let mut sprite = data.entry_mut::<&mut Sprite>(e).expect("");
            sprite.set_dirty(false);
            sprite.set_content(vertexes);
        }
    }

    fn upsert_ui_component_buffers<T: Component + Renderable2D + RenderableUi>(
        &mut self,
        data: &mut GameData,
        device: &&Device,
        _surface_config: &&SurfaceConfiguration,
        _queue: &mut Queue,
    ) {
        for (entity, (component, _, m)) in data.query::<(&mut T, &Transform, Option<&Material>)>().iter() {
            self.vertex_buffers.entry(entity).or_insert_with(|| {
                let vertex_buffer =
                    device.create_buffer_init(&component.vertex_buffer_descriptor(m));
                vertex_buffer
            });

            self.index_buffers.entry(entity).or_insert_with(|| {
                let index_buffer =
                    device.create_buffer_init(&component.indexes_buffer_descriptor());
                index_buffer
            });
        }
    }

    fn insert_pipeline_if_not_finded<T: Component + Renderable2D>(
        &mut self,
        device: &&Device,
        surface_config: &&SurfaceConfiguration,
    ) {
        let type_name = std::any::type_name::<T>();
        if !self.render_pipelines.contains_key(type_name) {
            self.render_pipelines.insert(
                type_name.to_string(),
                if type_name.eq_ignore_ascii_case("scion::core::components::tiles::sprite::Sprite") ||
                    type_name.eq_ignore_ascii_case("scion::core::components::tiles::tilemap::Tilemap") {
                    pipeline_sprite(
                        device,
                        surface_config,
                        self.texture_array_bind_group_layout.as_ref().unwrap(),
                        self.transform_bind_group_layout.as_ref().unwrap(),
                        T::topology(),
                    )
                } else {
                    pipeline(
                        device,
                        surface_config,
                        self.texture_bind_group_layout.as_ref().unwrap(),
                        self.transform_bind_group_layout.as_ref().unwrap(),
                        T::topology(),
                    )
                }
                ,
            );
        }
    }

    fn render_component(
        &mut self,
        default_background: &Option<Color>,
        texture_view: TextureView,
        encoder: &mut CommandEncoder,
        mut infos: Vec<RenderingInfos>,
    ) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[get_default_color_attachment(&texture_view, default_background)],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });

        while let Some(rendering_infos) = infos.pop() {
            render_pass.set_bind_group(
                0,
                &self.transform_uniform_bind_groups.get(&rendering_infos.entity).unwrap().2,
                &[],
            );
            info!("trying to render {:?}", &rendering_infos.entity);
            render_pass.set_vertex_buffer(
                0,
                self.vertex_buffers.get(&rendering_infos.entity).as_ref().unwrap().slice(..),
            );
            render_pass.set_index_buffer(
                self.index_buffers.get(&rendering_infos.entity).as_ref().unwrap().slice(..),
                wgpu::IndexFormat::Uint16,
            );
            render_pass.set_pipeline(
                self.render_pipelines.get(rendering_infos.type_name.as_str()).as_ref().unwrap(),
            );
            if let Some(path) = rendering_infos.texture_path {
                render_pass.set_bind_group(
                    1,
                    &self.diffuse_bind_groups.get(path.as_str()).unwrap().0,
                    &[],
                );
            }

            render_pass.draw_indexed(rendering_infos.range, 0, 0..1);
        }
    }

    fn clean_buffers(&mut self, data: &mut GameData) {
        self.vertex_buffers.retain(|&k, _| data.contains(k));
        self.index_buffers.retain(|&k, _| data.contains(k));
        self.transform_uniform_bind_groups.retain(|&k, _| data.contains(k));
    }

    fn update_material(&mut self, device: &Device, queue: &mut Queue, data: DiffuseBindGroupUpdate, path: String) {
        match data {
            DiffuseBindGroupUpdate::ColorBindGroup(tex) => {
                self.diffuse_bind_groups.insert(
                    path,
                    load_texture_to_queue(
                        tex,
                        queue,
                        device,
                        self.texture_bind_group_layout.as_ref().unwrap(),
                    ),
                );
            }
            DiffuseBindGroupUpdate::TextureBindGroup(tex) => {
                if self.diffuse_bind_groups.contains_key(path.as_str()) {
                    self.diffuse_bind_groups
                        .get(path.as_str())
                        .expect("Unreachable diffuse bind group after check")
                        .1.destroy();
                    self.diffuse_bind_groups.remove(path.as_str());
                }
                self.diffuse_bind_groups.insert(
                    path,
                    load_texture_to_queue(
                        tex,
                        queue,
                        device,
                        self.texture_bind_group_layout.as_ref().unwrap(),
                    ),
                );
            }
            DiffuseBindGroupUpdate::TilesetBindGroup(texture_array) => {
                self.diffuse_bind_groups.insert(
                    path,
                    load_texture_array_to_queue(texture_array, queue, device),
                );
            }
        }
    }

    fn update_transform_uniform(&mut self, device: &Device, queue: &mut Queue, entity: Entity, uniform: GlUniform) {
        if let std::collections::hash_map::Entry::Vacant(e) = self.transform_uniform_bind_groups.entry(entity) {
            let (uniform, uniform_buffer, group) = create_transform_uniform_bind_group(
                device,
                uniform,
                self.transform_bind_group_layout.as_ref().unwrap()
            );
            queue.write_buffer(&uniform_buffer, 0, bytemuck::cast_slice(&[uniform]));
            e.insert((uniform, uniform_buffer, group));
        } else {
            let (current_uniform, uniform_buffer, _) = self
                .transform_uniform_bind_groups
                .get_mut(&entity)
                .expect("Fatal error, a transform has been marked as found but doesn't exist");
            current_uniform.replace_with(uniform);
            queue.write_buffer(uniform_buffer, 0, bytemuck::cast_slice(&[*current_uniform]));
        }
    }

    fn create_uniform_bind_group_layout(device: &Device) -> BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("uniform_bind_group_layout"),
        })
    }

    fn create_texture_bind_group_layout(device: &Device) -> BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            label: Some("texture_bind_group_layout"),
        })
    }

    fn create_texture_array_bind_group_layout(device: &Device) -> BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2Array,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            label: Some("texture_bind_group_layout"),
        })
    }
}

fn load_texture_to_queue(
    texture: Texture,
    queue: &mut Queue,
    device: &Device,
    texture_bind_group_layout: &BindGroupLayout,
) -> (BindGroup, wgpu::Texture) {
    let texture_size = wgpu::Extent3d {
        width: texture.width,
        height: texture.height,
        depth_or_array_layers: 1,
    };

    let diffuse_texture = device.create_texture(&wgpu::TextureDescriptor {
        size: texture_size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        label: Some("diffuse_texture"),
        format: TextureFormat::Rgba8UnormSrgb,
        view_formats: &[TextureFormat::Rgba8UnormSrgb],
    });

    queue.write_texture(
        wgpu::ImageCopyTexture {
            texture: &diffuse_texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        &texture.bytes,
        wgpu::ImageDataLayout {
            offset: 0,
            bytes_per_row: Some(4 * &texture.width),
            rows_per_image: Some(texture.height),
        },
        texture_size,
    );
    let diffuse_texture_view = diffuse_texture.create_view(&wgpu::TextureViewDescriptor::default());
    let diffuse_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Nearest,
        min_filter: wgpu::FilterMode::Nearest,
        mipmap_filter: wgpu::FilterMode::Nearest,
        ..Default::default()
    });

    let diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: texture_bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&diffuse_texture_view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&diffuse_sampler),
            },
        ],
        label: Some("diffuse_bind_group"),
    });

    (diffuse_bind_group, diffuse_texture)
}

fn create_transform_uniform_bind_group(
    device: &Device,
    gl_uniform: GlUniform,
    uniform_bind_group_layout: &BindGroupLayout,
) -> (GlUniform, Buffer, BindGroup) {
    let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Uniform Buffer"),
        contents: bytemuck::cast_slice(&[gl_uniform]),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: uniform_bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: uniform_buffer.as_entire_binding(),
        }],
        label: Some("uniform_bind_group"),
    });

    (gl_uniform, uniform_buffer, uniform_bind_group)
}

fn get_default_color_attachment<'a>(
    texture_view: &'a TextureView,
    default_background: &'a Option<Color>,
) -> Option<RenderPassColorAttachment<'a>> {
    Some(RenderPassColorAttachment {
        view: texture_view,
        resolve_target: None,
        ops: wgpu::Operations {
            load: wgpu::LoadOp::Clear(
                if let Some(color) = &default_background {
                    color.to_linear()
                } else {
                    wgpu::Color { r: 1., g: 0., b: 0., a: 1.0 }
                },
            ),
            store: StoreOp::Store,
        },
    })
}

fn get_path_from_color(color: &Color) -> String {
    format!("color-{}-{}-{}-{}", color.red(), color.green(), color.blue(), color.alpha())
}

fn world_contains_camera(data: &mut GameData) -> bool {
    data.query::<&Camera>().iter().count() > 0
}
