use std::f32;

use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy)]
pub struct Vec3(f32, f32, f32);

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3(x, y, z)
    }

    pub fn x(&self) -> f32 {
        self.0
    }

    pub fn y(&self) -> f32 {
        self.1
    }

    pub fn z(&self) -> f32 {
        self.2
    }

    pub fn length(&self) -> f32 {
        f32::sqrt(self.0 * self.0 + self.1 * self.1 + self.2 * self.2)
    }

    pub fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }
}

pub fn dot(a: &Vec3, b: &Vec3) -> f32 {
    a.0 * b.0 + a.1 * b.1 + a.2 * b.2
}

impl Add<Vec3> for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Vec3::new(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl<'a> Sub<&'a Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Self::Output {
        Vec3::new(v.0 * self, v.1 * self, v.2 * self)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, v: f32) -> Self::Output {
        v * self
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Self::Output {
        Vec3::new(self.0 * v.0, self.1 * v.1, self.2 * v.2)
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, d: f32) -> Self::Output {
        Vec3::new(self.0 / d, self.1 / d, self.2 / d)
    }
}
