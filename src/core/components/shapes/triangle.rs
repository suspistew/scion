use std::ops::Range;

use wgpu::{util::BufferInitDescriptor, PrimitiveTopology};

use crate::{
    core::components::{material::Material, maths::coordinates::Coordinates, maths::Pivot},
    rendering::{gl_representations::TexturedGlVertex, Renderable2D},
    utils::maths::Vector,
};

const INDICES: &[u16] = &[1, 0, 2];

/// Renderable 2D Triangle.
pub struct Triangle {
    pub vertices: [Coordinates; 3],
    pub uvs: Option<[Coordinates; 3]>,
    contents: [TexturedGlVertex; 3],
}

impl Triangle {
    /// Creates a new square using `length`.
    /// uvs are mandatory but this will be updated
    pub fn new(vertices: [Coordinates; 3], uvs: Option<[Coordinates; 3]>) -> Self {
        let uvs_ref = uvs.as_ref().expect("Uvs are currently mandatory, this need to be fixed");
        let contents = [
            TexturedGlVertex::from((&vertices[0], &uvs_ref[0])),
            TexturedGlVertex::from((&vertices[1], &uvs_ref[1])),
            TexturedGlVertex::from((&vertices[2], &uvs_ref[2])),
        ];
        Self { vertices, uvs, contents }
    }

    pub fn pivot(self, pivot: Pivot) -> Self {
        let offset = match pivot {
            Pivot::TopLeft => Vector::new(0., 0.),
            Pivot::Center => Vector::new((self.vertices[0].x + self.vertices[1].x + self.vertices[2].x).abs() / 3., (self.vertices[0].y + self.vertices[1].y + self.vertices[2].y) / 3.),
        };
        Triangle::new_with_offset(self.vertices, self.uvs, offset)
    }

    fn new_with_offset(vertices: [Coordinates; 3], uvs: Option<[Coordinates; 3]>, offset: Vector) -> Self {
        let a = Coordinates::new(&vertices[0].x - offset.x, &vertices[0].y - offset.y);
        let b = Coordinates::new(&vertices[1].x - offset.x, &vertices[1].y - offset.y);
        let c = Coordinates::new(&vertices[2].x - offset.x, &vertices[2].y - offset.y);
        let uvs_ref = uvs.as_ref().expect("Uvs are currently mandatory, this need to be fixed");
        let contents = [
            TexturedGlVertex::from((&a, &uvs_ref[0])),
            TexturedGlVertex::from((&b, &uvs_ref[1])),
            TexturedGlVertex::from((&c, &uvs_ref[2])),
        ];
        Self { vertices, uvs, contents }
    }
}

impl Renderable2D for Triangle {
    fn vertex_buffer_descriptor(&mut self, _material: Option<&Material>) -> BufferInitDescriptor {
        wgpu::util::BufferInitDescriptor {
            label: Some("Triangle Vertex Buffer"),
            contents: bytemuck::cast_slice(&self.contents),
            usage: wgpu::BufferUsages::VERTEX,
        }
    }

    fn indexes_buffer_descriptor(&self) -> BufferInitDescriptor {
        wgpu::util::BufferInitDescriptor {
            label: Some("Triangle Index Buffer"),
            contents: bytemuck::cast_slice(&INDICES),
            usage: wgpu::BufferUsages::INDEX,
        }
    }

    fn range(&self) -> Range<u32> { 0..3 as u32 }

    fn topology() -> PrimitiveTopology { wgpu::PrimitiveTopology::TriangleList }

    fn dirty(&self) -> bool { false }

    fn set_dirty(&mut self, _is_dirty: bool) {}
}
