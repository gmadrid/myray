use std::f32;
use std::fs::File;
use std::path::PathBuf;

use rays::errors::*;
use rays::{
    gradient, load_world, unit_random, Camera, Color, Config, HitTest, IncrementalFrameBuffer, Ray,
    Screen, Vec3, World,
};

use rays::Progress;

fn color(ray: &Ray, hit_test: &impl HitTest, hue: f32, depth: usize, max_depth: usize) -> Color {
    if let Some(hit_record) = hit_test.hit_test(ray, 0.001, f32::MAX) {
        if depth >= max_depth {
            return Color::black();
        }

        if let Some((scattered, attenuation)) = hit_record.material.scatter(ray, &hit_record) {
            return Color::from(
                attenuation * color(&scattered, hit_test, hue, depth + 1, max_depth).as_vec(),
            );
        }
        return Color::black();
    } else {
        let unit_direction = ray.direction().unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        gradient(t, &Color::white(), &Color::from_hsv(hue, 0.5, 1.0).unwrap())
    }
}

fn path_trace_inc(config: &Config, world: &World) -> Result<()> {
    let mut screen = Screen::new(config.screen_width, config.screen_height, config.scale)?;
    let camera = Camera::new_from_to(
        &config.camera_from,
        &config.camera_to,
        &config.camera_up,
        90.0,
        screen.height() as f32 / screen.width() as f32,
    )?;

    let height = screen.height() as f32;
    let width = screen.width() as f32;
    let mut pg = Progress::new(config.num_samples as u64);

    let mut ifb = IncrementalFrameBuffer::new(screen.width(), screen.height())?;

    for n in 0..config.num_samples {
        screen.one_frame(|fb| {
            for y in 0..config.screen_height {
                for x in 0..config.screen_width {
                    let color = sample_color(config, &world, &camera, x, y, width, height);
                    ifb.set(x, y, color);
                }
            }
            ifb.copy_to_fb(n as u32, fb);
            Ok(())
        })?;

        pg.inc();
    }
    pg.finish_and_clear();

    screen.wait()
}

fn sample_color(
    config: &Config,
    world: &World,
    camera: &Camera,
    x: usize,
    y: usize,
    width: f32,
    height: f32,
) -> Color {
    let u = (x as f32 + unit_random()) / width;
    let v = (y as f32 + unit_random()) / height;
    let ray = camera.get_ray(u, v);
    color(&ray, world, config.hue, 0, config.max_depth)
}

fn path_trace(config: &Config, world: &World) -> Result<()> {
    let mut screen = Screen::new(config.screen_width, config.screen_height, config.scale)?;
    let camera = Camera::new_from_to(
        &config.camera_from,
        &config.camera_to,
        &config.camera_up,
        90.0,
        screen.height() as f32 / screen.width() as f32,
    )?;

    screen.one_frame(|fb| {
        let height = fb.height() as f32;
        let width = fb.width() as f32;

        let mut pg = Progress::new((height * width) as u64);
        for y in 0..config.screen_height {
            for x in 0..config.screen_width {
                pg.inc();
                let mut color_vec = Vec3::new(0.0, 0.0, 0.0);
                for _ in 0..config.num_samples {
                    color_vec = color_vec
                        + sample_color(config, &world, &camera, x, y, width, height).as_vec();
                }

                fb.set(x, y, Color::from(color_vec / config.num_samples as f32));
            }
        }
        pg.finish_and_clear();
        Ok(())
    })?;

    screen.wait()?;

    Ok(())
}

fn add_extension_if_missing(path: &PathBuf, ext: &str) -> PathBuf {
    let mut result = path.clone();
    if path.extension().is_none() {
        result.set_extension(ext);
    }
    result
}

fn get_world(config: &Config) -> Result<World> {
    if let Some(files) = &config.worlds {
        files.into_iter().try_fold(vec![], |v, filename| {
            let file = File::open(filename)?;
            let small_world = serde_yaml::from_reader::<_, World>(file)?;
            Ok(v.into_iter()
                .chain(small_world.into_iter())
                .collect::<World>())
        })
    } else {
        load_world(config.world)
    }
}

fn real_main() -> Result<()> {
    let config = Config::new()?;
    let world = get_world(&config)?;

    if let Some(write) = &config.write {
        let filename = add_extension_if_missing(&write, "yaml");
        let file = File::create(filename).unwrap(); // TODO
        serde_yaml::to_writer(file, &world).unwrap(); // TODO
    }

    path_trace_inc(&config, &world)?;

    Ok(())
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

