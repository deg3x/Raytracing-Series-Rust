mod primitive;
mod vector;
mod camera;
mod color;
mod ray;
mod rt_util;

use primitive::*;
use camera::*;
use vector::*;

fn main() {
    let camera: Camera = Camera::default();
    
    let mut world: HittableList = HittableList::new();
    world.add(Box::new(Sphere {center: Vec3::new(0.0, 0.0, -1.0), radius: 0.5}));
    world.add(Box::new(Sphere {center: Vec3::new(0.0, -100.5, -1.0), radius: 100.0}));
    
    camera.render(&world);
}
