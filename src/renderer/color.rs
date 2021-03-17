use crate::renderer::bidimensional::gl_representations::GlColor;

/// A struct that represents colors for rendering.
pub struct Color {
    /// Red value of the color
    r: u8,
    /// Green value of the color
    g: u8,
    /// Blue value of the color
    b: u8,
    /// Alpha value of the color
    a: f32,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: f32) -> Self {
        assert!(
            a <= 1. && a >= 0.,
            "Alpha value must be between 0.0 and 1.0"
        );
        Self { r, g, b, a }
    }

    pub fn new_rgb(r: u8, g: u8, b: u8) -> Self {
        Color::new(r, g, b, 1.0)
    }

    pub fn replace(&mut self, color: Color) {
        self.r = color.r;
        self.g = color.g;
        self.b = color.b;
        self.a = color.a;
    }

    pub fn red(&self) -> u8 {
        self.r
    }

    pub fn green(&self) -> u8 {
        self.g
    }

    pub fn blue(&self) -> u8 {
        self.b
    }

    pub fn alpha(&self) -> f32 {
        self.a
    }
}

impl Into<GlColor> for &Color {
    fn into(self) -> GlColor {
        GlColor {
            r: self.r as f32 / 255.,
            g: self.g as f32 / 255.,
            b: self.b as f32 / 255.,
            a: self.a,
        }
    }
}
