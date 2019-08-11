use std::f32;

use indicatif::ProgressBar;

use rays::errors::*;
use rays::{
    gradient, unit_random, Camera, Color, Config, Dielectric, HitTest, Lambertian, Metal, Ray,
    Screen, Sphere, Vec3,
};

const BACKGROUND_HUE: f32 = 205.0;

fn color(ray: &Ray, hit_test: &impl HitTest, depth: usize, max_depth: usize) -> Color {
    if let Some(hit_record) = hit_test.hit_test(ray, 0.001, f32::MAX) {
        if depth >= max_depth {
            return Color::black();
        }

        if let Some((scattered, attenuation)) = hit_record.material.scatter(ray, &hit_record) {
            return Color::from(
                attenuation * color(&scattered, hit_test, depth + 1, max_depth).as_vec(),
            );
        }
        return Color::black();
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

fn path_trace(config: &Config) -> Result<()> {
    let mut screen = Screen::new(config.screen_width, config.screen_height, config.scale).unwrap();
    let camera = Camera::new()?;
    // let camera = Camera::new_from_to(&Vec3::new(-2.0, 2.0, 1.0),
    //                                  &Vec3::new(0.0, 0.0, -1.0),
    //                                  &Vec3::new(0.0, 1.0, 0.0),
    //                                  45.0,
    //                                  screen.height() as f32 / screen.width() as f32)?;

    screen.one_frame(|fb| {
        let height = fb.height() as f32;
        let width = fb.width() as f32;

        //        let r = f32::cos(std::f32::consts::PI / 4.0);
        let vec = vec![
            Sphere::new(
                &Vec3::new(0.0, 0.0, -1.0),
                0.5,
                Lambertian::new(Color::new(0.8, 0.3, 0.3)?),
            )?,
            Sphere::new(
                &Vec3::new(0.0, -100.5, -1.0),
                100.0,
                Lambertian::new(Color::new(0.3, 0.3, 0.8)?),
            )?,
            Sphere::new(
                &Vec3::new(1.0, 0.0, -1.0),
                0.5,
                Metal::new(Color::new(0.8, 0.6, 0.2)?),
            )?,
            Sphere::new(&Vec3::new(-1.0, 0.0, -1.0), 0.5, Dielectric::new(1.5))?,
            // Sphere::new(&Vec3::new(-r, 0.0, -1.0), r, Lambertian::new(Color::blue()))?,
            // Sphere::new(&Vec3::new(r, 0.0, -1.0), r, Lambertian::new(Color::red()))?,
        ];

        let pb = ProgressBar::new((height * width) as u64);

        for y in 0..config.screen_height {
            for x in 0..config.screen_width {
                pb.inc(1);
                let mut color_vec = Vec3::new(0.0, 0.0, 0.0);
                for _ in 0..config.num_samples {
                    let u = (x as f32 + unit_random()) / width;
                    let v = (y as f32 + unit_random()) / height;
                    let ray = camera.get_ray(u, v);
                    color_vec = color_vec + color(&ray, &vec, 0, config.max_depth).as_vec();
                }

                fb.set(x, y, Color::from(color_vec / config.num_samples as f32));
            }
        }
        pb.finish_and_clear();
        Ok(())
    })?;

    screen.wait()?;

    Ok(())
}

fn real_main() -> Result<()> {
    let config = Config::new()?;
    path_trace(&config)
}

fn main() {
    match real_main() {
        Ok(_) => (),
        Err(err) => {
            match err {
                // Clap gets special attention. ('-h' for example is better handled by clap.)
                Error(ErrorKind::ClapError(ce), _) => ce.exit(),
                _ => println!("Error: {}", err),
            }
        }
    }
}
