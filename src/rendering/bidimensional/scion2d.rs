use std::{collections::HashMap, ops::Range, path::Path};

use legion::{storage::Component, Entity, IntoQuery, Resources, World};
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BindGroup, BindGroupLayout, Buffer, CommandEncoder, Device, Queue,
    RenderPassColorAttachmentDescriptor, RenderPipeline, SwapChainDescriptor, SwapChainTexture,
};

use crate::rendering::{
    bidimensional::{
        components::{Square, Triangle},
        gl_representations::GlUniform,
        material::{Material2D, Texture},
        transform::Transform2D,
        Camera2D,
    },
    ScionRenderer,
};

pub(crate) trait Renderable2D {
    fn vertex_buffer_descriptor(&self) -> BufferInitDescriptor;
    fn indexes_buffer_descriptor(&self) -> BufferInitDescriptor;
    fn pipeline(
        &self,
        device: &Device,
        sc_desc: &SwapChainDescriptor,
        texture_bind_group_layout: &BindGroupLayout,
        transform_bind_group_layout: &BindGroupLayout,
    ) -> RenderPipeline;
    fn range(&self) -> Range<u32>;
}

#[derive(Default)]
pub(crate) struct Scion2D {
    vertex_buffers: HashMap<Entity, wgpu::Buffer>,
    index_buffers: HashMap<Entity, wgpu::Buffer>,
    render_pipelines: HashMap<String, RenderPipeline>,
    diffuse_bind_groups: HashMap<String, (BindGroupLayout, BindGroup)>,
    transform_uniform_bind_groups: HashMap<Entity, (GlUniform, Buffer, BindGroupLayout, BindGroup)>,
}

impl ScionRenderer for Scion2D {
    fn update(
        &mut self,
        world: &mut World,
        resources: &mut Resources,
        device: &Device,
        sc_desc: &SwapChainDescriptor,
        queue: &mut Queue,
    ) {
        if resources.contains::<Camera2D>() {
            self.update_diffuse_bind_groups(world, device, queue);
            self.update_transforms(world, resources, &device, queue);
            self.upsert_component_pipeline::<Triangle>(world, &device, &sc_desc);
            self.upsert_component_pipeline::<Square>(world, &device, &sc_desc);
        }
    }

    fn render(
        &mut self,
        world: &mut World,
        resource: &mut Resources,
        frame: &SwapChainTexture,
        encoder: &mut CommandEncoder,
    ) {
        {
            encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Scion 2D Render Pass"),
                color_attachments: &[get_default_color_attachment(frame)],
                depth_stencil_attachment: None,
            });
        }

        if resource.contains::<Camera2D>() {
            self.render_component::<Triangle>(world, &frame, encoder);
            self.render_component::<Square>(world, &frame, encoder);
        }
    }
}

fn load_texture_to_queue(
    texture: &Texture,
    queue: &mut Queue,
    device: &Device,
) -> (BindGroupLayout, BindGroup) {
    let texture_size = wgpu::Extent3d {
        width: texture.width as u32,
        height: texture.height as u32,
        depth: 1,
    };

    let diffuse_texture = device.create_texture(&wgpu::TextureDescriptor {
        size: texture_size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
        label: Some("diffuse_texture"),
    });
    queue.write_texture(
        wgpu::TextureCopyView {
            texture: &diffuse_texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
        },
        &*texture.bytes,
        wgpu::TextureDataLayout {
            offset: 0,
            bytes_per_row: (4 * texture.width) as u32,
            rows_per_image: texture.height as u32,
        },
        texture_size,
    );
    let diffuse_texture_view = diffuse_texture.create_view(&wgpu::TextureViewDescriptor::default());
    let diffuse_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        address_mode_u: wgpu::AddressMode::Repeat,
        address_mode_v: wgpu::AddressMode::Repeat,
        address_mode_w: wgpu::AddressMode::Repeat,
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Linear,
        mipmap_filter: wgpu::FilterMode::Linear,
        ..Default::default()
    });
    let texture_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: false },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Sampler {
                        comparison: false,
                        filtering: true,
                    },
                    count: None,
                },
            ],
            label: Some("texture_bind_group_layout"),
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
    (texture_bind_group_layout, diffuse_bind_group)
}

fn create_transform_uniform_bind_group(
    device: &Device,
    transform: &Transform2D,
    camera: &Camera2D,
) -> (GlUniform, Buffer, BindGroupLayout, BindGroup) {
    let uniform = GlUniform::from((transform, camera));
    let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Uniform Buffer"),
        contents: bytemuck::cast_slice(&[uniform]),
        usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
    });

    let uniform_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStage::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("uniform_bind_group_layout"),
        });

    let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &uniform_bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: uniform_buffer.as_entire_binding(),
        }],
        label: Some("uniform_bind_group"),
    });

    (
        uniform,
        uniform_buffer,
        uniform_bind_group_layout,
        uniform_bind_group,
    )
}

fn get_default_color_attachment(frame: &SwapChainTexture) -> RenderPassColorAttachmentDescriptor {
    wgpu::RenderPassColorAttachmentDescriptor {
        attachment: &frame.view,
        resolve_target: None,
        ops: wgpu::Operations {
            load: wgpu::LoadOp::Clear(wgpu::Color {
                r: 0.,
                g: 0.,
                b: 0.,
                a: 1.0,
            }),
            store: true,
        },
    }
}

