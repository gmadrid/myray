#[macro_use]
extern crate error_chain;

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
            MiniFBError(minifb::Error);
            ParseIntError(std::num::ParseIntError);
        }
    }

}

pub use camera::Camera;
pub use color::Color;
pub use hittest::{HitRecord, HitTest};
pub use ray::Ray;
pub use screen::Screen;
pub use sphere::Sphere;
pub use vec3::{dot, Vec3};

mod camera;
mod color;
mod fb;
mod hittest;
mod ray;
mod screen;
mod sphere;
mod util;
mod vec3;
