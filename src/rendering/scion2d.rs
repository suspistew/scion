use std::{cfg, collections::HashMap, ops::Range, path::Path, time::SystemTime};

use legion::{component, storage::Component, Entity, IntoQuery, Resources, World};
use wgpu::{
    util::DeviceExt, BindGroup, BindGroupLayout, Buffer, CommandEncoder, Device, Queue,
    RenderPassColorAttachment, RenderPipeline, SurfaceConfiguration, TextureView,
};

use crate::{
    config::scion_config::ScionConfig,
    core::{
        components::{
            color::Color,
            material::{Material, Texture},
            maths::{camera::Camera, transform::Transform},
            shapes::{
                line::Line, polygon::Polygon, rectangle::Rectangle, square::Square,
                triangle::Triangle,
            },
            tiles::{
                sprite::Sprite,
                tilemap::{Tile, Tilemap},
            },
            ui::{ui_image::UiImage, ui_text::UiTextImage, UiComponent},
            Hide, HidePropagated,
        },
        legion_ext::ScionResourcesExtension,
    },
    rendering::{
        gl_representations::{GlUniform, UniformData},
        shaders::pipeline::pipeline,
        Renderable2D, RenderableUi, ScionRenderer,
    },
    utils::file::{read_file_modification_time, FileReaderError},
};

#[derive(Default)]
pub(crate) struct Scion2D {
    vertex_buffers: HashMap<Entity, wgpu::Buffer>,
    index_buffers: HashMap<Entity, wgpu::Buffer>,
    render_pipelines: HashMap<String, RenderPipeline>,
    texture_bind_group_layout: Option<BindGroupLayout>,
    transform_bind_group_layout: Option<BindGroupLayout>,
    diffuse_bind_groups: HashMap<String, (BindGroup, wgpu::Texture)>,
    transform_uniform_bind_groups: HashMap<Entity, (GlUniform, Buffer, BindGroup)>,
    assets_timestamps: HashMap<String, SystemTime>,
}

struct RenderingInfos {
    layer: usize,
    range: Range<u32>,
    entity: Entity,
    texture_path: Option<String>,
    type_name: String,
}

