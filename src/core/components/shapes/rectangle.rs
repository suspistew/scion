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

/// Renderable 2D Rectangle.
pub struct Rectangle {
    width: f32,
    height: f32,
    pub vertices: [Coordinates; 4],
    pub uvs: Option<[Coordinates; 4]>,
    contents: [TexturedGlVertex; 4],
    dirty: bool,
}

impl Rectangle {
    /// Creates a new rectangle using `length` and `height`.
    /// When rendering using a texture, you can customize uvs map using `uvs`. By default it will
    /// use 0 to 1 uvs
    pub fn new(width: f32, height: f32, uvs: Option<[Coordinates; 4]>) -> Self {
        Rectangle::new_with_offset(width, height, uvs, 0., 0.)
    }

    pub fn pivot(self, pivot: Pivot) -> Self {
        let (x_offset, y_offset) = match pivot {
            Pivot::TopLeft => (0., 0.),
            Pivot::Center => (self.width / 2., self.height / 2.),
        };

        Rectangle::new_with_offset(self.width, self.height, self.uvs, x_offset, y_offset)
    }

    pub fn new_with_offset(
        width: f32,
        height: f32,
        uvs: Option<[Coordinates; 4]>,
        x_offset: f32,
        y_offset: f32,
    ) -> Self {
        let a = Coordinates::new(0. - x_offset, 0. - y_offset);
        let b = Coordinates::new(a.x(), a.y() + height);
        let c = Coordinates::new(a.x() + width, a.y() + height);
        let d = Coordinates::new(a.x() + width, a.y());
        let uvs_ref = uvs.unwrap_or(default_uvs());
        let contents = [
            TexturedGlVertex::from((&a, &uvs_ref[0])),
            TexturedGlVertex::from((&b, &uvs_ref[1])),
            TexturedGlVertex::from((&c, &uvs_ref[2])),
            TexturedGlVertex::from((&d, &uvs_ref[3])),
        ];
        Self { width, height, vertices: [a, b, c, d], uvs: Some(uvs_ref), contents, dirty: false }
    }

    pub fn set_height(&mut self, new_height: f32) {
        let a = Coordinates::new(0., 0.);
        let b = Coordinates::new(a.x(), a.y() + new_height);
        let c = Coordinates::new(a.x() + self.width, a.y() + new_height);
        let d = Coordinates::new(a.x() + self.width, a.y());
        let uvs_ref = self.uvs.unwrap();
        let contents = [
            TexturedGlVertex::from((&a, &uvs_ref[0])),
            TexturedGlVertex::from((&b, &uvs_ref[1])),
            TexturedGlVertex::from((&c, &uvs_ref[2])),
            TexturedGlVertex::from((&d, &uvs_ref[3])),
        ];
        self.contents = contents;
        self.vertices = [a, b, c, d];
        self.dirty = true;
        self.height = new_height;
    }

    pub fn set_width(&mut self, new_width: f32) {
        let a = Coordinates::new(0., 0.);
        let b = Coordinates::new(a.x(), a.y() + self.height);
        let c = Coordinates::new(a.x() + new_width, a.y() + self.height);
        let d = Coordinates::new(a.x() + new_width, a.y());
        let uvs_ref = self.uvs.unwrap();
        let contents = [
            TexturedGlVertex::from((&a, &uvs_ref[0])),
            TexturedGlVertex::from((&b, &uvs_ref[1])),
            TexturedGlVertex::from((&c, &uvs_ref[2])),
            TexturedGlVertex::from((&d, &uvs_ref[3])),
        ];
        self.contents = contents;
        self.vertices = [a, b, c, d];
        self.dirty = true;
        self.width = new_width;
    }

    pub fn height(&self) -> f32 {
        self.height
    }
    pub fn width(&self) -> f32 {
        self.width
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

impl Renderable2D for Rectangle {
    fn vertex_buffer_descriptor(&mut self, _material: Option<&Material>) -> BufferInitDescriptor {
        wgpu::util::BufferInitDescriptor {
            label: Some("Rectangle Vertex Buffer"),
            contents: bytemuck::cast_slice(&self.contents),
            usage: wgpu::BufferUsages::VERTEX,
        }
    }

    fn indexes_buffer_descriptor(&self) -> BufferInitDescriptor {
        wgpu::util::BufferInitDescriptor {
            label: Some("Rectangle Index Buffer"),
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
        self.dirty = is_dirty
    }
}
