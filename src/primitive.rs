use crate::vector::*;
use crate::ray::*;

pub struct RayHitData {
    pub point: Vec3,
    pub normal: Vec3,
    pub ray_t: f64,
    pub front_face: bool
}

impl RayHitData {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = dot(&ray.direction, outward_normal) > 0.0;
        if self.front_face {
            self.normal = *outward_normal * -1.0;
        }
        else {
            self.normal = *outward_normal;
        }
    }
}

pub struct RayHitResult {
    pub is_hit: bool,
    pub data: RayHitData
}

impl Default for RayHitResult {
    fn default() -> Self {
        Self {
            is_hit: false,
            data: RayHitData {
                point: Vec3 {x: 0.0, y: 0.0, z: 0.0},
                normal: Vec3 {x: 0.0, y: 0.0, z: 0.0},
                ray_t: -1.0,
                front_face: false
            }
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> RayHitResult;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> RayHitResult {
        let mut result = RayHitResult::default();
        
        let sphere_to_ray = ray.origin - self.center;
        
        let a = ray.direction.len_sqr();
        let b = dot(&ray.direction, &sphere_to_ray);
        let c = sphere_to_ray.len_sqr() - self.radius * self.radius;
        
        let discriminant = b * b - a * c;
        
        if discriminant < 0.0 {
            return result;
        }
        
        let disc_sqrt = f64::sqrt(discriminant);
        
        let mut t = (-b - disc_sqrt) / a;
        if t < t_min || t > t_max {
            t = (-b + disc_sqrt) / a;
            
            if t < t_min || t > t_max {
                return result;
            }
        }
        
        result.is_hit = true;
        result.data.ray_t = t;
        result.data.point = ray.at(result.data.ray_t);
        
        let normal = (result.data.point - self.center) * (1.0 / self.radius);
        result.data.set_face_normal(ray, &normal);
        
        result
    }
}

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new()
        }
    }
    
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
    
    pub fn clear(&mut self) {
        self.objects.clear();
    }
    
    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> RayHitResult {
        let mut hit: RayHitResult = RayHitResult::default();
        let mut closest_t = t_max;
        
        for object in self.objects.iter() {
            let hit_result = object.hit(ray, t_min, closest_t);
            if hit_result.is_hit {
                closest_t = hit_result.data.ray_t;
                hit = hit_result;
            }
        }
        
        hit
    }
}
