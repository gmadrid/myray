use std::f32;

use rand::distributions::Uniform;
use rand::{thread_rng, Rng};

use rays::errors::*;
use rays::{gradient, Camera, Color, HitTest, Ray, Screen, Sphere, Vec3};

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

const NUM_SAMPLES: usize = 10;

const BACKGROUND_HUE: f32 = 205.0;

fn color(ray: &Ray, hit_test: &impl HitTest) -> Color {
    if let Some(hit_record) = hit_test.hit_test(ray, 0.0, f32::MAX) {
        return (0.5
            * Vec3::new(
                hit_record.normal.x() + 1.0,
                hit_record.normal.y() + 1.0,
                hit_record.normal.z() + 1.0,
            ))
        .into();
    } else {
        let unit_direction = ray.direction().unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        gradient(
            t,
            &Color::white(),
            &Color::from_hsv(BACKGROUND_HUE, 0.5, 1.0).unwrap(),
        )
    }
}

fn main() -> Result<()> {
    let mut screen = Screen::new(WIDTH, HEIGHT).unwrap();
    let camera = Camera::new()?;

    let mut rng = thread_rng();
    let jitter = Uniform::new(0.0, 1.0);

    screen.run(|fb| {
        let height = fb.height() as f32;
        let width = fb.width() as f32;

        let vec = vec![
            Sphere::new(&Vec3::new(0.0, 0.0, -1.0), 0.5)?,
            Sphere::new(&Vec3::new(0.0, -100.5, -1.0), 100.0)?,
        ];
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let mut color_vec = Vec3::new(0.0, 0.0, 0.0);
                for _ in 0..NUM_SAMPLES {
                    let u = (x as f32 + rng.sample(jitter)) / width;
                    let v = (y as f32 + rng.sample(jitter)) / height;
                    let ray = camera.get_ray(u, v);
                    color_vec = color_vec + color(&ray, &vec).as_vec();
                }

                fb.set(x, y, Color::from(color_vec / NUM_SAMPLES as f32));
            }
        }
        Ok(())
    })?;

    Ok(())
}
