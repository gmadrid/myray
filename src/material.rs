/*
use crate::hittest::HitRecord;
use crate::ray::Ray;
use crate::util::random_in_unit_sphere;
use crate::vec3::Vec3;

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)>;
}

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)> {
        let target = hit_record.point + hit_record.normal + random_in_unit_sphere();
        let scattered = Ray::new(hit_record.point, target - hit_record.point);
        Some((scattered, self.albedo))
    }
}
*/
