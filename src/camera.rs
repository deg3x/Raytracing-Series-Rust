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

impl Camera {
    pub fn new() -> Self {
        Camera {
            position: Vec3 {x: 0.0, y: 0.0, z: 0.0},
            aspect_ratio: 1.0,
            frame_width: 1024,
            frame_height: 1024,
            frame_res: 1024 * 1024,
            focal_length: 1.0,
            view_height: 2.0,
            view_width: 2.0,
            view_u: Vec3 {x: 2.0, y: 0.0, z: 0.0},
            view_v: Vec3 {x: 0.0, y: -2.0, z: 0.0},
            pixel_zero: Vec3 {x: 0.0, y: 0.0, z: 0.0},
            pixel_delta_u: Vec3 {x: 0.0, y: 0.0, z: 0.0},
            pixel_delta_v: Vec3 {x: 0.0, y: 0.0, z: 0.0},
            view_pixel_upper_left: Vec3 {x: 0.0, y: 0.0, z: 0.0},
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        let mut camera: Camera = Camera::new();
        
        camera.aspect_ratio = 16.0 / 9.0;
        camera.frame_width = 1024;
        camera.frame_height = (camera.frame_width as f64 / camera.aspect_ratio) as u32;
        camera.frame_res = camera.frame_width * camera.frame_height;
        
        camera.focal_length = 1.0;
        camera.view_height = 2.0;
        camera.view_width = camera.view_height * (camera.frame_width as f64 / camera.frame_height as f64);
        camera.view_u = Vec3::new(camera.view_width, 0.0, 0.0);
        camera.view_v = Vec3::new(0.0, -camera.view_height, 0.0);
        camera.pixel_delta_u = camera.view_u * (1.0 / camera.frame_width as f64);
        camera.pixel_delta_v = camera.view_v * (1.0 / camera.frame_height as f64);
        camera.view_pixel_upper_left = camera.position - Vec3::new(0.0, 0.0, camera.focal_length) - (camera.view_u + camera.view_v) * 0.5;
        camera.pixel_zero = camera.view_pixel_upper_left + (camera.pixel_delta_u + camera.pixel_delta_v) * 0.5;
        
        camera
    }
}
