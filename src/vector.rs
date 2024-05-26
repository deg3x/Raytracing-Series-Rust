use std::ops::{ Add, Sub, Mul };

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {x: x, y: y, z: z}
    }
    
    pub fn len_sqr(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    
    pub fn len(&self) -> f64 {
        let squared = self.len_sqr();
        
        squared.sqrt()
    }
    
    pub fn normalized(&self) -> Vec3 {
        let inv_len = 1f64 / self.len();
        
        Vec3 {x: self.x * inv_len, y: self.y * inv_len, z: self.z * inv_len}
    }
}

impl Add for Vec3 {
    type Output = Self;
    
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z   
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;
    
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z   
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

pub fn dot(a: Vec3, b: Vec3) -> f64 {
    a.x * b.x + a.y * b.y + a.z * b.z
}

pub fn cross(a: Vec3, b: Vec3) -> Vec3 {
    Vec3 { 
        x: a.y * b.z - b.y * a.z,
        y: a.x * b.z - b.x * a.z,
        z: a.x * b.y - b.x * a.y
    }
}
