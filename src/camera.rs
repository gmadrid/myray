use crate::errors::*;
use crate::ray::Ray;
use crate::vec3::{cross, Vec3};

#[derive(Debug)]
pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new() -> Result<Camera> {
        Camera::new_with_vert_fov(90.0, 2.0)
    }

    pub fn new_with_vert_fov(vfov: f32, aspect: f32) -> Result<Camera> {
        Camera::new_from_to(
            &Vec3::new(0.0, 0.0, 0.0),
            &Vec3::new(0.0, 0.0, -1.0),
            &Vec3::new(0.0, 1.0, 0.0),
            vfov,
            aspect,
        )
    }

    pub fn new_from_to(
        lookfrom: &Vec3,
        lookat: &Vec3,
        vup: &Vec3,
        vfov: f32,
        aspect: f32,
    ) -> Result<Camera> {
        let theta = vfov * std::f32::consts::PI / 180.0;
        let half_height = f32::tan(theta / 2.0);
        let half_width = half_height * aspect;
        let origin = lookfrom;
        let w = (lookfrom - lookat).unit_vector();
        let u = cross(&vup, &w).unit_vector();
        let v = cross(&w, &u);

        Ok(Camera {
            lower_left_corner: origin - half_width * u - half_height * v - w,
            horizontal: 2.0 * half_width * u,
            vertical: 2.0 * half_height * v,
            origin: *origin,
        })
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
