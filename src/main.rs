mod primitive;
mod vector;
mod camera;
mod color;
mod ray;
mod rt_util;
mod material;

use color::Color01;
use material::*;
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
    
    let mat_ground = Material {type_info: MaterialType::Lambert, albedo: Color01::new(0.8, 0.8, 0.0), fuzziness: 0.0, refraction_idx: 1.0};
    let mat_center = Material {type_info: MaterialType::Lambert, albedo: Color01::new(0.1, 0.2, 0.5), fuzziness: 0.0, refraction_idx: 1.0};
    let mat_left = Material {type_info: MaterialType::Dielectric, albedo: Color01::new(0.8, 0.8, 0.8), fuzziness: 0.3, refraction_idx: 1.0 / 1.33};
    let mat_right = Material {type_info: MaterialType::Metal, albedo: Color01::new(0.8, 0.6, 0.2), fuzziness: 0.8, refraction_idx: 1.0};
    
    world.add(Box::new(Sphere {center: Vec3::new(0.0, -100.5, -1.0), radius: 100.0, material: mat_ground}));
    world.add(Box::new(Sphere {center: Vec3::new(0.0, 0.0, -1.2), radius: 0.5, material: mat_center}));
    world.add(Box::new(Sphere {center: Vec3::new(-1.0, 0.0, -1.0), radius: 0.5, material: mat_left}));
    world.add(Box::new(Sphere {center: Vec3::new(1.0, 0.0, -1.0), radius: 0.5, material: mat_right}));
    
    camera.render(&world);
}
