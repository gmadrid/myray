use std::f32;

use rays::errors::*;
use rays::{unit_random, random_in_unit_sphere};
use rays::{gradient, Camera, Color, HitTest, Ray, Screen, Sphere, Vec3};

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

const NUM_SAMPLES: usize = 4;

const BACKGROUND_HUE: f32 = 205.0;

fn color(ray: &Ray, hit_test: &impl HitTest/*, depth: usize*/) -> Color {
    if let Some(hit_record) = hit_test.hit_test(ray, 0.001, f32::MAX) {
//        if depth >= 50 {
//            return Color::black();
//        }

/*        if let Some((attenuation, scattered)) = hit_record.material.scatter(ray, hit_record) {
            return Color::from(attenuation * color(scattered, hit_test, depth + 1).as_vec());
        }
        return Color::black();*/

    let target = hit_record.point + hit_record.normal + random_in_unit_sphere();
    return Color::from(
        0.5 * color(
            &Ray::new(hit_record.point, target - hit_record.point),
            hit_test
        ).as_vec());
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

    //    let mut rng = thread_rng();
    //    let jitter = Uniform::new(0.0, 1.0);

    screen.one_frame(|fb| {
        let height = fb.height() as f32;
        let width = fb.width() as f32;

        let vec = vec![
            Sphere::new(&Vec3::new(0.0, 0.0, -1.0), 0.5)?,
            Sphere::new(&Vec3::new(0.0, -100.5, -1.0), 100.0)?,
        ];
        let total_iters = (HEIGHT * WIDTH) as f32;
        let mut count = 0;

        println!("");
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                print!("\r{:.2}%", (count * 100) as f32 / total_iters);
                count += 1;
                let mut color_vec = Vec3::new(0.0, 0.0, 0.0);
                for _ in 0..NUM_SAMPLES {
                    let u = (x as f32 + unit_random()) / width;
                    let v = (y as f32 + unit_random()) / height;
                    let ray = camera.get_ray(u, v);
                    color_vec = color_vec + color(&ray, &vec/*, 0*/).as_vec();
                }

                fb.set(x, y, Color::from(color_vec / NUM_SAMPLES as f32));
            }
        }
        print!("\r                \n");
        Ok(())
    })?;

    screen.wait()?;

    Ok(())
}
