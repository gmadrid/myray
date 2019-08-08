use crate::errors::*;
use crate::hittest::{HitRecord, HitTest};
//use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{dot, Vec3};

pub struct Sphere {
    center: Vec3,
    radius: f32,
    //    material: Box<dyn Material>,
}

impl Sphere {
    // radius must be > 0.0
    pub fn new(center: &Vec3, radius: f32) -> Result<Sphere>
/* where M: Material */ {
        if radius <= 0.0 {
            Err(ErrorKind::InvalidParam(radius, "radius must be > 0.0".into()).into())
        } else {
            Ok(Sphere {
                center: *center,
                radius,
                //                material: Box::new(material),
            })
        }
    }
}

impl HitTest for Sphere {
    fn hit_test(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = *ray.origin() - self.center;
        let a = dot(ray.direction(), ray.direction());
        let b = dot(&oc, ray.direction());
        let c = dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let temp = (-b - f32::sqrt(b * b - a * c)) / a;
            if temp < t_max && temp > t_min {
                let point = ray.point_at(temp);
                let normal = (point - self.center) / self.radius;
                return Some(HitRecord {
                    t: temp,
                    point,
                    normal,
                    //                    material: &self.material,
                });
            }
            let temp = (-b + f32::sqrt(b * b - a * c)) / a;
            if temp < t_max && temp > t_min {
                // TODO: DRY
                let point = ray.point_at(temp);
                let normal = (point - self.center) / self.radius;
                return Some(HitRecord {
                    t: temp,
                    point,
                    normal,
                    //                    material: &self.material,
                });
            }
        }
        return None;
    }
}
