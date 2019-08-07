use std::f32;

use rays::errors::*;
use rays::{Color, HitTest, Ray, Screen, Sphere, Vec3, Camera};

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

fn color(ray: &Ray, hit_test: &impl HitTest) -> Color {
    if let Some(hit_record) = hit_test.hit_test(ray, 0.0, f32::MAX) {
        return (0.5 * Vec3::new(hit_record.normal.x() + 1.0,
        hit_record.normal.y() + 1.0,
        hit_record.normal.z() +1.0)).into();
    } else {
        // Nothing hit. Use background color (blue-to-white gradient).
        let unit_direction = ray.direction().unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (((1.0 - t) * Vec3::new(1.0, 1.0, 1.0)) + (t * Vec3::new(0.5, 0.7, 1.0))).into()
    }
}

fn main() -> Result<()> {
    let mut screen = Screen::new(WIDTH, HEIGHT).unwrap();
    let camera = Camera::new()?;

    screen.run(|fb| {
        let height = fb.height() as f32;
        let width = fb.width() as f32;

        let vec = vec!{
            Sphere::new(&Vec3::new(0.0, 0.0, -1.0), 0.5)?,
            Sphere::new(&Vec3::new(0.0, -100.5, -1.0), 100.0)?
        };
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let u = x as f32 / width;
                let v = y as f32 / height;
                let r = camera.get_ray(u, v);

                let col = color(&r, &vec);
                fb.set(x, y, col);
            }
        }
        Ok(())
    })?;

    Ok(())
}
