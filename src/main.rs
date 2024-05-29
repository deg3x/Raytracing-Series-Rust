use indicatif::{ProgressBar, ProgressStyle};

pub mod vector;
pub mod ray;

use vector::*;
use ray::*;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMG_WIDTH: u32 = 1024;
const IMG_HEIGHT: u32 = (IMG_WIDTH as f64 / ASPECT_RATIO) as u32;
const IMG_RES: u32 = IMG_WIDTH * IMG_HEIGHT;

const FOCAL_LEN: f64 = 1.0;
const CAMERA_CENTER: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
const VIEW_HEIGHT: f64 = 2.0;
const VIEW_WIDTH: f64 = VIEW_HEIGHT * (IMG_WIDTH as f64 / IMG_HEIGHT as f64);
const VIEW_U: Vec3 = Vec3 { x: VIEW_WIDTH, y: 0.0, z: 0.0 };
const VIEW_V: Vec3 = Vec3 { x: 0.0, y: -VIEW_HEIGHT, z: 0.0 };
const PX_DELTA_U: f64 = VIEW_U.x / IMG_WIDTH as f64;
const PX_DELTA_V: f64 = VIEW_V.y / IMG_HEIGHT as f64;
// const VIEW_PX_UL: Vec3 = CAMERA_CENTER - Vec3 { x: 0.0, y: 0.0, z: FOCAL_LEN } - VIEW_U * 0.5 - VIEW_V * 0.5;
// const PX00: Vec3 = VIEW_PX_UL + 0.5 * (PX_DELTA_U + PX_DELTA_V);

fn main() {
    let VIEW_PX_UL: Vec3 = CAMERA_CENTER - Vec3 { x: 0.0, y: 0.0, z: FOCAL_LEN } - VIEW_U * 0.5 - VIEW_V * 0.5;
    let PX00: Vec3 = VIEW_PX_UL + 0.5 * (PX_DELTA_U + PX_DELTA_V);
    
    assert!(IMG_HEIGHT > 1);
    
    print_image_header();
    
    let mult_w: f32 = (256 as f32 / IMG_WIDTH as f32) as f32;
    let mult_h: f32 = (256 as f32 / IMG_HEIGHT as f32) as f32;
    
    let progress_bar = ProgressBar::new(IMG_RES as u64);
    progress_bar.set_style(ProgressStyle::with_template("[{elapsed_precise}] |{bar:40.cyan/blue}| {percent}%")
        .unwrap()
        .progress_chars("=> "));
    
    for i in 0..IMG_HEIGHT {
        for j in 0..IMG_WIDTH {
            let px_center = PX00 + j as f64 * PX_DELTA_U + i as f64 * PX_DELTA_V;
            let ray_dir = CAMERA_CENTER - px_center;
            
            let ray: Ray = Ray::new(CAMERA_CENTER, ray_dir);
            
            let color = ray_color(&ray);
            
            println!("{} {} {}", color.x, color.y, color.z);
            progress_bar.inc(1);
        }
    }
    
    progress_bar.finish();
}

fn ray_color(ray: &Ray) -> Vec3 {
    Vec3 { x: 0.0, y: 0.0, z: 0.0}
}

fn print_image_header() {
    println!("P3");
    println!("{IMG_WIDTH} {IMG_HEIGHT}");
    println!("255");
}
