#[macro_use]
extern crate error_chain;

pub mod errors {
    error_chain! {
        errors {
        }
        foreign_links {
            MiniFBError(minifb::Error);
        }
    }

}

pub use screen::Screen;

mod fb;
mod screen;

