use miniquad::{Context, Buffer, BufferType, Bindings, Shader, Pipeline, BufferLayout, VertexAttribute, VertexFormat};
use crate::renderer::bidimensional::gl_representations::{GlVertex, GlVec2,GlVec4, GlColor, GlUniform};
use crate::renderer::bidimensional::material::Material2D;
use ultraviolet::{Isometry3, Similarity3, Vec4, Vec3, Rotor3, Rotor2, Similarity2, Vec2, Mat4};
use crate::renderer::bidimensional::renderer::Renderable2D;
use crate::renderer::bidimensional::transform::Transform2D;

pub struct Triangle;

impl Renderable2D for Triangle {
    fn render(context: &mut Context, material: Option<&Material2D>, transform: &Transform2D) {
        let color: GlColor = match material.expect("Render function must not be called without a material") {
            Material2D::Color(c) => c.into()
        };
        let vertices: [GlVertex; 3] = [
            GlVertex { pos: GlVec2 { x: -0.5, y: -0.5 }, color: color.clone() },
            GlVertex { pos: GlVec2 { x: 0., y: 0.5 }, color: color.clone() },
            GlVertex { pos: GlVec2 { x: 0.5, y: -0.5 }, color },
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

        context.apply_pipeline(&pipeline);
        context.apply_bindings(&bindings);

        let mut transform_rotate = Isometry3::identity();
        transform_rotate.append_translation(Vec3{
            x: transform.position.x,
            y: transform.position.y,
            z: 1.0
        });
        transform_rotate.prepend_rotation(Rotor3::from_rotation_xy(transform.angle).normalized());

        let mut scale = Similarity3::identity();
        scale.append_scaling(transform.scale);

        let mut transform_rotate = transform_rotate.into_homogeneous_matrix();
        let mut scale = scale.into_homogeneous_matrix();

        context.apply_uniforms(&GlUniform {
            offset: (0. , 0.),
            trans: Triangle::create_glmat4(&mut transform_rotate),
            scale: Triangle::create_glmat4(&mut scale),
        });

        context.draw(0, 3, 1);
    }
}


mod shader {
    use miniquad::*;

    pub const VERTEX: &str =
        r#"
            #version 330 core
            in vec2 pos;
            in vec4 color;
            uniform mat4 trans;
            uniform mat4 scale;
            uniform vec2 offset;
            out lowp vec4 color_lowp;
            void main() {
                gl_Position = (trans * vec4(pos, 0, 1)) * scale;
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
                uniforms: vec![
                    UniformDesc::new("offset", UniformType::Float2),
                    UniformDesc::new("trans", UniformType::Mat4),
                    UniformDesc::new("scale", UniformType::Mat4)],

            },
        }
    }
}
