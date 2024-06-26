use std::convert::From;
use std::ops::{Add, Mul, AddAssign};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

#[derive(Clone, Copy, Debug, PartialEq)]
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

impl Default for Color01 {
    fn default() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0
        }
    }
}

impl Add for Color01 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl AddAssign for Color01 {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

impl Mul<f64> for Color01 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs
        }
    }
}

impl Mul<Color01> for f64 {
    type Output = Color01;

    fn mul(self, rhs: Color01) -> Self::Output {
        Color01 {
            r: self * rhs.r,
            g: self * rhs.g,
            b: self * rhs.b
        }
    }
}

impl Mul<Color01> for Color01 {
    type Output = Self;

    fn mul(self, rhs: Color01) -> Self {
        Self {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b
        }
    }
}

impl Color01 {
    pub fn new(r: f64, g: f64, b: f64) -> Color01 {
        Color01 {r, g, b}
    }
    
    pub fn linear_to_gamma(&self) -> Color01{
        let mut ret = Color01::default();
        
        if self.r > 0.0 {
            ret.r = self.r.sqrt();
        }
        if self.g > 0.0 {
            ret.g = self.g.sqrt();
        }
        if self.b > 0.0 {
            ret.b = self.b.sqrt();
        }
        
        ret
    }
}

fn print_color(color: Color) {
    println!("{} {} {}", color.r.clamp(0, 255), color.g.clamp(0, 255), color.b.clamp(0, 255));
}

pub fn print_color_01(color: Color01) {
    let color_byte = Color::from(color.linear_to_gamma());
    
    print_color(color_byte);
}
