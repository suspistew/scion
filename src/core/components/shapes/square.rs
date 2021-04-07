use std::ops::Range;

use wgpu::util::BufferInitDescriptor;

use crate::{
    core::components::maths::transform::Coordinates,
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
    pub fn new(origin: Coordinates, length: f32, uvs: Option<[Coordinates; 4]>) -> Self {
        let a = origin;
        let b = Coordinates::new(a.x(), a.y() + length);
        let c = Coordinates::new(a.x() + length, a.y() + length);
        let d = Coordinates::new(a.x() + length, a.y());
        let uvs_ref = uvs
            .as_ref()
            .expect("Uvs are currently mandatory, this need to be fixed");
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

impl Renderable2D for Square {
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

    fn range(&self) -> Range<u32> {
        0..INDICES.len() as u32
    }
}
