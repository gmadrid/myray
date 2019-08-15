use serde::{Serialize, Deserialize};

use crate::errors::*;
use crate::hittest::{HitRecord, HitTest};
use crate::material::Material;
use crate::ray::Ray;
use crate::util::if_then;
use crate::vec3::{dot, Vec3};

#[derive(Serialize, Deserialize)]
pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Box<dyn Material>,
}

impl Sphere {
    // radius must be > 0.0
    pub fn new<M>(center: &Vec3, radius: f32, material: M) -> Result<Sphere>
    where
        M: Material + 'static,
    {
        if radius <= 0.0 {
            Err(ErrorKind::InvalidParam(radius, "radius must be > 0.0".into()).into())
        } else {
            Ok(Sphere {
                center: *center,
                radius,
                material: Box::new(material),
            })
        }
    }
}

impl HitTest for Sphere {
    fn hit_test(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = dot(ray.direction(), ray.direction());
        let b = dot(&oc, ray.direction());
        let c = dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        let check_hit = |sign| {
            let t = (-b + sign * f32::sqrt(discriminant)) / a;
            if_then(t < t_max && t > t_min, || {
                let point = ray.point_at(t);
                let normal = (point - self.center) / self.radius;
                Some(HitRecord {
                    t,
                    point,
                    normal,
                    material: self.material.as_ref(),
                })
            })
        };

        if_then(discriminant > 0.0, || {
            check_hit(-1.0).or_else(|| check_hit(1.0))
        })
    }
}
