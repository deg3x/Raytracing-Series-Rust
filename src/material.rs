use crate::{color::Color01, ray::Ray, RayHitResult};

pub trait Material {
    fn scatter(ray: &Ray, hit: &RayHitResult, attenuation: &mut Color01, scattered_ray: &mut Ray) -> bool;
}
