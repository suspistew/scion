use std::ops::Range;

use wgpu::util::BufferInitDescriptor;

use crate::{
    core::components::{material::Material, maths::transform::Coordinates},
    rendering::bidimensional::{gl_representations::TexturedGlVertex, scion2d::Renderable2D},
};

const INDICES: &[u16] = &[0, 1, 3, 3, 1, 2];

/// Renderable 2D Square.
pub struct Square {
    pub vertices: [Coordinates; 4],
    pub uvs: Option<[Coordinates; 4]>,
    contents: [TexturedGlVertex; 4],
}

impl Square {
    pub fn new(length: f32, uvs: Option<[Coordinates; 4]>) -> Self {
        let a = Coordinates::new(0., 0.);
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
        Self {
            vertices: [a, b, c, d],
            uvs,
            contents,
        }
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

    fn range(&self) -> Range<u32> {
        0..INDICES.len() as u32
    }

    fn dirty(&self) -> bool {
        false
    }

    fn set_dirty(&mut self, _is_dirty: bool) {}
}
