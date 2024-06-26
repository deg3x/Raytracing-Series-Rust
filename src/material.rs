use std::ops::Neg;

use crate::primitive::RayHitResult;
use crate::color::Color01;
use crate::rt_util;
use crate::vector::*;
use crate::ray::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MaterialType {
    Lambert,
    Metal,
    Dielectric
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Material {
    pub type_info: MaterialType,
    pub albedo: Color01,
    pub fuzziness: f64,
    pub refraction_idx: f64
}

impl Default for Material {
    fn default() -> Self {
        Material {type_info: MaterialType::Lambert, albedo: Color01::default(), fuzziness: 0.0, refraction_idx: 1.0}
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
            MaterialType::Dielectric => {
                let refr_factor = if hit.data.front_face {
                    1.0 / self.refraction_idx
                }
                else {
                    self.refraction_idx
                };
                
                let ray_dir_norm = &ray.direction.normalized();
                let cos_theta = f64::min(dot(&ray_dir_norm.neg(), &hit.data.normal), 1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
                
                let out_dir: Vec3;
                if sin_theta * refr_factor > 1.0 || schlick_reflectance(cos_theta, refr_factor) > rt_util::random() {
                    out_dir = ray_dir_norm.reflect(&hit.data.normal);
                }
                else {
                    out_dir = ray_dir_norm.refract(&hit.data.normal, refr_factor);
                }
                
                let out_ray = Ray::new(hit.data.point, out_dir);
                
                (out_ray, Color01::new(1.0, 1.0, 1.0), true)
            },
        }
    }
}

fn schlick_reflectance(cosine: f64, refr_idx: f64) -> f64 {
    let mut r0 = (1.0 - refr_idx) / (1.0 + refr_idx);
    r0 = r0 * r0;
    
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
