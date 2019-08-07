use std::f32;

use rays::errors::*;
use rays::{dot, Color, Ray, Screen, Vec3};

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

fn hit_sphere(center: &Vec3, radius: f32, ray: &Ray) -> Option<f32> {
    let oc = *ray.origin() - *center;
    let a = dot(ray.direction(), ray.direction());
    let b = 2.0 * dot(&oc, ray.direction());
    let c = dot(&oc, &oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        None
    } else {
        Some((-b - f32::sqrt(discriminant)) / (2.0 * a))
    }
}

fn color(ray: &Ray) -> Color {
    if let Some(t) = hit_sphere(&Vec3::new(0.0, 0.0, -1.0), 0.5, ray) {
        let n = (ray.point_at(t) - Vec3::new(0.0, 0.0, -1.0)).unit_vector();
        (0.5 * Vec3::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0)).into()
    } else {
        let unit_direction = ray.direction().unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (((1.0 - t) * Vec3::new(1.0, 1.0, 1.0)) + (t * Vec3::new(0.5, 0.7, 1.0))).into()
    }
}

fn main() -> Result<()> {
    let mut screen = Screen::new(WIDTH, HEIGHT).unwrap();

    screen.run(|fb| {
        let height = fb.height() as f32;
        let width = fb.width() as f32;

        let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
        let horiz = Vec3::new(4.0, 0.0, 0.0);
        let vert = Vec3::new(0.0, 2.0, 0.0);
        let origin = Vec3::new(0.0, 0.0, 0.0);

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let u = x as f32 / width;
                let v = y as f32 / height;
                let r = Ray::new(origin, lower_left_corner + u * horiz + v * vert);
                let col = color(&r);
                fb.set(x, y, col);
            }
        }
        Ok(())
    })?;

    Ok(())
}