impl ScionRenderer for Scion2D {
    fn start(&mut self, device: &Device, surface_config: &SurfaceConfiguration) {
        let uniform_bind_group_layout =
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
            });

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: false },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler { comparison: false, filtering: true },
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });

        self.transform_bind_group_layout = Some(uniform_bind_group_layout);
        self.texture_bind_group_layout = Some(texture_bind_group_layout);
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
        world: &mut World,
        resources: &mut Resources,
        device: &Device,
        surface_config: &SurfaceConfiguration,
        queue: &mut Queue,
    ) {
        if world_contains_camera(world) {
            self.update_diffuse_bind_groups(world, resources, device, queue);
            self.update_transforms(world, &device, queue);
            self.upsert_component_buffers::<Triangle>(world, &device);
            self.upsert_component_buffers::<Square>(world, &device);
            self.upsert_component_buffers::<Rectangle>(world, &device);
            self.upsert_component_buffers::<Sprite>(world, &device);
            self.upsert_component_buffers::<Line>(world, &device);
            self.upsert_component_buffers::<Polygon>(world, &device);
            self.upsert_tilemaps_buffers(world, &device);
            self.upsert_ui_component_buffers::<UiImage>(world, &device, &surface_config, queue);
            self.upsert_ui_component_buffers::<UiTextImage>(world, &device, &surface_config, queue);
        } else {
            log::warn!("No camera has been found in resources");
        }
        self.clean_buffers(world);
    }

    fn render(
        &mut self,
        world: &mut World,
        config: &ScionConfig,
        texture_view: &TextureView,
        encoder: &mut CommandEncoder,
    ) {
        {
            encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Scion 2D Render Pass"),
                color_attachments: &[get_default_color_attachment(texture_view, config)],
                depth_stencil_attachment: None,
            });
        }

        if world_contains_camera(world) {
            let mut rendering_infos = Vec::new();
            rendering_infos.append(&mut self.pre_render_component::<Triangle>(world));
            rendering_infos.append(&mut self.pre_render_component::<Square>(world));
            rendering_infos.append(&mut self.pre_render_component::<Rectangle>(world));
            rendering_infos.append(&mut self.pre_render_component::<Sprite>(world));
            rendering_infos.append(&mut self.pre_render_component::<Line>(world));
            rendering_infos.append(&mut self.pre_render_component::<Polygon>(world));
            rendering_infos.append(&mut self.pre_render_ui_component::<UiImage>(world));
            rendering_infos.append(&mut self.pre_render_ui_component::<UiTextImage>(world));
            rendering_infos.append(&mut self.pre_render_tilemaps(world));

            rendering_infos.sort_by(|a, b| b.layer.cmp(&a.layer));
            while let Some(info) = rendering_infos.pop() {
                self.render_component(texture_view, encoder, info);
            }
        }
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
        world: &mut World,
        device: &&Device,
    ) {
        for (entity, component, material, _) in
            <(Entity, &mut T, &Material, &Transform)>::query().iter_mut(world)
        {
            if !self.vertex_buffers.contains_key(entity) || component.dirty() {
                let vertex_buffer =
                    device.create_buffer_init(&component.vertex_buffer_descriptor(Some(material)));
                self.vertex_buffers.insert(*entity, vertex_buffer);
            }

            if !self.index_buffers.contains_key(entity) || component.dirty() {
                let index_buffer =
                    device.create_buffer_init(&component.indexes_buffer_descriptor());
                self.index_buffers.insert(*entity, index_buffer);
            }

            component.set_dirty(false);
        }
    }

    fn upsert_tilemaps_buffers(&mut self, world: &mut World, device: &&Device) {
        let mut tilemap_query = <(Entity, &mut Tilemap, &Material, &Transform)>::query();
        let (mut tilemap_world, mut tile_world) = world.split_for_query(&tilemap_query);

        let mut tiles: Vec<(&Tile, &mut Sprite)> =
            <(&Tile, &mut Sprite)>::query().iter_mut(&mut tile_world).collect();

        for (entity, _tilemap, material, _) in tilemap_query.iter_mut(&mut tilemap_world) {
            let tile_size = Material::tile_size(material).expect("");
            let mut vertexes = Vec::new();
            let mut position = 0;
            let mut indexes = Vec::new();
            let any_tile_modified = !self.vertex_buffers.contains_key(entity)
                || tiles
                    .iter_mut()
                    .filter(|(tile, sprite)| tile.tilemap == *entity && sprite.dirty())
                    .count()
                    > 0;
            if any_tile_modified {
                tiles.iter_mut().filter(|(tile, _sprite)| tile.tilemap == *entity).for_each(
                    |(tile, sprite)| {
                        let mut vec = sprite.upsert_content(Some(material)).to_vec();
                        vec.iter_mut().for_each(|gl_vertex| {
                            gl_vertex.position.append_position(
                                tile_size as f32 * tile.position.x() as f32,
                                tile_size as f32 * tile.position.y() as f32,
                                tile.position.z() as f32 / 100.,
                            )
                        });
                        vertexes.append(&mut vec);
                        let sprite_indexes = Sprite::indices();
                        let mut sprite_indexes: Vec<u16> = sprite_indexes
                            .iter()
                            .map(|indice| (*indice as usize + (position * 4)) as u16)
                            .collect();
                        indexes.append(&mut sprite_indexes);
                        position += 1;
                        sprite.set_dirty(false);
                    },
                );
                let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("TileMap Vertex Buffer"),
                    contents: bytemuck::cast_slice(vertexes.as_slice()),
                    usage: wgpu::BufferUsages::VERTEX,
                });

                self.vertex_buffers.insert(*entity, buffer);

                let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("TileMap Index Buffer"),
                    contents: bytemuck::cast_slice(&indexes),
                    usage: wgpu::BufferUsages::INDEX,
                });
                self.index_buffers.insert(*entity, index_buffer);
            }
        }
    }

    fn upsert_ui_component_buffers<T: Component + Renderable2D + RenderableUi>(
        &mut self,
        world: &mut World,
        device: &&Device,
        _surface_config: &&SurfaceConfiguration,
        queue: &mut Queue,
    ) {
        for (entity, component, _) in <(Entity, &mut T, &Transform)>::query().iter_mut(world) {
            if !self.vertex_buffers.contains_key(entity) {
                let vertex_buffer =
                    device.create_buffer_init(&component.vertex_buffer_descriptor(None));
                self.vertex_buffers.insert(*entity, vertex_buffer);
            }

            if !self.index_buffers.contains_key(entity) {
                let index_buffer =
                    device.create_buffer_init(&component.indexes_buffer_descriptor());
                self.index_buffers.insert(*entity, index_buffer);
            }
            if let Some(texture_path) = component.get_texture_path() {
                if !self.diffuse_bind_groups.contains_key(texture_path.as_str()) {
                    let path = Path::new(texture_path.as_str());
                    let loaded_texture = Texture::from_png(path);
                    self.diffuse_bind_groups.insert(
                        texture_path.clone(),
                        load_texture_to_queue(
                            &loaded_texture,
                            queue,
                            device,
                            self.texture_bind_group_layout.as_ref().unwrap(),
                        ),
                    );

                    let timestamp = read_file_modification_time(path);
                    if let Ok(timestamp) = timestamp {
                        self.assets_timestamps.insert(texture_path.clone(), timestamp);
                    }
                }
            }
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
                pipeline(
                    device,
                    surface_config,
                    self.texture_bind_group_layout.as_ref().unwrap(),
                    self.transform_bind_group_layout.as_ref().unwrap(),
                    T::topology(),
                ),
            );
        }
    }

    fn render_component(
        &mut self,
        texture_view: &TextureView,
        encoder: &mut CommandEncoder,
        rendering_infos: RenderingInfos,
    ) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Scion 2D Render Pass"),
            color_attachments: &[get_no_color_attachment(texture_view)],
            depth_stencil_attachment: None,
        });

        render_pass.set_bind_group(
            1,
            &self.transform_uniform_bind_groups.get(&rendering_infos.entity).unwrap().2,
            &[],
        );
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
                0,
                &self.diffuse_bind_groups.get(path.as_str()).unwrap().0,
                &[],
            );
        }

        render_pass.draw_indexed(rendering_infos.range, 0, 0..1);
    }

    fn pre_render_component<T: Component + Renderable2D>(
        &mut self,
        world: &mut World,
    ) -> Vec<RenderingInfos> {
        let type_name = std::any::type_name::<T>();
        let mut render_infos = Vec::new();
        for (entity, component, material, transform) in
            <(Entity, &mut T, &Material, &Transform)>::query()
                .filter(
                    !component::<Tile>() & !component::<Hide>() & !component::<HidePropagated>(),
                )
                .iter_mut(world)
        {
            let path = match material {
                Material::Color(color) => Some(get_path_from_color(&color)),
                Material::Texture(p) => Some(p.clone()),
                Material::Tileset(tileset) => Some(tileset.texture.clone()),
            };
            render_infos.push(RenderingInfos {
                layer: transform.translation().z(),
                range: component.range(),
                entity: *entity,
                texture_path: path,
                type_name: type_name.to_string(),
            });
        }
        render_infos
    }

    fn pre_render_tilemaps(&mut self, world: &mut World) -> Vec<RenderingInfos> {
        let type_name = std::any::type_name::<Tilemap>();
        let mut render_infos = Vec::new();

        let mut tilemap_query = <(Entity, &mut Tilemap, &Material, &Transform)>::query()
            .filter(!component::<Hide>() & !component::<HidePropagated>());
        let (mut tilemap_world, mut tile_world) = world.split_for_query(&tilemap_query);

        for (entity, _, material, transform) in tilemap_query.iter_mut(&mut tilemap_world) {
            let tiles_nb = <(&Tile, &mut Sprite)>::query()
                .iter_mut(&mut tile_world)
                .filter(|(tile, _sprite)| tile.tilemap == *entity)
                .count();
            let path = match material {
                Material::Tileset(tileset) => Some(tileset.texture.clone()),
                _ => None,
            };
            render_infos.push(RenderingInfos {
                layer: transform.translation().z(),
                range: 0..(tiles_nb * Sprite::indices().len()) as u32,
                entity: *entity,
                texture_path: path,
                type_name: type_name.to_string(),
            });
        }
        render_infos
    }

    fn pre_render_ui_component<T: Component + Renderable2D + RenderableUi>(
        &mut self,
        world: &mut World,
    ) -> Vec<RenderingInfos> {
        let type_name = std::any::type_name::<T>();
        let mut render_infos = Vec::new();
        for (entity, component, transform) in <(Entity, &mut T, &Transform)>::query()
            .filter(!component::<Hide>() & !component::<HidePropagated>())
            .iter_mut(world)
        {
            render_infos.push(RenderingInfos {
                layer: transform.translation().z(),
                range: component.range(),
                entity: *entity,
                texture_path: component.get_texture_path(),
                type_name: type_name.to_string(),
            });
        }
        render_infos
    }

    fn update_transforms(&mut self, world: &mut World, device: &&Device, queue: &mut Queue) {
        self.update_transforms_for_type::<Triangle>(world, &device, queue);
        self.update_transforms_for_type::<Square>(world, &device, queue);
        self.update_transforms_for_type::<Rectangle>(world, &device, queue);
        self.update_transforms_for_type::<Sprite>(world, &device, queue);
        self.update_transforms_for_type::<Line>(world, &device, queue);
        self.update_transforms_for_type::<Polygon>(world, &device, queue);
        self.update_transforms_for_type::<UiImage>(world, &device, queue);
        self.update_transforms_for_type::<UiTextImage>(world, &device, queue);
        self.update_transforms_for_type::<Tilemap>(world, &device, queue);
    }

    fn update_transforms_for_type<T: Component + Renderable2D>(
        &mut self,
        main_world: &mut World,
        device: &&Device,
        queue: &mut Queue,
    ) {
        let mut camera_query = <(&Camera, &Transform)>::query();
        let (camera_world, mut world) = main_world.split_for_query(&camera_query);
        let camera = camera_query
            .iter(&camera_world)
            .next()
            .expect("No camera has been found in the world after the security check");
        for (entity, transform, optional_ui_component, _) in
            <(Entity, &Transform, Option<&UiComponent>, &T)>::query().iter_mut(&mut world)
        {
            if !self.transform_uniform_bind_groups.contains_key(entity) {
                let (uniform, uniform_buffer, group) = create_transform_uniform_bind_group(
                    &device,
                    transform,
                    camera,
                    optional_ui_component.is_some(),
                    self.transform_bind_group_layout.as_ref().unwrap(),
                );
                queue.write_buffer(&uniform_buffer, 0, bytemuck::cast_slice(&[uniform]));
                self.transform_uniform_bind_groups
                    .insert(*entity, (uniform, uniform_buffer, group));
            } else {
                let (uniform, uniform_buffer, _) = self
                    .transform_uniform_bind_groups
                    .get_mut(entity)
                    .expect("Fatal error, a transform has been marked as found but doesn't exist");
                uniform.replace_with(GlUniform::from(UniformData {
                    transform,
                    camera,
                    is_ui_component: optional_ui_component.is_some(),
                }));
                queue.write_buffer(uniform_buffer, 0, bytemuck::cast_slice(&[*uniform]));
            }
        }
    }

    fn texture_should_be_reloaded(
        &self,
        path: &String,
        new_timestamp: &Option<Result<SystemTime, FileReaderError>>,
    ) -> bool {
        !self.diffuse_bind_groups.contains_key(path.as_str())
            || if let Some(Ok(timestamp)) = new_timestamp {
                !self.assets_timestamps.contains_key(path.as_str())
                    || !self.assets_timestamps.get(path.as_str()).unwrap().eq(timestamp)
            } else {
                false
            }
    }

    /// Loads in the queue materials that are not yet loaded.
    fn update_diffuse_bind_groups(
        &mut self,
        world: &mut World,
        resources: &mut Resources,
        device: &Device,
        queue: &mut Queue,
    ) {
        let hot_timer_cycle = if cfg!(feature = "hot-reload") {
            let mut timers = resources.timers();
            let hot_reload_timer =
                timers.get_timer("hot-reload-timer").expect("Missing mandatory timer : hot_reload");
            hot_reload_timer.cycle() > 0
        } else {
            false
        };

        <(Entity, &Material)>::query().for_each(world, |(_entity, material)| {
            match material {
                Material::Texture(texture_path) => {
                    let path = Path::new(texture_path.as_str());
                    let new_timestamp = if hot_timer_cycle
                        || !self.diffuse_bind_groups.contains_key(texture_path.as_str())
                    {
                        Some(read_file_modification_time(path))
                    } else {
                        None
                    };

                    if self.texture_should_be_reloaded(&texture_path, &new_timestamp) {
                        if self.diffuse_bind_groups.contains_key(texture_path.as_str()) {
                            self.diffuse_bind_groups
                                .get(texture_path.as_str())
                                .expect("Unreachable diffuse bind group after check")
                                .1
                                .destroy();
                            self.diffuse_bind_groups.remove(texture_path.as_str());
                        }

                        let loaded_texture = Texture::from_png(path);
                        self.diffuse_bind_groups.insert(
                            texture_path.clone(),
                            load_texture_to_queue(
                                &loaded_texture,
                                queue,
                                device,
                                self.texture_bind_group_layout.as_ref().unwrap(),
                            ),
                        );

                        if let Some(Ok(timestamp)) = new_timestamp {
                            self.assets_timestamps.insert(texture_path.clone(), timestamp);
                        }
                    }
                }
                Material::Color(color) => {
                    let path = get_path_from_color(&color);
                    if !self.diffuse_bind_groups.contains_key(path.as_str()) {
                        let loaded_texture = Texture::from_color(&color);
                        self.diffuse_bind_groups.insert(
                            path.clone(),
                            load_texture_to_queue(
                                &loaded_texture,
                                queue,
                                device,
                                self.texture_bind_group_layout.as_ref().unwrap(),
                            ),
                        );
                    }
                }
                Material::Tileset(tileset) => {
                    let path = Path::new(tileset.texture.as_str());
                    let new_timestamp = if hot_timer_cycle
                        || !self.diffuse_bind_groups.contains_key(tileset.texture.as_str())
                    {
                        Some(read_file_modification_time(path))
                    } else {
                        None
                    };

                    if self.texture_should_be_reloaded(&tileset.texture, &new_timestamp) {
                        let loaded_texture = Texture::from_png(Path::new(tileset.texture.as_str()));
                        self.diffuse_bind_groups.insert(
                            tileset.texture.clone(),
                            load_texture_to_queue(
                                &loaded_texture,
                                queue,
                                device,
                                self.texture_bind_group_layout.as_ref().unwrap(),
                            ),
                        );
                        if let Some(Ok(timestamp)) = new_timestamp {
                            self.assets_timestamps.insert(tileset.texture.clone(), timestamp);
                        }
                    }
                }
            }
        });
    }

    fn clean_buffers(&mut self, world: &mut World) {
        let entities: Vec<&Entity> = <Entity>::query().iter(world).collect();
        self.vertex_buffers.retain(|&k, _| entities.contains(&&k));
        self.index_buffers.retain(|&k, _| entities.contains(&&k));
        self.transform_uniform_bind_groups.retain(|&k, _| entities.contains(&&k));
    }
}

