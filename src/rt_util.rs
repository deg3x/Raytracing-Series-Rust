#![allow(dead_code)]

use rand::Rng;

use crate::vector::{Vec3, dot};

pub const PI: f64 = 3.1415926535897932385;
pub const INFINITY: f64 = f64::INFINITY;

pub fn deg_to_rad(deg: f64) -> f64 {
    deg * PI / 180.0
}

pub fn rad_to_deg(rad: f64) -> f64 {
    rad * 180.0 / PI
}

pub fn print_image_header(width: u32, height: u32) {
    println!("P3");
    println!("{} {}", width, height);
    println!("255");
}

pub fn random() -> f64 {
    let mut rng = rand::thread_rng();
    
    rng.gen()
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let mut point = Vec3::random_range(-1.0..1.0);
        point.z = 0.0;
        
        if point.len_sqr() < 1.0 {
            return point;
        }
    }
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let point = Vec3::random_range(-1.0..1.0);
        
        if point.len_sqr() < 1.0 {
            return point;
        }
    }
}

pub fn random_on_unit_hemisphere(normal: &Vec3) -> Vec3 {
    let point = random_in_unit_sphere().normalized();
    let is_correct_hs = dot(normal, &point) > 0.0;
    
    if is_correct_hs {
        return point;
    }
    
    point * -1.0
}
