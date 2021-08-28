use std::ops::Range;

use wgpu::{util::BufferInitDescriptor, PrimitiveTopology};

use crate::{
    core::components::{material::Material, maths::coordinates::Coordinates},
    rendering::{gl_representations::TexturedGlVertex, Renderable2D},
};

const INDICES: &[u16] = &[0, 1];

/// Renderable 2D Line.
pub struct Line {
    pub vertices: [Coordinates; 2],
    contents: [TexturedGlVertex; 2],
}

impl Line {
    /// Creates a new line using `vertices`.
    pub fn new(vertices: [Coordinates; 2]) -> Self {
        let contents = [
            TexturedGlVertex::from((&vertices[0], &Coordinates::new(0., 0.))),
            TexturedGlVertex::from((&vertices[1], &Coordinates::new(0., 0.))),
        ];
        Self { vertices, contents }
    }
}

impl Renderable2D for Line {
    fn vertex_buffer_descriptor(&mut self, _material: Option<&Material>) -> BufferInitDescriptor {
        wgpu::util::BufferInitDescriptor {
            label: Some("Line Vertex Buffer"),
            contents: bytemuck::cast_slice(&self.contents),
            usage: wgpu::BufferUsages::VERTEX,
        }
    }

    fn indexes_buffer_descriptor(&self) -> BufferInitDescriptor {
        wgpu::util::BufferInitDescriptor {
            label: Some("Line Index Buffer"),
            contents: bytemuck::cast_slice(&INDICES),
            usage: wgpu::BufferUsages::INDEX,
        }
    }

    fn range(&self) -> Range<u32> { 0..INDICES.len() as u32 }

    fn topology() -> PrimitiveTopology { wgpu::PrimitiveTopology::LineList }

    fn dirty(&self) -> bool { false }

    fn set_dirty(&mut self, _is_dirty: bool) {}
}
