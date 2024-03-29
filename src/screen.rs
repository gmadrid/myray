use minifb::{Key, Scale, Window, WindowOptions};

use crate::errors::*;
use crate::fb::FrameBuffer;

pub struct Screen {
    fb: FrameBuffer,
    window: Window,
}

impl Screen {
    pub fn new(width: usize, height: usize, scale: Scale) -> Result<Self> {
        let fb = FrameBuffer::new(width, height)?;
        let window = Window::new(
            "Test - ESC to exit",
            width,
            height,
            WindowOptions {
                scale,
                ..WindowOptions::default()
            },
        )?;
        Ok(Screen { fb, window })
    }

    pub fn height(&self) -> usize {
        self.fb.height()
    }

    pub fn width(&self) -> usize {
        self.fb.width()
    }

    pub fn one_frame<F>(&mut self, mut f: F) -> Result<()>
    where
        F: FnMut(&mut FrameBuffer) -> Result<()>,
    {
        // Workaround the Mac black window by showing a window immediately.
        #[cfg(target_os = "macos")]
        self.window.update_with_buffer(&self.fb.buffer())?;

        f(&mut self.fb)?;
        self.window.update_with_buffer(&self.fb.buffer())?;
        Ok(())
    }

    pub fn wait(&mut self) -> Result<()> {
        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            self.window.update();
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
