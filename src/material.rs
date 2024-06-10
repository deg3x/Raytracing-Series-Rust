use crate::{color::Color01, dot, ray::Ray, rt_util, RayHitResult};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MaterialType {
    Lambert,
    Metal
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Material {
    pub type_info: MaterialType,
    pub albedo: Color01,
    pub fuzziness: f64
}

impl Default for Material {
    fn default() -> Self {
        Material {type_info: MaterialType::Lambert, albedo: Color01::default(), fuzziness: 0.0}
    }
}

impl Material {
    pub fn scatter(&self, ray: &Ray, hit: &RayHitResult) -> (Ray, Color01, bool) {
        match self.type_info {
            MaterialType::Lambert => {
                let mut scatter_dir = hit.data.normal + rt_util::random_in_unit_sphere().normalized();
                if scatter_dir.near_zero() {
                    scatter_dir = hit.data.normal;
                }
                
                let scattered_ray = Ray::new(hit.data.point, scatter_dir);
                let attenuation = self.albedo;
                
                (scattered_ray, attenuation, true)
            },
            MaterialType::Metal => {
                let reflection_dir = ray.direction.reflect(&hit.data.normal) + self.fuzziness * rt_util::random_in_unit_sphere().normalized();
                
                let scattered_ray = Ray::new(hit.data.point, reflection_dir.normalized());
                let attenuation = self.albedo;
                
                (scattered_ray, attenuation, dot(&reflection_dir, &hit.data.normal) > 0.0)
            },
        }
    }
}
