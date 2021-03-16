use crate::renderer::ScionRenderer;
use legion::{Resources, World};
use crate::renderer::bidimensional::triangle::{triangle_pipeline};
use crate::renderer::bidimensional::material::Material2D;
use crate::renderer::bidimensional::transform::Transform2D;



use wgpu::{Device, SwapChainDescriptor, CommandEncoder, SwapChainTexture, RenderPassColorAttachmentDescriptor};
use std::collections::HashMap;
use wgpu::util::DeviceExt;
use crate::renderer::bidimensional::gl_representations::{ColoredGlVertex, GlVec3, GlColor};

const VERTICES: &[ColoredGlVertex] = &[
    ColoredGlVertex { position: GlVec3 { x: 0.0, y: 0.5, z: 0.0 }, color: GlColor { r: 1.0, g: 0.0, b: 0.0, a: 1. } },
    ColoredGlVertex { position: GlVec3 { x: -0.5, y: -0.5, z: 0.0 }, color: GlColor { r: 0.0, g: 1.0, b: 0.0, a: 1.0 } },
    ColoredGlVertex { position: GlVec3 { x: 0.5, y: -0.5, z: 0.0 }, color: GlColor { r: 0.0, g: 0.0, b: 1.0, a: 1.0 } },
];

pub trait Renderable2D {
    fn render(&self,
              material: Option<&Material2D>,
              transform: &Transform2D);
}

#[derive(Default)]
pub struct Scion2D{
    render_pipelines: HashMap<String, wgpu::RenderPipeline>,
    vertex_buffer: Option<wgpu::Buffer>,
}

impl ScionRenderer for Scion2D{

    fn setup_pipelines(&mut self, device: &Device, sc_desc: &SwapChainDescriptor){
        self.render_pipelines.insert("triangle".to_string(), triangle_pipeline(&device, &sc_desc));

        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(VERTICES),
                usage: wgpu::BufferUsage::VERTEX,
            }
        );
        self.vertex_buffer = Some(vertex_buffer);
    }

    fn render(&mut self, _world: &mut World, _resources: &mut Resources, frame: &SwapChainTexture, encoder: &mut CommandEncoder){
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Scion 2D Render Pass"),
                color_attachments: &[get_default_color_attachment(frame)],
                depth_stencil_attachment: None,
            });
            render_pass.set_pipeline(self.render_pipelines.get("triangle").unwrap());
            render_pass.set_vertex_buffer(0, self.vertex_buffer.as_ref().unwrap().slice(..));
            render_pass.draw(0..VERTICES.len() as u32, 0..1);
        };
    }
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

/*
impl ScionRenderer for Scion2D {
    fn draw(&mut self, world: &mut World, _resource: &mut Resources) {

        context.begin_default_pass(Default::default());
        let mut query_triangles = <(Entity, &Triangle, &Material2D, &Transform2D)>::query();
        query_triangles.for_each(world, |(_e, triangle, material, transform)| {
            info!(
                "rendering triangle {:?}", transform
            );
            triangle.render(context, Some(material), transform)
        });
        context.end_render_pass();
        context.commit_frame();

    }
}
 */