#[macro_use]
extern crate error_chain;

pub mod errors {
    error_chain! {
        errors {
            OutOfRange(val: f32, min: f32, max: f32) {
                description("Value out of range.")
                    display("Value, {}, out of range: [{}, {}]", val, min, max)
            }
        }
        foreign_links {
            MiniFBError(minifb::Error);
        }
    }

}

pub use color::Color;
pub use ray::Ray;
pub use screen::Screen;
pub use vec3::{dot, Vec3};

mod color;
mod fb;
mod ray;
mod screen;
mod vec3;
