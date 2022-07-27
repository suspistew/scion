use wgpu::{
    BindGroupLayout, BlendComponent, BlendFactor, BlendOperation, Device, RenderPipeline,
    SurfaceConfiguration,
};

use crate::rendering::gl_representations::TexturedGlVertex;

pub fn pipeline(
    device: &Device,
    surface_config: &SurfaceConfiguration,
    texture_bind_group_layout: &BindGroupLayout,
    transform_bind_group_layout: &BindGroupLayout,
    topology: wgpu::PrimitiveTopology,
) -> RenderPipeline {
    let vs_module = device.create_shader_module(wgpu::include_spirv!("./shader.vert.spv"));
    let fs_module = device.create_shader_module(wgpu::include_spirv!("./shader.frag.spv"));

    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Basic square pipeline layout"),
        bind_group_layouts: &[texture_bind_group_layout, transform_bind_group_layout],
        push_constant_ranges: &[],
    });

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Scion's render pipeline"),
        layout: Some(&render_pipeline_layout),
        vertex: wgpu::VertexState {
            module: &vs_module,
            entry_point: "main",
            buffers: &[TexturedGlVertex::desc()],
        },
        fragment: Some(wgpu::FragmentState {
            module: &fs_module,
            entry_point: "main",
            targets: &[Some(wgpu::ColorTargetState {
                format: surface_config.format,
                write_mask: wgpu::ColorWrites::ALL,
                blend: Some(wgpu::BlendState {
                    color: BlendComponent {
                        src_factor: BlendFactor::SrcAlpha,
                        dst_factor: BlendFactor::OneMinusSrcAlpha,
                        operation: BlendOperation::Add,
                    },
                    alpha: BlendComponent {
                        src_factor: BlendFactor::One,
                        dst_factor: BlendFactor::One,
                        operation: BlendOperation::Add,
                    },
                }),
            })],
        }),
        primitive: wgpu::PrimitiveState {
            topology,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            unclipped_depth: false,
            polygon_mode: wgpu::PolygonMode::Fill,
            conservative: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None
    });
    render_pipeline
}
