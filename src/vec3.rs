use std::f32;

use std::ops::Add;
use std::ops::Mul;
use std::ops::Div;

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

impl Add<Vec3> for Vec3 {
    type Output = Self;
    
    fn add(self, rhs: Self) -> Self {
        Vec3::new(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;
        
    fn mul(self, v: Vec3) -> Self::Output {
        Vec3::new(v.0 * self, v.1 * self, v.2 * self)
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, d: f32) -> Self::Output {
        Vec3::new(self.0 / d, self.1 / d, self.2 / d)
    }
}
