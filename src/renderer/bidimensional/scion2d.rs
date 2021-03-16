use crate::renderer::bidimensional::material::{Material2D, Texture2D};
use crate::renderer::bidimensional::transform::Transform2D;
use crate::renderer::bidimensional::triangle::{triangle_pipeline, Triangle};
use crate::renderer::ScionRenderer;
use legion::{Resources, World, Entity, IntoQuery};

use crate::renderer::bidimensional::gl_representations::{ColoredGlVertex, GlColor, GlVec3, TexturedGlVertex, GlVec2};
use std::collections::HashMap;
use wgpu::util::DeviceExt;
use wgpu::{CommandEncoder, Device, RenderPassColorAttachmentDescriptor, SwapChainDescriptor, SwapChainTexture, Queue, BindGroup, RenderPipeline, BindGroupLayout};
use std::path::Path;

const VERTICES: &[TexturedGlVertex] = &[
    TexturedGlVertex {
        position: GlVec3 {
            x: -0.5,
            y: -0.5,
            z: 0.0,
        },
        tex_coords: GlVec2 {
            x: 0.0,
            y: 1.,
        },
    },
    TexturedGlVertex {
        position: GlVec3 {
            x: 0.5,
            y: -0.5,
            z: 0.0,
        },
        tex_coords: GlVec2 {
            x: 1.0,
            y: 1.0,
        },
    },
    TexturedGlVertex {
        position: GlVec3 {
            x: 0.0,
            y: 0.5,
            z: 0.0,
        },
        tex_coords: GlVec2 {
            x: 0.5,
            y: 0.0,
        },
    },
];

pub trait Renderable2D {
    fn render(&self, material: Option<&Material2D>, transform: &Transform2D);
}

struct RenderingPipelineWithData {
    render_pipeline: RenderPipeline,
    bind_group: usize,
}


#[derive(Default)]
pub struct Scion2D {
    vertex_buffers: HashMap<Entity, wgpu::Buffer>,
    render_pipelines: HashMap<String, RenderPipeline>,
    diffuse_bind_groups: HashMap<String, (BindGroupLayout, BindGroup)>,
}

impl ScionRenderer for Scion2D {

    fn render(
        &mut self,
        world: &mut World,
        _resources: &mut Resources,
        frame: &SwapChainTexture,
        encoder: &mut CommandEncoder,
        device: &Device,
        sc_desc: &SwapChainDescriptor,
        queue: &mut Queue,
    ) {
        self.load_textures_to_queue(world, device, queue);

        let mut query_triangles = <(Entity, &mut Triangle, &Material2D, &Transform2D)>::query();
        let triangles: Vec<(&Entity, &mut Triangle, &Material2D, &Transform2D)> =
            query_triangles.iter_mut(world).map(|(e)| e).collect();
        for (entity, _triangle, material, transform) in triangles.iter() {
            if !self.vertex_buffers.contains_key(*entity){
                let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Vertex Buffer"),
                    contents: bytemuck::cast_slice(VERTICES),
                    usage: wgpu::BufferUsage::VERTEX,
                });
                self.vertex_buffers.insert(**entity, vertex_buffer);
            }

            match material {
                Material2D::Color(_) => {}
                Material2D::Texture(texture) => {
                    if !self.render_pipelines.contains_key(&texture.path) {
                        self.render_pipelines.insert(texture.path.clone(), triangle_pipeline(&device, &sc_desc, &self.diffuse_bind_groups.get(&texture.path).unwrap().0));
                    }
                }
            };
        };

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Scion 2D Render Pass"),
                color_attachments: &[get_default_color_attachment(frame)],
                depth_stencil_attachment: None,
            });

            for (entity, triangle, material, transform) in triangles.iter() {
                match material {
                    Material2D::Color(_) => {}
                    Material2D::Texture(texture) => {
                        render_pass.set_pipeline(self.render_pipelines.get(&texture.path).as_ref().unwrap());
                        render_pass.set_bind_group(0, &self.diffuse_bind_groups.get(&texture.path).unwrap().1, &[]);
                        render_pass.set_vertex_buffer(0,  self.vertex_buffers.get(*entity).as_ref().unwrap().slice(..));
                        render_pass.draw(0..VERTICES.len() as u32, 0..1);
                    }
                };
            };
        }
    }
}

impl Scion2D {
    /// Loads in the queue materials that are not yet loaded.
    fn load_textures_to_queue(&mut self, world: &mut World, device: &Device, queue: &mut Queue) {
        <(Entity, &Material2D)>::query().for_each(world, |(entity, material)| {
            match material {
                Material2D::Texture(texture) => {
                    if !self.diffuse_bind_groups.contains_key(&texture.path) {
                        self.diffuse_bind_groups.insert(texture.path.clone(), load_texture_to_queue(&texture, queue, device));
                    }
                }
                _ => {}
            }
        });
    }
}

fn load_texture_to_queue(texture: &Texture2D, queue: &mut Queue, device: &Device) -> (BindGroupLayout, BindGroup) {
    let texture_size = wgpu::Extent3d {
        width: texture.width as u32,
        height: texture.height as u32,
        depth: 1,
    };

    let diffuse_texture = device.create_texture(
        &wgpu::TextureDescriptor {
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
            label: Some("diffuse_texture"),
        }
    );
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
    let texture_bind_group_layout = device.create_bind_group_layout(
        &wgpu::BindGroupLayoutDescriptor {
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
        }
    );

    let diffuse_bind_group = device.create_bind_group(
        &wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&diffuse_sampler),
                }
            ],
            label: Some("diffuse_bind_group"),
        }
    );
    (texture_bind_group_layout, diffuse_bind_group)
}

fn get_default_color_attachment(frame: &SwapChainTexture) -> RenderPassColorAttachmentDescriptor {
    wgpu::RenderPassColorAttachmentDescriptor {
        attachment: &frame.view,
        resolve_target: None,
        ops: wgpu::Operations {
            load: wgpu::LoadOp::Clear(wgpu::Color {
                r: 0.,
                g: 0.2,
                b: 0.7,
                a: 1.0,
            }),
            store: true,
        },
    }
}
