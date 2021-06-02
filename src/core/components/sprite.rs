use std::ops::Range;

use wgpu::util::BufferInitDescriptor;

use crate::{
    core::components::maths::transform::Coordinates,
    rendering::bidimensional::{gl_representations::TexturedGlVertex, scion2d::Renderable2D},
};

const INDICES: &[u16] = &[0, 1, 3, 3, 1, 2];

/// Renderable Sprite.
pub struct Sprite {
    /// Maximum number of tiles per line
    length: usize,
    /// Number of lines in the sprite
    height: usize,
    /// Size of a tile
    tile_size: usize,
    /// Desired tile to render for this material.
    tile_number: usize,
    /// Current computed content for vertex
    contents: Option<[TexturedGlVertex; 4]>,
}

impl Sprite {
    pub fn new(length: usize, height: usize, tile_size: usize, tile_number: usize) -> Self {
        Self {
            length,
            height,
            tile_size,
            tile_number,
            contents: None,
        }
    }

    fn uv_refs(&self) -> [Coordinates; 4] {
        let line = (self.tile_number / self.length) as f32;
        let column = (self.tile_number % self.length) as f32;

        let unit_line = 1.0 / self.height as f32;
        let unit_column = 1.0 / self.length as f32;

        let a = Coordinates::new(column * unit_column, line * unit_line);
        let b = Coordinates::new(a.x(), a.y() + unit_line);
        let c = Coordinates::new(a.x() + unit_column, a.y() + unit_line);
        let d = Coordinates::new(a.x() + unit_column, a.y());
        [a, b, c, d]
    }
}

impl Renderable2D for Sprite {
    fn vertex_buffer_descriptor(&mut self) -> BufferInitDescriptor {
        if self.contents.is_none() {
            let a = Coordinates::new(0., 0.);
            let b = Coordinates::new(0., self.tile_size as f32);
            let c = Coordinates::new(self.tile_size as f32, self.tile_size as f32);
            let d = Coordinates::new(self.tile_size as f32, 0.);
            let uvs_ref = self.uv_refs();
            let contents = [
                TexturedGlVertex::from((&a, &uvs_ref[0])),
                TexturedGlVertex::from((&b, &uvs_ref[1])),
                TexturedGlVertex::from((&c, &uvs_ref[2])),
                TexturedGlVertex::from((&d, &uvs_ref[3])),
            ];
            self.contents = Some(contents);
        }
        wgpu::util::BufferInitDescriptor {
            label: Some("Square Vertex Buffer"),
            contents: bytemuck::cast_slice(
                self.contents
                    .as_ref()
                    .expect("A computed content is missing in Sprite component"),
            ),
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
