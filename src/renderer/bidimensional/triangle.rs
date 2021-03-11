use crate::renderer::Renderable2D;
use miniquad::{Context, Buffer, BufferType, Bindings, Shader, Pipeline, BufferLayout, VertexAttribute, VertexFormat };
use crate::renderer::bidimensional::{Vertex, Vec2, Uniforms, Vec4};

pub struct Triangle;

impl Renderable2D for Triangle {
    fn render(context: &mut Context) {
        let vertices: [Vertex; 3] = [
            Vertex { pos: Vec2 { x: -0.5, y: -0.5 }, color: Vec4 { r: 1.0, g: 0., b: 0., a: 1.0, }, },
            Vertex { pos: Vec2 { x: 0.5, y: -0.5 }, color: Vec4 { r: 0., g: 1., b: 0., a: 1.0 } },
            Vertex { pos: Vec2 { x: 0., y: 0.5 }, color: Vec4 { r: 0., g: 0., b: 1., a: 1.0 } },
        ];
        let vertex_buffer = Buffer::immutable(context, BufferType::VertexBuffer, &vertices);

        let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];
        let index_buffer = Buffer::immutable(context, BufferType::IndexBuffer, &indices);

        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer,
            images: vec![],
        };

        let shader = Shader::new(context, shader::VERTEX, shader::FRAGMENT, shader::meta()).unwrap();

        let pipeline = Pipeline::new(
            context,
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("pos", VertexFormat::Float2),
                VertexAttribute::new("color", VertexFormat::Float4),
            ],
            shader,
        );

        context.begin_default_pass(Default::default());

        context.apply_pipeline(&pipeline);
        context.apply_bindings(&bindings);

        context.apply_uniforms(&Uniforms {
            offset: (0., 0.),
        });

        context.draw(0, 3, 1);
        context.end_render_pass();

        context.commit_frame();
    }
}

mod shader {
    use miniquad::*;

    pub const VERTEX: &str =
        r#"
            #version 330 core
            in vec2 pos;
            in vec4 color;
            uniform vec2 offset;
            out lowp vec4 color_lowp;
            void main() {
                gl_Position = vec4(pos + offset, 0, 1);
                color_lowp = color;
            }
        "#;

    pub const FRAGMENT: &str =
        r#"
            #version 330 core
            in lowp vec4 color_lowp;
            out vec4 FragColor;
            void main() {
                FragColor = color_lowp;
            }
        "#;

    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images: vec![],
            uniforms: UniformBlockLayout {
                uniforms: vec![UniformDesc::new("offset", UniformType::Float2)],
            },
        }
    }
}