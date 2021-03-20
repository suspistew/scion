use wgpu::util::BufferInitDescriptor;
use wgpu::{BindGroupLayout, Device, RenderPipeline, SwapChainDescriptor};

use crate::rendering::bidimensional::gl_representations::TexturedGlVertex;
use crate::rendering::bidimensional::scion2d::Renderable2D;
use crate::rendering::bidimensional::transform::Position2D;
use std::ops::Range;

const INDICES: &[u16] = &[1, 0, 2];

pub struct Triangle {
    pub vertices: [Position2D; 3],
    pub uvs: Option<[Position2D; 3]>,
    contents: [TexturedGlVertex; 3],
}

impl Triangle {
    pub fn new(vertices: [Position2D; 3], uvs: Option<[Position2D; 3]>) -> Self {
        let uvs_ref = uvs
            .as_ref()
            .expect("Uvs are currently mandatory, this need to be fixed");
        let contents = [
            TexturedGlVertex::from((&vertices[0], &uvs_ref[0])),
            TexturedGlVertex::from((&vertices[1], &uvs_ref[1])),
            TexturedGlVertex::from((&vertices[2], &uvs_ref[2])),
        ];
        Self {
            vertices,
            uvs,
            contents,
        }
    }
}

impl Renderable2D for Triangle {
    fn vertex_buffer_descriptor(&self) -> BufferInitDescriptor {
        wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&self.contents),
            usage: wgpu::BufferUsage::VERTEX,
        }
    }

    fn indexes_buffer_descriptor(&self) -> BufferInitDescriptor {
        wgpu::util::BufferInitDescriptor {
            label: Some("Square Index Buffer"),
            contents: bytemuck::cast_slice(&INDICES),
            usage: wgpu::BufferUsage::INDEX,
        }
    }

    fn pipeline(
        &self,
        device: &Device,
        sc_desc: &SwapChainDescriptor,
        texture_bind_group_layout: &BindGroupLayout,
        transform_bind_group_layout: &BindGroupLayout,
    ) -> RenderPipeline {
        let vs_module =
            device.create_shader_module(&wgpu::include_spirv!("shaders/shader.vert.spv"));
        let fs_module =
            device.create_shader_module(&wgpu::include_spirv!("shaders/shader.frag.spv"));

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Basic triangle pipeline layout"),
                bind_group_layouts: &[texture_bind_group_layout, transform_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Triangle render pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vs_module,
                entry_point: "main",
                buffers: &[TexturedGlVertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &fs_module,
                entry_point: "main",
                targets: &[wgpu::ColorTargetState {
                    format: sc_desc.format,
                    alpha_blend: wgpu::BlendState::REPLACE,
                    color_blend: wgpu::BlendState::REPLACE,
                    write_mask: wgpu::ColorWrite::ALL,
                }],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: wgpu::CullMode::Back,
                polygon_mode: wgpu::PolygonMode::Fill,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
        });
        render_pipeline
    }

    fn range(&self) -> Range<u32> {
        0..3 as u32
    }
}
