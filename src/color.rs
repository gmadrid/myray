use crate::errors::*;
use crate::vec3::Vec3;

pub struct Color {
    // All values will be stored as 0-1.0
    r: f32,
    g: f32,
    b: f32,
}

fn range_check(val: f32, min: f32, max:f32) -> Result<()> {
    if val < min || val > max {
        Err(ErrorKind::OutOfRange(val, min, max).into())
    } else {
        Ok(())
    }
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Result<Self> {
        range_check(r, 0.0, 1.0)?;
        range_check(g, 0.0, 1.0)?;
        range_check(b, 0.0, 1.0)?;

        Ok(Color { r, g, b })
    }
}

impl From<Color> for u32 {
    fn from(color: Color) -> Self {
        (((255.99 * color.r) as u32) << 16) +
            (((255.99 * color.g) as u32) << 8) +
            ((255.99 * color.b) as u32)
    }
}

impl From<Vec3> for Color {
    fn from(v: Vec3) -> Self {
        Color::new(v.x(), v.y(), v.z()).unwrap()
    }
}
