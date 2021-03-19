use crate::rendering::bidimensional::gl_representations::{create_glmat4, GlUniform};
use ultraviolet::{Isometry3, Rotor3, Similarity3, Vec3};

#[derive(Default, Debug, Copy, Clone)]
pub struct Position2D {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug)]
pub struct Transform2D {
    pub(crate) position: Position2D,
    pub(crate) scale: f32,
    pub(crate) angle: f32,
}

impl Default for Transform2D {
    fn default() -> Self {
        Self {
            position: Default::default(),
            scale: 1.0,
            angle: 0.0,
        }
    }
}

impl Transform2D {
    pub fn new(position: Position2D, scale: f32, angle: f32) -> Self {
        Self {
            position,
            scale,
            angle,
        }
    }

    pub fn append_translation(&mut self, x: f32, y: f32) {
        self.position.x += x;
        self.position.y += y;
    }

    pub fn append_angle(&mut self, angle: f32) {
        self.angle += angle;
    }

    pub fn position(&self) -> &Position2D{
        &self.position
    }

    pub fn set_scale(&mut self, scale: f32){
        self.scale = scale
    }
}

impl From<&Transform2D> for GlUniform {
    fn from(transform: &Transform2D) -> Self {
        let mut transform_rotate = Similarity3::identity();
        transform_rotate.prepend_scaling(transform.scale);
        transform_rotate.append_translation(Vec3 {
            x: transform.position.x,
            y: transform.position.y,
            z: 1.0,
        });
        transform_rotate.prepend_rotation(Rotor3::from_rotation_xy(transform.angle).normalized());

        let mut scale = Similarity3::identity();

        let mut transform_rotate = transform_rotate.into_homogeneous_matrix();
        let mut scale = scale.into_homogeneous_matrix();
        GlUniform {
            trans: create_glmat4(&mut transform_rotate),
            scale: create_glmat4(&mut scale),
        }
    }
}
