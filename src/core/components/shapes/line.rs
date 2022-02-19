use std::ops::Range;

use wgpu::{util::BufferInitDescriptor, PrimitiveTopology};

use crate::{
    core::components::{
        material::Material,
        maths::{coordinates::Coordinates, Pivot},
    },
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
        Line::new_with_offset(vertices, Coordinates::new(0., 0.))
    }

    /// Sets the pivot point of the line and returns it
    pub fn pivot(self, pivot: Pivot) -> Self {
        let offset = match pivot {
            Pivot::TopLeft => Coordinates::new(0., 0.),
            Pivot::Center => Coordinates::new(
                -(self.vertices[1].x - self.vertices[0].x).abs() / 2.,
                -(self.vertices[1].y - self.vertices[0].y).abs() / 2.,
            ),
        };
        Line::new_with_offset(self.vertices, offset)
    }

    /// Creates a new line using `vertices`.
    pub fn new_with_offset(vertices: [Coordinates; 2], offset: Coordinates) -> Self {
        let contents = [
            TexturedGlVertex::from((
                &Coordinates::new(&vertices[0].x + offset.x, &vertices[0].y + offset.y),
                &Coordinates::new(0., 0.),
            )),
            TexturedGlVertex::from((
                &Coordinates::new(&vertices[1].x + offset.x, &vertices[1].y + offset.y),
                &Coordinates::new(0., 0.),
            )),
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

    fn range(&self) -> Range<u32> {
        0..INDICES.len() as u32
    }

    fn topology() -> PrimitiveTopology {
        wgpu::PrimitiveTopology::LineList
    }

    fn dirty(&self) -> bool {
        false
    }

    fn set_dirty(&mut self, _is_dirty: bool) {}
}
