use serde::{Deserialize, Serialize};

use crate::rendering::gl_representations::GlColor;

/// A struct that represents colors for rendering.
#[derive(Clone, Debug, Serialize, Deserialize)]
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
    /// Creates a new color using RGBA values
    pub fn new(r: u8, g: u8, b: u8, a: f32) -> Self {
        assert!(a <= 1. && a >= 0., "Alpha value must be between 0.0 and 1.0");
        Self { r, g, b, a }
    }

    /// Create a new color using RGB. Will create an RGBA with Alpha = 1.0
    pub fn new_rgb(r: u8, g: u8, b: u8) -> Self {
        Color::new(r, g, b, 1.0)
    }

    /// Replaces the current color with the given one
    pub fn replace(&mut self, color: Color) {
        self.r = color.r;
        self.g = color.g;
        self.b = color.b;
        self.a = color.a;
    }

    pub fn new_hex(hex_code:&str) -> Self {
        let hex:Vec<char> = hex_code.chars().collect();
        let red:u8 = Color::get_hex_value(hex[1]) * 16 + Color::get_hex_value(hex[2]) * 1;
        let green:u8 = Color::get_hex_value(hex[3]) * 16 + Color::get_hex_value(hex[4]) * 1;
        let blue:u8 = Color::get_hex_value(hex[5]) * 16 + Color::get_hex_value(hex[6]) * 1;
        let mut alpha:f32 = 1.0;
        if hex_code.len() > 7 // alpha value also exists in hex code
        {
            let alpha255: u8 = Color::get_hex_value(hex[7]) * 16 + Color::get_hex_value(hex[8]) * 1;
            alpha = alpha255 as f32 / 255.0;
        }
        Color::new(red, green, blue, alpha)
    }

   fn get_hex_value(mut ch: char) -> u8 {
        ch = ch.to_ascii_lowercase();
        if ch == 'a' {
            return 10;
        }
        else if ch == 'b' {
            return 11;
        }
        else if ch == 'c' {
            return 12;
        }
        else if ch == 'd' {
            return 13;
        }
        else if ch == 'e' {
            return 14;
        }
        else if ch == 'f' {
            return 15
        }
        else {
            return ch.to_digit(10).unwrap() as u8;
        }
        
    }

    /// Red value
    pub fn red(&self) -> u8 {
        self.r
    }

    /// Green value
    pub fn green(&self) -> u8 {
        self.g
    }

    /// Blue value
    pub fn blue(&self) -> u8 {
        self.b
    }

    /// Alpha value
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

impl ToString for Color{
    fn to_string(&self) -> String {
        format!("r{:?}g{:?}b{:?}a{:?}", self.r,self.g,self.b,self.a)
    }
}
