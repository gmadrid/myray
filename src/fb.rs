use crate::color::Color;
use crate::errors::*;

pub struct FrameBuffer {
    buffer: Vec<u32>,
    width: usize,
    height: usize,
}

impl FrameBuffer {
    pub fn new(width: usize, height: usize) -> Result<Self> {
        let buffer = vec![0; width * height];
        Ok(FrameBuffer { buffer, width, height })
    }

    pub fn height(&self) -> usize { self.height }
    pub fn width(&self) -> usize { self.width }
    pub fn buffer(&self) -> &Vec<u32> { &self.buffer }

    pub fn set(&mut self, x: usize, y: usize, color: Color) {
        self.buffer[(self.height - y - 1) * self.width + x] = color.into();
    }
}

