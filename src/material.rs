use serde::{Serialize, Deserialize};

use crate::color::Color;
use crate::hittest::HitRecord;
use crate::ray::Ray;
use crate::unit_random::unit_random;
use crate::util::{if_then, random_in_unit_sphere};
use crate::vec3::{dot, Vec3};

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)>;
}

#[derive(Serialize, Deserialize)]
pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian {
            albedo: albedo.as_vec(),
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)> {
        let target = hit_record.point + hit_record.normal + random_in_unit_sphere();
        let scattered = Ray::new(hit_record.point, target - hit_record.point);
        Some((scattered, self.albedo))
    }
}

#[derive(Serialize, Deserialize)]
pub struct Metal {
    albedo: Vec3,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Metal {
            albedo: albedo.as_vec(),
        }
    }
}

fn reflect(vec: &Vec3, normal: &Vec3) -> Vec3 {
    vec - 2.0 * dot(vec, normal) * normal
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)> {
        let reflected = reflect(&ray.direction().unit_vector(), &hit_record.normal);
        let scattered = Ray::new(hit_record.point, reflected);
        let attenuation = self.albedo;
        if_then(dot(scattered.direction(), &hit_record.normal) > 0.0, || {
            Some((scattered, attenuation))
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct Dielectric {
    refractive_index: f32,
}

impl Dielectric {
    pub fn new(refractive_index: f32) -> Dielectric {
        Dielectric { refractive_index }
    }
}

fn refract(vec: &Vec3, normal: &Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv = vec.unit_vector();
    let dt = dot(&uv, normal);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if_then(discriminant > 0.0, || {
        Some(ni_over_nt * (uv - normal * dt) - normal * f32::sqrt(discriminant))
    })
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0sq = r0 * r0;
    r0sq + (1.0 - r0sq) * f32::powi(1.0 - cosine, 5)
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)> {
        let dotp = dot(ray.direction(), &hit_record.normal);
        let (outward_normal, ni_over_nt, cosine) = if dotp > 0.0 {
            (
                -hit_record.normal,
                self.refractive_index,
                self.refractive_index * dotp / ray.direction().length(),
            )
        } else {
            (
                hit_record.normal,
                1.0 / self.refractive_index,
                -dotp / ray.direction().length(),
            )
        };

        let mut refracted = None;
        let reflect_prob =
            if let Some(refracted_val) = refract(ray.direction(), &outward_normal, ni_over_nt) {
                refracted = Some(refracted_val);
                schlick(cosine, self.refractive_index)
            } else {
                1.0
            };

        let scattered = if unit_random() < reflect_prob {
            Ray::new(
                hit_record.point,
                reflect(ray.direction(), &hit_record.normal),
            )
        } else {
            // refracted will always take the other path when it == None because the
            // reflect_prob will == 1.0
            // TODO: make this code more clear and less prone to problems.
            Ray::new(hit_record.point, refracted.unwrap())
        };

        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        Some((scattered, attenuation))
    }
}
