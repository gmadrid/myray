use std::str::FromStr;

use crate::color::Color;
use crate::errors::*;
use crate::material::{Lambertian, Metal, Dielectric};
use crate::unit_random::unit_random;
use crate::sphere::Sphere;
use crate::vec3::Vec3;

pub type World = Vec<Sphere>;

#[derive(Debug, Copy, Clone)]
pub enum Worlds {
    ThreeBalls,
    Random,
}

impl FromStr for Worlds {
    type Err = Error;

    fn from_str(s: &str) -> Result<Worlds> {
        match s.to_lowercase().as_str() {
            "threeballs" => Ok(Worlds::ThreeBalls),
            "random" => Ok(Worlds::Random),
            _ => Err(ErrorKind::ParseError(
                s.to_string(),
                "Must be 'threeballs' or 'random'.".to_string(),
            )
            .into()),
        }
    }
}

pub fn load_world(world: Worlds) -> World {
    match world {
        Worlds::ThreeBalls => three_balls(),
        Worlds::Random => random_scene(),
    }.unwrap()
}

fn three_balls() -> Result<World> {
    Ok(vec![
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
    ])
}

fn random_scene() -> Result<World> {
    let mut world = Vec::new();

    world.push(Sphere::new(
        &Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::new(Color::new(0.5, 0.5, 0.5)?),
    )?);

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = unit_random();
            let center = Vec3::new(
                a as f32 + 0.9 * unit_random(),
                0.2,
                b as f32 + 0.9 + unit_random(),
            );
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    world.push(Sphere::new(
                        &center,
                        0.2,
                        Lambertian::new(Color::new(
                            unit_random() * unit_random(),
                            unit_random() * unit_random(),
                            unit_random() * unit_random(),
                        )?),
                    )?);
                } else if choose_mat < 0.95 {
                    world.push(Sphere::new(
                        &center,
                        0.2,
                        Metal::new(Color::new(
                            0.5 * (1.0 + unit_random()),
                            0.5 * (1.0 + unit_random()),
                            0.5 * (1.0 + unit_random()),
                        )?),
                    )?);
                }
            }
        }
    }

    world.push(Sphere::new(
        &Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Dielectric::new(1.5),
    )?);
    world.push(Sphere::new(
        &Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Lambertian::new(Color::new(0.4, 0.2, 0.1)?),
    )?);
    world.push(Sphere::new(
        &Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Metal::new(Color::new(0.7, 0.6, 0.5)?),
    )?);

    Ok(world)
}

