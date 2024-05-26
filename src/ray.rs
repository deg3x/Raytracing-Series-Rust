use crate::vector::Vec3;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3
}

impl Ray {
    pub fn new(orig: Vec3, dir: Vec3) -> Ray {
        Ray {
            origin: orig,
            direction: dir
        }
    }
    
    pub fn at(&self, interp: f64) -> Vec3 {
        self.origin + self.direction * interp
    }
}
