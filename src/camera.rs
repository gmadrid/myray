use crate::errors::*;
use crate::ray::Ray;
use crate::vec3::{cross, Vec3};

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new() -> Result<Camera> {
        Ok(Camera {
            origin: Vec3::new(0.0, 0.0, 0.0),
            lower_left_corner: Vec3::new(-2.0, -1.0, -1.0),
            horizontal: Vec3::new(4.0, 0.0, 0.0),
            vertical: Vec3::new(0.0, 2.0, 0.0),
        })
    }

    pub fn new_with_vert_fov(vfov: f32, aspect: f32) -> Result<Camera> {
        let theta = vfov * std::f32::consts::PI / 180.0;
        let half_height = f32::tan(theta / 2.0);
        let half_width = half_height * aspect;
        Ok(Camera {
            lower_left_corner: Vec3::new(-half_width, -half_height, -1.0),
            horizontal: Vec3::new(2.0 * half_width, 0.0, 0.0),
            vertical: Vec3::new(0.0, 2.0 * half_height, 0.0),
            origin: Vec3::origin(),
        })
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
