use std::ops::Range;

use wgpu::{
    util::BufferInitDescriptor, BindGroupLayout, BlendFactor, BlendOperation, Device,
    RenderPipeline, SwapChainDescriptor,
};

use crate::{
    core::components::maths::transform::Coordinates,
    rendering::bidimensional::{
        gl_representations::TexturedGlVertex,
        scion2d::{Renderable2D, RenderableUi},
    },
};

/// Renderable 2D Square.
#[derive(Debug)]
pub struct UiImage {
    image_path: String,
    contents: [TexturedGlVertex; 4],
}

const INDICES: &[u16] = &[0, 1, 3, 3, 1, 2];

impl UiImage {
    pub fn new(width: f32, height: f32, image_path: String) -> Self {
        let uvs = [
            Coordinates::new(0., 0.),
            Coordinates::new(0., 1.),
            Coordinates::new(1., 1.),
            Coordinates::new(1., 0.),
        ];
       UiImage::new_with_uv_map(width, height, image_path,uvs)
    }

    pub fn new_with_uv_map(width: f32, height: f32, image_path: String, uvs: [Coordinates;4]) -> Self {
        let a = Coordinates::new(0., 0.);
        let b = Coordinates::new(a.x(), a.y() + height);
        let c = Coordinates::new(a.x() + width, a.y() + height);
        let d = Coordinates::new(a.x() + width, a.y());

        let contents = [
            TexturedGlVertex::from((&a, &uvs[0])),
            TexturedGlVertex::from((&b, &uvs[1])),
            TexturedGlVertex::from((&c, &uvs[2])),
            TexturedGlVertex::from((&d, &uvs[3])),
        ];
        Self {
            image_path,
            contents,
        }
    }
}

impl Renderable2D for UiImage {
    fn vertex_buffer_descriptor(&self) -> BufferInitDescriptor {
        wgpu::util::BufferInitDescriptor {
            label: Some("Square Vertex Buffer"),
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
        let vs_module = device.create_shader_module(&wgpu::include_spirv!(
            "../../../rendering/shaders/shader.vert.spv"
        ));
        let fs_module = device.create_shader_module(&wgpu::include_spirv!(
            "../../../rendering/shaders/shader.frag.spv"
        ));

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Basic square pipeline layout"),
                bind_group_layouts: &[texture_bind_group_layout, transform_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Square render pipeline"),
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
                    alpha_blend: wgpu::BlendState {
                        src_factor: BlendFactor::One,
                        dst_factor: BlendFactor::One,
                        operation: BlendOperation::Add,
                    },
                    color_blend: wgpu::BlendState {
                        src_factor: BlendFactor::SrcAlpha,
                        dst_factor: BlendFactor::OneMinusSrcAlpha,
                        operation: BlendOperation::Add,
                    },
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
        0..INDICES.len() as u32
    }
}

impl RenderableUi for UiImage {
    fn get_texture_path(&self) -> Option<String> {
        Some(self.image_path.clone())
    }
}
