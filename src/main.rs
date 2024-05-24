const IMG_WIDTH: u16 = 512;
const IMG_HEIGHT: u16 = 512;

fn main() {
    print_image_header();
    
    let mult_w: f32 = (256 as f32 / IMG_WIDTH as f32) as f32;
    let mult_h: f32 = (256 as f32 / IMG_HEIGHT as f32) as f32;
    
    for i in 0..IMG_WIDTH {
        for j in 0..IMG_HEIGHT {
            let r = (j as f32 * mult_w) as u32;
            let g = (i as f32 * mult_h) as u32;
            let b = 0;
            
            println!("{r} {g} {b}");
        }
    }
}

fn print_image_header() {
    println!("P3");
    println!("{IMG_WIDTH} {IMG_HEIGHT}");
    println!("255");
}
