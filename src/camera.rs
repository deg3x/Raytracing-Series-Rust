use crate::vector::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Camera {
    pub position: Vec3,
    
    pub aspect_ratio: f64,
    pub frame_width: u32,
    pub frame_height: u32,
    pub frame_res: u32,
    
    pub focal_length: f64,
    pub view_height: f64,
    pub view_width: f64,
    pub view_u: Vec3,
    pub view_v: Vec3,
    pub view_pixel_upper_left: Vec3,
    pub pixel_zero: Vec3,
    
    pub pixel_delta_u: Vec3,
    pub pixel_delta_v: Vec3
}

impl Default for Camera {
    fn default() -> Self {
        let position = Vec3 {x: 0.0, y: 0.0, z: 0.0};
        let aspect_ratio = 16.0 / 9.0;
        let frame_width = 1024;
        let frame_height = (frame_width as f64 / aspect_ratio) as u32;
        let frame_res = frame_width * frame_height;
        let focal_length = 1.0;
        let view_height = 2.0;
        let view_width = view_height * (frame_width as f64 / frame_height as f64);
        let view_u = Vec3::new(view_width, 0.0, 0.0);
        let view_v = Vec3::new(0.0, -view_height, 0.0);
        let pixel_delta_u = view_u * (1.0 / frame_width as f64);
        let pixel_delta_v = view_v * (1.0 / frame_height as f64);
        let view_pixel_upper_left = position - Vec3::new(0.0, 0.0, focal_length) - (view_u + view_v) * 0.5;
        let pixel_zero = view_pixel_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;
        
        Camera {
            position,
            aspect_ratio,
            frame_width,
            frame_height,
            frame_res,
            focal_length,
            view_height,
            view_width,
            view_u,
            view_v,
            pixel_zero,
            pixel_delta_u,
            pixel_delta_v,
            view_pixel_upper_left,
        }
    }
}
