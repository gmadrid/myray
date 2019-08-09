#[macro_use]
extern crate clap;

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate impl_ops;

#[macro_use]
extern crate lazy_static;

pub mod errors {
    error_chain! {
        errors {
            InvalidParam(val: f32, t: String) {
               description("Value invalid")
               display("Value invalid ({}): {}", t, val)
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
            ClapError(clap::Error);
            MiniFBError(minifb::Error);
            ParseIntError(std::num::ParseIntError);
        }
    }

}

pub use args::Config;
pub use camera::Camera;
pub use color::{gradient, Color};
pub use hittest::{HitRecord, HitTest};
pub use material::{Lambertian, Material, Metal};
pub use ray::Ray;
pub use screen::Screen;
pub use sphere::Sphere;
pub use unit_random::unit_random;
pub use util::random_in_unit_sphere;
pub use vec3::{dot, Vec3};

mod args;
mod camera;
mod color;
mod fb;
mod hittest;
mod material;
mod ray;
mod screen;
mod sphere;
mod unit_random;
mod util;
mod vec3;
