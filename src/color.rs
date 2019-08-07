use crate::errors::*;
use crate::util::range_check;
use crate::vec3::Vec3;

pub struct Color {
    // All values will be stored as 0-1.0
    r: f32,
    g: f32,
    b: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Result<Self> {
        range_check(r, 0.0, 1.0)?;
        range_check(g, 0.0, 1.0)?;
        range_check(b, 0.0, 1.0)?;

        Ok(Color { r, g, b })
    }

    pub fn from_hex(s: &str) -> Result<Self> {
        let digits = if s.starts_with('#') { &s[1..] } else { s };

        if digits.chars().any(|ch| !ch.is_digit(16)) {
            return Err(ErrorKind::ParseError(
                s.to_string(),
                "Must contain only hex digits, 0-f".to_string(),
            )
            .into());
        }

        if digits.len() != 6 {
            return Err(
                ErrorKind::ParseError(s.to_string(), "Must have 6 digits.".to_string()).into(),
            );
        }

        // We can split_at since all chars should be ASCII 0-9 or a-f or A-F.
        let r = u8::from_str_radix(&digits[0..2], 16)?;
        let g = u8::from_str_radix(&digits[2..4], 16)?;
        let b = u8::from_str_radix(&digits[4..6], 16)?;

        Color::from_rgb(r, g, b)
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Result<Self> {
        Color::new((r as f32) / 255.0, (g as f32) / 255.0, (b as f32) / 255.0)
    }

    // h: [0-360]
    // s: [0-1]
    // v: [0-1]
    pub fn from_hsv(h: f32, s: f32, v: f32) -> Result<Self> {
        range_check(h, 0.0, 360.0)?;
        range_check(s, 0.0, 1.0)?;
        range_check(v, 0.0, 1.0)?;

        let f = |n| {
            let k = (((n as f32) + h / 60.0) as i32) % 6;
            let min_of_three = (i32::min(k, i32::min(4 - k, 1))) as f32;
            v - v * s * f32::max(min_of_three, 0.0)
        };

        Color::new(f(5), f(3), f(1))
    }

    pub fn white() -> Self {
        Color::new(1.0, 1.0, 1.0).unwrap()
    }

    pub fn black() -> Self {
        Color::new(0.0, 0.0, 0.0).unwrap()
    }

    pub fn as_vec(&self) -> Vec3 {
        Vec3::new(self.r, self.g, self.b)
    }
}

pub fn gradient(t: f32, l_color: &Color, r_color: &Color) -> Color {
    range_check(t, 0.0, 1.0).unwrap();

    let l_vec = (1.0 - t) * l_color.as_vec();
    let r_vec = t * r_color.as_vec();
    (l_vec + r_vec).into()
}

impl From<Color> for u32 {
    fn from(color: Color) -> Self {
        (((255.99 * color.r) as u32) << 16)
            + (((255.99 * color.g) as u32) << 8)
            + ((255.99 * color.b) as u32)
    }
}

impl From<Vec3> for Color {
    fn from(v: Vec3) -> Self {
        Color::new(v.x(), v.y(), v.z()).unwrap()
    }
}