fn load_texture_to_queue(
    texture: &Texture,
    queue: &mut Queue,
    device: &Device,
    texture_bind_group_layout: &BindGroupLayout,
) -> (BindGroup, wgpu::Texture) {
    let texture_size = wgpu::Extent3d {
        width: texture.width as u32,
        height: texture.height as u32,
        depth_or_array_layers: 1,
    };

    let diffuse_texture = device.create_texture(&wgpu::TextureDescriptor {
        size: texture_size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        label: Some("diffuse_texture"),
    });

    queue.write_texture(
        wgpu::ImageCopyTexture {
            texture: &diffuse_texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        &*texture.bytes,
        wgpu::ImageDataLayout {
            offset: 0,
            bytes_per_row: std::num::NonZeroU32::new((4 * texture.width) as u32),
            rows_per_image: std::num::NonZeroU32::new(texture.height as u32),
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
        layout: &texture_bind_group_layout,
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
    transform: &Transform,
    camera: (&Camera, &Transform),
    is_ui_component: bool,
    uniform_bind_group_layout: &BindGroupLayout,
) -> (GlUniform, Buffer, BindGroup) {
    let uniform = GlUniform::from(UniformData { transform, camera, is_ui_component });
    let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Uniform Buffer"),
        contents: bytemuck::cast_slice(&[uniform]),
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

    (uniform, uniform_buffer, uniform_bind_group)
}

fn get_default_color_attachment<'a>(
    texture_view: &'a TextureView,
    config: &'a ScionConfig,
) -> RenderPassColorAttachment<'a> {
    RenderPassColorAttachment {
        view: texture_view,
        resolve_target: None,
        ops: wgpu::Operations {
            load: wgpu::LoadOp::Clear(
                if let Some(color) = &config
                    .window_config
                    .as_ref()
                    .expect("Window config is missing")
                    .default_background_color
                {
                    wgpu::Color {
                        r: (color.red() as f32 / 255.) as f64,
                        g: (color.green() as f32 / 255.) as f64,
                        b: (color.blue() as f32 / 255.) as f64,
                        a: color.alpha() as f64,
                    }
                } else {
                    wgpu::Color { r: 1., g: 0., b: 0., a: 1.0 }
                },
            ),
            store: true,
        },
    }
}

fn get_no_color_attachment(texture_view: &TextureView) -> RenderPassColorAttachment {
    RenderPassColorAttachment {
        view: texture_view,
        resolve_target: None,
        ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: true },
    }
}

fn get_path_from_color(color: &Color) -> String {
    format!("color-{}-{}-{}-{}", color.red(), color.green(), color.blue(), color.alpha())
}

fn world_contains_camera(world: &mut World) -> bool { <&Camera>::query().iter(world).count() > 0 }
