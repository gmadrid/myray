use minifb::{Key, Scale, Window, WindowOptions};

use crate::errors::*;
use crate::fb::FrameBuffer;

pub struct Screen {
    fb: FrameBuffer,
    window: Window,
}

impl Screen {
    pub fn new(width: usize, height: usize) -> Result<Self> {
        let fb = FrameBuffer::new(width, height)?;
        let window = Window::new(
            "Test - ESC to exit",
            width,
            height,
            WindowOptions {
                //                scale: Scale::X4,
                ..WindowOptions::default()
            },
        )?;
        Ok(Screen { fb, window })
    }

    pub fn one_frame<F>(&mut self, mut f: F) -> Result<()>
    where
        F: FnMut(&mut FrameBuffer) -> Result<()>,
    {
        f(&mut self.fb)?;
        self.window.update_with_buffer(&self.fb.buffer())?;
        Ok(())
    }

    pub fn wait(&self) -> Result<()> {
        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            // Find some better way to do this without hard-looping.
        }
        Ok(())
    }

    pub fn run<F>(&mut self, mut f: F) -> Result<()>
    where
        F: FnMut(&mut FrameBuffer) -> Result<()>,
    {
        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            f(&mut self.fb)?;
            self.window.update_with_buffer(&self.fb.buffer()).unwrap();
        }
        Ok(())
    }
}
