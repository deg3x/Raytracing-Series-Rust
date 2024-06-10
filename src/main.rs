mod primitive;
mod vector;
mod camera;
mod color;
mod ray;
mod rt_util;
mod material;

use primitive::*;
use camera::*;
use vector::*;

fn main() {
    let mut camera: Camera = Camera::default();
    
    camera.position = Vec3::new(0.0, 0.0, 0.0);
    camera.focal_length = 1.0;
    camera.frame_width = 1080;
    camera.aspect_ratio = 16.0 / 9.0;
    camera.samples_per_pixel = 10;
    camera.ray_bounces_max = 10;
    
    let mut world: HittableList = HittableList::new();
    world.add(Box::new(Sphere {center: Vec3::new(0.0, 0.0, -1.0), radius: 0.5}));
    world.add(Box::new(Sphere {center: Vec3::new(0.0, -100.5, -1.0), radius: 100.0}));
    
    camera.render(&world);
}
