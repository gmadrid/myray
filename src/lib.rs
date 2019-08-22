#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate impl_ops;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate structopt;

pub mod errors {
    error_chain! {
        errors {
            InvalidParam(val: f32, t: String) {
                description("Value invalid")
                display("Value invalid ({}): {}", t, val)
            }
            MissingParam(val: String) {
                description("Missing command line argument.")
                display("'{}' must be specified (or have a default).", val)
            }
            OutOfRange(val: f32, min: f32, max: f32) {
                description("Value out of range.")
                display("Value, {}, out of range: [{}, {}]", val, min, max)
            }
            ParseError(val: String, t: String) {
                description("Parse error")
                display("Cannot parse ({}): \"{}\"", t, val)
            }
        }
        foreign_links {
            IoError(std::io::Error);
            MiniFBError(minifb::Error);
            ParseIntError(std::num::ParseIntError);
            ParseFloatError(std::num::ParseFloatError);
            SerdeYamlError(serde_yaml::Error);
        }
    }

}

pub use camera::Camera;
pub use color::{gradient, Color};
pub use config::Config;
pub use fb::IncrementalFrameBuffer;
pub use hittest::{HitRecord, HitTest};
pub use material::{Dielectric, Lambertian, Material, Metal};
pub use ray::Ray;
pub use screen::Screen;
pub use sphere::Sphere;
pub use unit_random::unit_random;
pub use util::random_in_unit_sphere;
pub use vec3::{dot, Vec3};
pub use world::{load_world, World, Worlds};

pub use pg::Progress;

mod camera;
mod color;
mod config;
mod fb;
mod hittest;
mod material;
mod pg;
mod ray;
mod screen;
mod sphere;
mod tracer;
mod unit_random;
mod util;
mod vec3;
mod world;
