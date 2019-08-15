use crate::color::Color;
use crate::errors::*;

pub struct IncrementalFrameBuffer {
    buffer: Vec<f64>,
    width: usize,
    height: usize,
}

impl IncrementalFrameBuffer {
    pub fn new(width: usize, height: usize) -> Result<Self> {
        let buffer = vec![0.0; height * width * 3];
        Ok(IncrementalFrameBuffer {
            buffer,
            width,
            height,
        })
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn set(&mut self, x: usize, y: usize, color: Color) {
        let start_index = (self.height - y - 1) * 3 * self.width + x * 3;
        self.buffer[start_index] += color.r as f64;
        self.buffer[start_index + 1] += color.g as f64;
        self.buffer[start_index + 2] += color.b as f64;
    }

    pub fn to_fb(&mut self, n: u32, fb: &mut FrameBuffer) {
        let div = n as f64 + 1.0;
        let i = self.buffer.chunks(3).map(|chunk| {
            let r = chunk[0] / div;
            let g = chunk[1] / div;
            let b = chunk[2] / div;
            u32::from(Color::new(r as f32, g as f32, b as f32).unwrap())
        });

        fb.buffer_mut().clear();
        fb.buffer_mut().extend(i);
    }
}

pub struct FrameBuffer {
    buffer: Vec<u32>,
    width: usize,
    height: usize,
}

impl FrameBuffer {
    pub fn new(width: usize, height: usize) -> Result<Self> {
        let buffer = vec![0; width * height];
        Ok(FrameBuffer {
            buffer,
            width,
            height,
        })
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn buffer(&self) -> &Vec<u32> {
        &self.buffer
    }

    pub fn buffer_mut(&mut self) -> &mut Vec<u32> {
        &mut self.buffer
    }

    pub fn set(&mut self, x: usize, y: usize, color: Color) {
        self.buffer[(self.height - y - 1) * self.width + x] = color.into();
    }
}
