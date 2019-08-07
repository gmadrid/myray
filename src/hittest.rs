use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct HitRecord {
    pub t: f32,
    pub point: Vec3,
    pub normal: Vec3,
}

pub trait HitTest {
    fn hit_test(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

impl<T> HitTest for Vec<T> where T: HitTest {
    fn hit_test(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit_record = None;
        let mut closest_so_far = t_max;
        for test in self {
            if let Some(hit) = test.hit_test(ray, t_min, closest_so_far) {
                closest_so_far = hit.t;
                hit_record = Some(hit);
            }
        }
        hit_record
    }
}