use indicatif::{ProgressBar, ProgressStyle};

pub mod primitive;
pub mod vector;
pub mod camera;
pub mod color;
pub mod ray;

use primitive::*;
use camera::*;
use vector::*;
use color::*;
use ray::*;

fn main() {
    let camera: Camera = Camera::default();
    
    print_image_header(camera.frame_width, camera.frame_height);
    
    let progress_bar = ProgressBar::new(camera.frame_res as u64);
    progress_bar.set_style(ProgressStyle::with_template("[{elapsed_precise}] |{bar:40.cyan/blue}| {percent}%")
        .unwrap()
        .progress_chars("=> "));
    
    for i in 0..camera.frame_height {
        for j in 0..camera.frame_width {
            let px_center = camera.pixel_zero + camera.pixel_delta_u * j as f64 + camera.pixel_delta_v * i as f64;
            let ray_dir = px_center - camera.position;
            let ray: Ray = Ray::new(camera.position, ray_dir);
            let color = ray_color(&ray);
            
            print_color(color);
            
            progress_bar.inc(1);
        }
    }
    
    progress_bar.finish();
}

fn ray_color(ray: &Ray) -> Color {
    let sphere = Sphere {center: Vec3::new(0.0, 0.0, -1.0), radius: 0.5};
    
    let hit_result = sphere.hit(ray, 0.0, 1000.0);
    if hit_result.is_hit {
        return Color::from(Color01 {r: hit_result.data.normal.x + 1.0, g: hit_result.data.normal.y + 1.0, b: hit_result.data.normal.z + 1.0} * 0.5);
    }
    
    let ray_dir_norm = ray.direction.normalized();
    let interp = (ray_dir_norm.y + 1.0) * 0.5;
    let color_white = Color01 {r: 1.0, g: 1.0, b: 1.0};
    let color_blue = Color01 {r: 0.5, g: 0.7, b: 1.0};
    
    let color_01 = (1.0 - interp) * color_white + interp * color_blue;
    
    Color::from(color_01)
}

fn print_image_header(width: u32, height: u32) {
    println!("P3");
    println!("{} {}", width, height);
    println!("255");
}
