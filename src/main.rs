use indicatif::{ProgressBar, ProgressStyle};

pub mod vector;
pub mod ray;

use vector::*;
use ray::*;

const IMG_WIDTH: u32 = 1024;
const IMG_HEIGHT: u32 = 512;
const IMG_RES: u32 = IMG_WIDTH * IMG_HEIGHT;

fn main() {
    print_image_header();
    
    let mult_w: f32 = (256 as f32 / IMG_WIDTH as f32) as f32;
    let mult_h: f32 = (256 as f32 / IMG_HEIGHT as f32) as f32;
    
    let progress_bar = ProgressBar::new(IMG_RES as u64);
    progress_bar.set_style(ProgressStyle::with_template("[{elapsed_precise}] |{bar:40.cyan/blue}| {percent}%")
        .unwrap()
        .progress_chars("=> "));
    
    for i in 0..IMG_HEIGHT {
        for j in 0..IMG_WIDTH {
            let r = (j as f32 * mult_w) as u32;
            let g = (i as f32 * mult_h) as u32;
            let b = 0;
            
            println!("{r} {g} {b}");
            progress_bar.inc(1);
        }
    }
    
    progress_bar.finish();
}

fn print_image_header() {
    println!("P3");
    println!("{IMG_WIDTH} {IMG_HEIGHT}");
    println!("255");
}
