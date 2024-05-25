use indicatif::{ProgressBar, ProgressStyle};

const IMG_WIDTH: u32 = 1024;
const IMG_HEIGHT: u32 = 512;
const IMG_RES: u32 = IMG_WIDTH * IMG_HEIGHT;

struct Vec3 {
    x: f64,
    y: f64,
    z: f64
}

impl Vec3 {
    fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {x: x, y: y, z: z}
    }
    
    fn len_sqr(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    
    fn len(&self) -> f64 {
        let squared = self.len_sqr();
        
        squared.sqrt()
    }
    
    fn normalized(&self) -> Vec3 {
        let inv_len = 1f64 / self.len();
        
        Vec3 {x: self.x * inv_len, y: self.y * inv_len, z: self.z * inv_len}
    }
}

fn dot(a: Vec3, b: Vec3) -> f64 {
    a.x * b.x + a.y * b.y + a.z * b.z
}

fn cross(a: Vec3, b: Vec3) -> Vec3 {
    
    Vec3 { 
        x: a.y * b.z - b.y * a.z,
        y: a.x * b.z - b.x * a.z,
        z: a.x * b.y - b.x * a.y
    }
}

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
