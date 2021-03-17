use wgpu::{BindGroupLayout, Device, RenderPipeline, SwapChainDescriptor};

use crate::renderer::bidimensional::gl_representations::TexturedGlVertex;

use crate::renderer::bidimensional::transform::Position2D;

pub struct Triangle {
    pub vertices: [Position2D; 3],
    pub uvs: Option<[Position2D; 3]>,
}

pub(crate) fn triangle_pipeline(
    device: &Device,
    sc_desc: &SwapChainDescriptor,
    texture_bind_group_layout: &BindGroupLayout,
    transform_bind_group_layout: &BindGroupLayout,
) -> RenderPipeline {
    let vs_module = device.create_shader_module(&wgpu::include_spirv!("shaders/shader.vert.spv"));
    let fs_module = device.create_shader_module(&wgpu::include_spirv!("shaders/shader.frag.spv"));

    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
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
