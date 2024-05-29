use std::convert::{From};

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

pub struct Color01 {
    pub r: f64,
    pub g: f64,
    pub b: f64
}

impl From<Color01> for Color {
    fn from(value: Color01) -> Self {
        let mult: f64 = 255.999;
        
        Self {
            r: (value.r * mult) as u8,
            g: (value.g * mult) as u8,
            b: (value.b * mult) as u8
        }
    }
}

impl From<Color> for Color01 {
    fn from(value: Color) -> Self {
        let inv_mult: f64 = 1.0 / 255.999;
        
        Self {
            r: value.r as f64 * inv_mult,
            g: value.g as f64 * inv_mult,
            b: value.b as f64 * inv_mult
        }
    }
}
