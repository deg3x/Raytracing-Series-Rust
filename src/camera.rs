use indicatif::{ProgressBar, ProgressStyle};

use crate::vector::Vec3;
use crate::ray::Ray;
use crate::color::*;
use crate::primitive::*;
use crate::rt_util;

#[derive(Clone, Copy, Debug)]
pub struct Camera {
    pub position: Vec3,
    pub view_u: Vec3,
    pub view_v: Vec3,
    pub view_pixel_upper_left: Vec3,
    pub pixel_zero: Vec3,
    pub pixel_delta_u: Vec3,
    pub pixel_delta_v: Vec3,
    
    pub pixel_samples_scale: f64,
    pub aspect_ratio: f64,
    pub focal_length: f64,
    pub view_height: f64,
    pub view_width: f64,
    
    pub frame_width: u32,
    pub frame_height: u32,
    pub frame_res: u32,
    
    pub samples_per_pixel: u16,
}

impl Camera {
    pub fn render(&self, world: &HittableList) {
        rt_util::print_image_header(self.frame_width, self.frame_height);
        
        let progress_bar = ProgressBar::new(self.frame_res as u64);
        progress_bar.set_style(ProgressStyle::with_template("[{elapsed_precise}] |{bar:40.cyan/blue}| {percent}%")
            .unwrap()
            .progress_chars("=> "));
        
        for i in 0..self.frame_height {
            for j in 0..self.frame_width {
                let px_center = self.pixel_zero + self.pixel_delta_u * j as f64 + self.pixel_delta_v * i as f64;
                let ray_dir = px_center - self.position;
                let ray: Ray = Ray::new(self.position, ray_dir);
                let color = Camera::ray_color(&ray, &world);
                
                print_color(color);
                
                progress_bar.inc(1);
            }
        }
        
        progress_bar.finish();
    }
    
    fn ray_color(ray: &Ray, world: &HittableList) -> Color {
        let hit_result = world.hit(ray, 0.0..rt_util::INFINITY);
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
}

impl Default for Camera {
    fn default() -> Self {
        let position: Vec3 = Vec3 {x: 0.0, y: 0.0, z: 0.0};
        let aspect_ratio: f64 = 16.0 / 9.0;
        let frame_width: u32 = 1024u32;
        let frame_height: u32 = (frame_width as f64 / aspect_ratio) as u32;
        let frame_res: u32 = frame_width * frame_height;
        let focal_length: f64 = 1.0;
        let view_height: f64 = 2.0;
        let view_width: f64 = view_height * (frame_width as f64 / frame_height as f64);
        let view_u: Vec3 = Vec3::new(view_width, 0.0, 0.0);
        let view_v: Vec3 = Vec3::new(0.0, -view_height, 0.0);
        let pixel_delta_u: Vec3 = view_u * (1.0 / frame_width as f64);
        let pixel_delta_v: Vec3 = view_v * (1.0 / frame_height as f64);
        let view_pixel_upper_left: Vec3 = position - Vec3::new(0.0, 0.0, focal_length) - (view_u + view_v) * 0.5;
        let pixel_zero: Vec3 = view_pixel_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;
        let samples_per_pixel: u16 = 10;
        let pixel_samples_scale: f64 = 1.0/samples_per_pixel as f64;
        
        Camera {
            position,
            view_u,
            view_v,
            view_pixel_upper_left,
            pixel_zero,
            pixel_delta_u,
            pixel_delta_v,
            pixel_samples_scale,
            aspect_ratio,
            focal_length,
            view_height,
            view_width,
            frame_width,
            frame_height,
            frame_res,
            samples_per_pixel
        }
    }
}
