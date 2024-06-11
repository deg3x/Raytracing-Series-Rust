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
    
    camera.position = Vec3::new(13.0, 2.0, 3.0);
    camera.look_at = Vec3::new(0.0, 0.0, 0.0);
    camera.up_vector = Vec3::new(0.0, 1.0, 0.0);
    camera.fov_vertical = 20.0;
    camera.defocus_angle = 0.6;
    camera.focus_distance = 10.0;
    camera.frame_width = 1080;
    camera.aspect_ratio = 16.0 / 9.0;
    camera.samples_per_pixel = 500; // 500
    camera.ray_bounces_max = 50; // 50
    
    let mut world: HittableList = HittableList::new();
    
    let mat_ground = Material {type_info: MaterialType::Lambert, albedo: Color01::new(0.5, 0.3, 0.5), fuzziness: 0.0, refraction_idx: 1.0};
    world.add(Box::new(Sphere {center: Vec3::new(0.0, -1000.0, 0.0), radius: 1000.0, material: mat_ground}));
    
    let mat_1 = Material {type_info: MaterialType::Dielectric, albedo: Color01::default(), fuzziness: 0.0, refraction_idx: 1.5};
    let mat_2 = Material {type_info: MaterialType::Lambert, albedo: Color01::new(0.4, 0.2, 0.1), fuzziness: 0.0, refraction_idx: 1.0};
    let mat_3 = Material {type_info: MaterialType::Metal, albedo: Color01::new(0.7, 0.6, 0.5), fuzziness: 0.0, refraction_idx: 1.0};
    
    world.add(Box::new(Sphere {center: Vec3::new(0.0, 1.0, 0.0), radius: 1.0, material: mat_1}));
    world.add(Box::new(Sphere {center: Vec3::new(-4.0, 1.0, 0.0), radius: 1.0, material: mat_2}));
    world.add(Box::new(Sphere {center: Vec3::new(4.0, 1.0, 0.0), radius: 1.0, material: mat_3}));
    
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rt_util::random();
            let position = Vec3::new(a as f64 + 0.9 * rt_util::random(), 0.2, b as f64 + 0.9 * rt_util::random());
            
            let mat: Material;
            if (position - Vec3::new(4.0, 0.2, 0.0)).len() > 0.9 {
                if choose_mat < 0.8 {
                    let color_vec = Vec3::random_range(0.0..1.0);
                    let color = Color01::new(color_vec.x, color_vec.y, color_vec.z);
                    
                    mat = Material {type_info: MaterialType::Lambert, albedo: color, fuzziness: 0.0, refraction_idx: 1.0};
                }
                else if choose_mat < 0.95 {
                    let color_vec = Vec3::random_range(0.5..1.0);
                    let color = Color01::new(color_vec.x, color_vec.y, color_vec.z);
                    let fuzz = rt_util::random() * 0.5;
                    
                    mat = Material {type_info: MaterialType::Metal, albedo: color, fuzziness: fuzz, refraction_idx: 1.0};
                }
                else {
                    mat = Material {type_info: MaterialType::Dielectric, albedo: Color01::default(), fuzziness: 0.0, refraction_idx: 1.5};
                }
                
                world.add(Box::new(Sphere {center: position, radius: 0.2, material: mat }));
            } 
        }
    }
    
    camera.render(&world);
}
