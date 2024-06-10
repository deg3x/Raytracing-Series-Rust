use indicatif::{ProgressBar, ProgressStyle};
use rand::prelude::*;

use crate::vector::Vec3;
use crate::ray::Ray;
use crate::color::*;
use crate::primitive::*;
use crate::rt_util;

#[derive(Clone, Copy, Debug)]
pub struct Camera {
    pub position: Vec3,
    pub aspect_ratio: f64,
    pub focal_length: f64,
    pub frame_width: u32,
    pub samples_per_pixel: u16,
    pub ray_bounces_max: u16,
    
    pixel_zero: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel_samples_scale: f64,
    
    frame_height: u32,
    frame_res: u32,
}

impl Camera {
    pub fn initialize(&mut self) {
        self.frame_height = (self.frame_width as f64 / self.aspect_ratio) as u32;
        self.frame_res = self.frame_width * self.frame_height;
        
        let view_height = 2.0;
        let view_width = view_height * (self.frame_width as f64 / self.frame_height as f64);
        let view_u = Vec3::new(view_width, 0.0, 0.0);
        let view_v = Vec3::new(0.0, -view_height, 0.0);
        self.pixel_delta_u = view_u * (1.0 / self.frame_width as f64);
        self.pixel_delta_v = view_v * (1.0 / self.frame_height as f64);
        
        let view_pixel_upper_left = self.position - Vec3::new(0.0, 0.0, self.focal_length) - (view_u + view_v) * 0.5;
        self.pixel_zero = view_pixel_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5;
        
        self.pixel_samples_scale = 1.0/self.samples_per_pixel as f64;
    }
    
    pub fn render(&mut self, world: &HittableList) {
        self.initialize();
        
        rt_util::print_image_header(self.frame_width, self.frame_height);
        
        let progress_bar = ProgressBar::new(self.frame_res as u64);
        progress_bar.set_style(ProgressStyle::with_template("[{elapsed_precise}] |{bar:40.cyan/blue}| {percent}%")
            .unwrap()
            .progress_chars("=> "));
        
        for i in 0..self.frame_height {
            for j in 0..self.frame_width {
                let mut color = Color01::default();
                
                for _ in 0..self.samples_per_pixel {
                    let ray: Ray = self.get_ray(i, j);
                    color += Camera::ray_color(&ray, self.ray_bounces_max, &world) * self.pixel_samples_scale;
                }
                
                print_color_01(color);
                
                progress_bar.inc(1);
            }
        }
        
        progress_bar.finish();
    }
    
    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let mut rng = rand::thread_rng();
        let offset = Vec3::new(rng.gen::<f64>() - 0.5, rng.gen::<f64>() - 0.5, 0.0);
        let pixel_sample = self.pixel_zero + (self.pixel_delta_u * (j as f64 + offset.x)) + (self.pixel_delta_v * (i as f64 + offset.y));
        
        let ray_origin = self.position;
        let ray_direction = pixel_sample - ray_origin;
        
        Ray::new(ray_origin, ray_direction)
    }
    
    fn ray_color(ray: &Ray, depth: u16, world: &HittableList) -> Color01 {
        if depth <= 0 {
            return Color01::default();
        }
        
        let hit_result = world.hit(ray, 0.001..rt_util::INFINITY);
        if hit_result.is_hit {
            let bounce = hit_result.data.material.scatter(ray, &hit_result);
            
            if bounce.2 {
                return Camera::ray_color(&bounce.0, depth - 1, world) * bounce.1;
            }
            
            return Color01::default();
        }
        
        let ray_dir_norm = ray.direction.normalized();
        let interp = (ray_dir_norm.y + 1.0) * 0.5;
        let color_white = Color01 {r: 1.0, g: 1.0, b: 1.0};
        let color_blue = Color01 {r: 0.5, g: 0.7, b: 1.0};
        
        let color_01 = (1.0 - interp) * color_white + interp * color_blue;
        
        color_01
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
        let ray_bounces_max: u16 = 10;
        
        Camera {
            position,
            aspect_ratio,
            focal_length,
            frame_width,
            samples_per_pixel,
            ray_bounces_max,
            pixel_zero,
            pixel_delta_u,
            pixel_delta_v,
            pixel_samples_scale,
            frame_height,
            frame_res,
        }
    }
}
