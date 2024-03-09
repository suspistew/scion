use std::ops::Range;

use wgpu::{util::BufferInitDescriptor, PrimitiveTopology};

use crate::{
    core::components::{
        material::Material, maths::coordinates::Coordinates, tiles::tileset::Tileset,
    },
    rendering::{gl_representations::TexturedGlVertex, Renderable2D},
};
use crate::core::components::maths::Pivot;
use crate::core::components::shapes::rectangle::Rectangle;
use crate::utils::maths::Vector;

const INDICES: &[u16] = &[0, 1, 3, 3, 1, 2];

/// Renderable Sprite.
#[derive(Debug)]
pub struct Sprite {
    /// Desired tile to render for this material.
    tile_number: usize,
    /// Current computed content for vertex
    contents: Option<[TexturedGlVertex; 4]>,
    /// Flag to keep track of changed tile number
    dirty: bool,
    /// Pivot point of the sprite, default topleft
    pivot: Pivot,
}

impl Sprite {
    /// Creates a new sprite that will use the `tile_number` from the tileset associated in the same
    /// entity
    pub fn new(tile_number: usize) -> Self {
        Self { tile_number, contents: None, dirty: false, pivot: Pivot::TopLeft }
    }

    pub fn pivot(self, pivot: Pivot) -> Self {
        Self { tile_number: self.tile_number, contents: None, dirty: false, pivot }
    }

    /// Modify the current sprite tile number
    pub fn set_tile_nb(&mut self, new_tile_nb: usize) {
        self.tile_number = new_tile_nb;
        self.dirty = true;
    }

    pub fn get_tile_nb(&self) -> usize {
        self.tile_number
    }

    fn compute_pivot_offset(pivot: &Pivot, length: usize) -> Vector {
        match pivot {
            Pivot::TopLeft => Vector::new(0., 0.),
            Pivot::Center => Vector::new(length as f32 / 2., length as f32 / 2.),
        }
    }

    fn uv_refs(&self, tileset: &Tileset) -> [Coordinates; 4] {
        let line = (self.tile_number / tileset.width) as f32;
        let column = (self.tile_number % tileset.width) as f32;

        let unit_line = 1.0 / tileset.height as f32;
        let unit_column = 1.0 / tileset.width as f32;

        let a = Coordinates::new(column * unit_column, line * unit_line);
        let b = Coordinates::new(a.x(), a.y() + unit_line);
        let c = Coordinates::new(a.x() + unit_column, a.y() + unit_line);
        let d = Coordinates::new(a.x() + unit_column, a.y());
        [a, b, c, d]
    }

    pub(crate) fn compute_content(&self, material: Option<&Material>) -> [TexturedGlVertex; 4] {
        if (self.dirty || self.contents.is_none()) && material.is_some() {
            if let Material::Tileset(tileset) = material.unwrap() {
                let offset = Self::compute_pivot_offset(&self.pivot, tileset.tile_size);
                let a = Coordinates::new(0. - offset.x, 0. - offset.y);
                let b = Coordinates::new(a.x, a.y + tileset.tile_size as f32);
                let c = Coordinates::new(a.x + tileset.tile_size as f32, a.y + tileset.tile_size as f32);
                let d = Coordinates::new(a.x + tileset.tile_size as f32, a.y);
                let uvs_ref = self.uv_refs(&tileset);
                return [
                    TexturedGlVertex::from((&a, &uvs_ref[0])),
                    TexturedGlVertex::from((&b, &uvs_ref[1])),
                    TexturedGlVertex::from((&c, &uvs_ref[2])),
                    TexturedGlVertex::from((&d, &uvs_ref[3])),
                ];
            }
        }
        self.contents.as_ref().expect("A computed content is missing in Sprite component").clone()
    }

    pub(crate) fn indices() -> Vec<u16> {
        INDICES.to_vec()
    }

    pub(crate) fn set_content(&mut self, content: [TexturedGlVertex; 4]) {
        self.contents = Some(content);
    }
    pub fn get_pivot(&self) -> Pivot {
        self.pivot.clone()
    }
}

impl Renderable2D for Sprite {
    fn vertex_buffer_descriptor(&mut self, material: Option<&Material>) -> BufferInitDescriptor {
        let content = self.compute_content(material);
        self.contents = Some(content);
        BufferInitDescriptor {
            label: Some("Sprite Vertex Buffer"),
            contents: bytemuck::cast_slice(
                self.contents.as_ref().expect("A computed content is missing in Sprite component"),
            ),
            usage: wgpu::BufferUsages::VERTEX,
        }
    }

    fn indexes_buffer_descriptor(&self) -> BufferInitDescriptor {
        BufferInitDescriptor {
            label: Some("Sprite Index Buffer"),
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
        self.dirty
    }

    fn set_dirty(&mut self, is_dirty: bool) {
        self.dirty = is_dirty;
    }

    fn get_pivot_offset(&self, material: Option<&Material>) -> Vector {
        if (material.is_none()) {
            Vector::default()
        } else if let Material::Tileset(tileset) = material.unwrap() {
            Self::compute_pivot_offset(&self.pivot, tileset.tile_size)
        } else { Vector::default() }
    }
}