fn get_no_color_attachment(frame: &SwapChainTexture) -> RenderPassColorAttachmentDescriptor {
    wgpu::RenderPassColorAttachmentDescriptor {
        attachment: &frame.view,
        resolve_target: None,
        ops: wgpu::Operations {
            load: wgpu::LoadOp::Load,
            store: true,
        },
    }
}

impl Scion2D {
    fn upsert_component_pipeline<T: Component + Renderable2D>(
        &mut self,
        world: &mut World,
        device: &&Device,
        sc_desc: &&SwapChainDescriptor,
    ) {
        for (entity, component, material, _) in
            <(Entity, &mut T, &Material2D, &Transform2D)>::query().iter_mut(world)
        {
            if !self.vertex_buffers.contains_key(entity) {
                let vertex_buffer =
                    device.create_buffer_init(&component.vertex_buffer_descriptor());
                self.vertex_buffers.insert(*entity, vertex_buffer);
            }

            if !self.index_buffers.contains_key(entity) {
                let index_buffer =
                    device.create_buffer_init(&component.indexes_buffer_descriptor());
                self.index_buffers.insert(*entity, index_buffer);
            }

            match material {
                Material2D::Color(_) => {}
                Material2D::Texture(path) => {
                    if !self.render_pipelines.contains_key(path.as_str()) {
                        self.render_pipelines.insert(
                            path.clone(),
                            component.pipeline(
                                &device,
                                &sc_desc,
                                &self.diffuse_bind_groups.get(path.as_str()).unwrap().0,
                                &self.transform_uniform_bind_groups.get(entity).unwrap().2,
                            ),
                        );
                    }
                }
            };
        }
    }

    fn render_component<T: Component + Renderable2D>(
        &mut self,
        world: &mut World,
        frame: &&SwapChainTexture,
        encoder: &mut CommandEncoder,
    ) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Scion 2D Render Pass"),
            color_attachments: &[get_no_color_attachment(frame)],
            depth_stencil_attachment: None,
        });

        for (entity, component, material, _transform) in
            <(Entity, &mut T, &Material2D, &Transform2D)>::query().iter_mut(world)
        {
            render_pass.set_bind_group(
                1,
                &self.transform_uniform_bind_groups.get(entity).unwrap().3,
                &[],
            );
            render_pass.set_vertex_buffer(
                0,
                self.vertex_buffers.get(entity).as_ref().unwrap().slice(..),
            );
            render_pass.set_index_buffer(
                self.index_buffers.get(entity).as_ref().unwrap().slice(..),
                wgpu::IndexFormat::Uint16,
            );
            match material {
                Material2D::Color(_) => {}
                Material2D::Texture(path) => {
                    render_pass
                        .set_pipeline(self.render_pipelines.get(path.as_str()).as_ref().unwrap());
                    render_pass.set_bind_group(
                        0,
                        &self.diffuse_bind_groups.get(path.as_str()).unwrap().1,
                        &[],
                    );
                }
            };
            render_pass.draw_indexed(component.range(), 0, 0..1);
        }
    }

    fn update_transforms(
        &mut self,
        world: &mut World,
        resources: &mut Resources,
        device: &&Device,
        queue: &mut Queue,
    ) {
        let camera = resources
            .get::<Camera2D>()
            .expect("Missing Camera2D component, can't update transform without the camera view");
        for (entity, transform) in <(Entity, &Transform2D)>::query().iter_mut(world) {
            if !self.transform_uniform_bind_groups.contains_key(entity) {
                let (uniform, uniform_buffer, glayout, group) =
                    create_transform_uniform_bind_group(&device, transform, &*camera);
                queue.write_buffer(&uniform_buffer, 0, bytemuck::cast_slice(&[uniform]));
                self.transform_uniform_bind_groups
                    .insert(*entity, (uniform, uniform_buffer, glayout, group));
            } else {
                let (uniform, uniform_buffer, _, _) = self
                    .transform_uniform_bind_groups
                    .get_mut(entity)
                    .expect("Fatal error, a transform has been marked as found but doesn't exist");
                uniform.replace_with(GlUniform::from((transform, &*camera)));
                queue.write_buffer(uniform_buffer, 0, bytemuck::cast_slice(&[*uniform]));
            }
        }
    }

    /// Loads in the queue materials that are not yet loaded.
    fn update_diffuse_bind_groups(
        &mut self,
        world: &mut World,
        device: &Device,
        queue: &mut Queue,
    ) {
        <(Entity, &Material2D)>::query().for_each(world, |(_entity, material)| {
            match material {
                Material2D::Texture(path) => {
                    if !self.diffuse_bind_groups.contains_key(path.as_str()) {
                        let loaded_texture = Texture::from_png(Path::new(path.as_str()));
                        self.diffuse_bind_groups.insert(
                            path.clone(),
                            load_texture_to_queue(&loaded_texture, queue, device),
                        );
                    }
                }
                _ => {}
            }
        });
    }
}
