#[macro_use]
extern crate error_chain;

use minifb::{Key, WindowOptions, Window};

pub mod errors {
    error_chain! {
        errors {
        }
        foreign_links {
            MiniFBError(minifb::Error);
        }
    }

}

use errors::*;

const WIDTH: usize = 240;
const HEIGHT: usize = 100;

struct FrameBuffer {
    buffer: Vec<u32>,
    width: usize,
    height: usize,
}

impl FrameBuffer {
    fn new(width: usize, height: usize) -> Result<Self> {
        let buffer = vec![0; width * height];
        Ok(FrameBuffer { buffer, width, height })
    }

    fn height(&self) -> usize { self.height }
    fn width(&self) -> usize { self.width }
    fn buffer(&self) -> &Vec<u32> { &self.buffer }

    fn set(&self, x: usize, y: usize, value: u32) {
    }
}

struct Screen {
    fb: FrameBuffer,
    window: Window,
}

impl Screen {
    fn new(width: usize, height: usize) -> Result<Self> {
        let fb = FrameBuffer::new(width, height)?;
        let window = Window::new("Test - ESC to exit",
                            width,
                            height,
                            WindowOptions::default())?;
        Ok(Screen { fb, window })
    }

    fn run<F>(&mut self, f: F) where F: Fn(&FrameBuffer) -> () {
        f(&self.fb);
        self.window.update_with_buffer(&self.fb.buffer()).unwrap();
    }
}

fn main() {
    let screen = Screen::new(WIDTH, HEIGHT).unwrap();

    screen.run(|fb| {
        let height = fb.height() as f32;
        let width = fb.width() as f32;
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let r = x as f32 / width;
                let g = y as f32 / height;
                let b = 0.2;

                let value = (((255.99 * r) as u32) << 16) +
                    (((255.99 * g) as u32) << 8) +
                    ((255.99 * b) as u32);
                fb.set(x, y, value);  // TODO: make this a color.
            }
        }
    });

    
    
/*
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new("Test - ESC to exit",
                                 WIDTH,
                                 HEIGHT,
                                 WindowOptions::default()).unwrap();
    
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let height = HEIGHT as f32;
        let width = WIDTH as f32;
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let r = x as f32 / width;
                let g = y as f32 / height;
                let b = 0.2;

                buffer[(y * WIDTH + x)] =
                    (((255.99 * r) as u32) << 16) +
                    (((255.99 * g) as u32) << 8) +
                    ((255.99 * b) as u32);
            }
        }
        window.update_with_buffer(&buffer).unwrap();
    }
*/
}
