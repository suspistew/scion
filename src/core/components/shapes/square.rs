use std::ops::Range;

use wgpu::{util::BufferInitDescriptor, PrimitiveTopology};

use crate::{
    core::components::{
        material::Material,
        maths::{coordinates::Coordinates, Pivot},
    },
    rendering::{gl_representations::TexturedGlVertex, Renderable2D},
};

const INDICES: &[u16] = &[0, 1, 3, 3, 1, 2];

/// Renderable 2D Square.
pub struct Square {
    pub vertices: [Coordinates; 4],
    pub uvs: Option<[Coordinates; 4]>,
    contents: [TexturedGlVertex; 4],
    length: f32,
}

impl Square {
    /// Creates a new square using `length`.
    /// When rendering using a texture, you can customize uvs map using `uvs`. By default it will
    /// use 0 to 1 uvs
    pub fn new(length: f32, uvs: Option<[Coordinates; 4]>) -> Self {
        Square::new_with_offset(length, uvs, 0.)
    }

    /// Sets the pivot point of the square and returns it
    pub fn pivot(self, pivot: Pivot) -> Self {
        let offset = match pivot {
            Pivot::TopLeft => 0.,
            Pivot::Center => self.length / 2.,
        };
        Square::new_with_offset(self.length, self.uvs, offset)
    }

    fn new_with_offset(length: f32, uvs: Option<[Coordinates; 4]>, offset: f32) -> Self {
        let a = Coordinates::new(0. - offset, 0. - offset);
        let b = Coordinates::new(a.x(), a.y() + length);
        let c = Coordinates::new(a.x() + length, a.y() + length);
        let d = Coordinates::new(a.x() + length, a.y());
        let uvs_ref = uvs.unwrap_or(default_uvs());
        let contents = [
            TexturedGlVertex::from((&a, &uvs_ref[0])),
            TexturedGlVertex::from((&b, &uvs_ref[1])),
            TexturedGlVertex::from((&c, &uvs_ref[2])),
            TexturedGlVertex::from((&d, &uvs_ref[3])),
        ];
        Self { vertices: [a, b, c, d], uvs, contents, length }
    }
}

fn default_uvs() -> [Coordinates; 4] {
    [
        Coordinates::new(0., 0.),
        Coordinates::new(0., 1.),
        Coordinates::new(1., 1.),
        Coordinates::new(1., 0.),
    ]
}

impl Renderable2D for Square {
    fn vertex_buffer_descriptor(&mut self, _material: Option<&Material>) -> BufferInitDescriptor {
        wgpu::util::BufferInitDescriptor {
            label: Some("Square Vertex Buffer"),
            contents: bytemuck::cast_slice(&self.contents),
            usage: wgpu::BufferUsages::VERTEX,
        }
    }

    fn indexes_buffer_descriptor(&self) -> BufferInitDescriptor {
        wgpu::util::BufferInitDescriptor {
            label: Some("Square Index Buffer"),
            contents: bytemuck::cast_slice(&INDICES),
            usage: wgpu::BufferUsages::INDEX,
        }
    }

    fn range(&self) -> Range<u32> {
        0..INDICES.len() as u32
    }

    fn topology() -> PrimitiveTopology {
        wgpu::PrimitiveTopology::TriangleList
    }

    fn dirty(&self) -> bool {
        false
    }

    fn set_dirty(&mut self, _is_dirty: bool) {}
}
