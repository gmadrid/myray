use crate::color::Color;
use crate::hittest::HitRecord;
use crate::ray::Ray;
use crate::util::random_in_unit_sphere;
use crate::vec3::{dot, Vec3};

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)>;
}

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
    *vec - 2.0 * dot(vec, normal) * *normal
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)> {
        let reflected = reflect(&ray.direction().unit_vector(), &hit_record.normal);
        let scattered = Ray::new(hit_record.point, reflected);
        let attenuation = self.albedo;
        if dot(scattered.direction(), &hit_record.normal) > 0.0 {
            Some((scattered, attenuation))
        } else {
            None
        }
    }
}
