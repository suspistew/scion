use std::ops::Range;

use wgpu::{util::BufferInitDescriptor, PrimitiveTopology};

use crate::{
    core::components::{material::Material, maths::coordinates::Coordinates},
    rendering::{gl_representations::TexturedGlVertex, Renderable2D},
};

/// Renderable 2D Polygon made of outlines.
pub struct Polygon {
    pub vertices: Vec<Coordinates>,
    contents: Vec<TexturedGlVertex>,
    indices: Vec<u16>,
    dirty: bool,
}

impl Polygon {
    /// Creates a new polygon using `vertices`.
    /// Rendering is done with strip, meaning if you give 3 vertices (a,b,c) then it will create a to b and b to c.
    pub fn new(vertices: Vec<Coordinates>) -> Self {
        let contents = vertices
            .iter()
            .map(|c| TexturedGlVertex::from((c, &Coordinates::new(0., 0.))))
            .collect();
        let indices = (0..vertices.len() as u16).collect();
        Self { vertices, contents, indices, dirty: true }
    }

    /// Retrieves the vertices list
    pub fn get_vertices(&self) -> &Vec<Coordinates> {
        &self.vertices
    }

    /// Append x to the nth vertice
    pub fn append_x(&mut self, index: usize, x: f32) {
        self.vertices.get_mut(index).unwrap().x += x;
        self.compute_contents();
    }

    /// Append y to the nth vertice
    pub fn append_y(&mut self, index: usize, y: f32) {
        self.vertices.get_mut(index).unwrap().y += y;
        self.compute_contents();
    }

    fn compute_contents(&mut self) {
        self.contents = self
            .vertices
            .iter()
            .map(|c| TexturedGlVertex::from((c, &Coordinates::new(0., 0.))))
            .collect();
        self.dirty = true;
    }
}

impl Renderable2D for Polygon {
    fn vertex_buffer_descriptor(&mut self, _material: Option<&Material>) -> BufferInitDescriptor {
        wgpu::util::BufferInitDescriptor {
            label: Some("Polygon Vertex Buffer"),
            contents: bytemuck::cast_slice(&self.contents),
            usage: wgpu::BufferUsages::VERTEX,
        }
    }

    fn indexes_buffer_descriptor(&self) -> BufferInitDescriptor {
        wgpu::util::BufferInitDescriptor {
            label: Some("Polygon Index Buffer"),
            contents: bytemuck::cast_slice(&self.indices.as_slice()),
            usage: wgpu::BufferUsages::INDEX,
        }
    }

    fn range(&self) -> Range<u32> {
        0..self.indices.len() as u32
    }

    fn topology() -> PrimitiveTopology {
        wgpu::PrimitiveTopology::LineStrip
    }

    fn dirty(&self) -> bool {
        self.dirty
    }

    fn set_dirty(&mut self, is_dirty: bool) {
        self.dirty = is_dirty;
    }
}
