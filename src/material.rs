use crate::{color::Color01, ray::Ray, rt_util, RayHitResult};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Material {
    pub albedo: Color01
}

impl Default for Material {
    fn default() -> Self {
        Material {albedo: Color01::default()}
    }
}

impl Material {
    fn scatter(&self, ray: &Ray, hit: &RayHitResult) -> (Ray, Color01, bool) {
        let mut scatter_dir = hit.data.normal + rt_util::random_in_unit_sphere();
        
        if scatter_dir.near_zero() {
            scatter_dir = hit.data.normal;
        }
        
        let scattered_ray = Ray::new(hit.data.point, scatter_dir);
        let attenuation = self.albedo;
        
        (scattered_ray, attenuation, true)
    }
}
