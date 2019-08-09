use std::f32;

use std::ops;

#[derive(Clone, Copy, Debug)]
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

    pub fn squared_length(&self) -> f32 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn length(&self) -> f32 {
        f32::sqrt(self.squared_length())
    }

    pub fn unit_vector(&self) -> Vec3 {
        self / self.length()
    }
}

pub fn dot(a: &Vec3, b: &Vec3) -> f32 {
    a.0 * b.0 + a.1 * b.1 + a.2 * b.2
}

impl_op_ex!(+ |lhs: &Vec3, rhs: &Vec3| -> Vec3 {
    Vec3::new(lhs.0 + rhs.0, lhs.1 + rhs.1, lhs.2 + rhs.2)
});

impl_op_ex!(- |lhs: &Vec3, rhs: &Vec3| -> Vec3 {
    Vec3::new(lhs.0 - rhs.0, lhs.1 - rhs.1, lhs.2 - rhs.2)
});

impl_op_ex_commutative!(* |lhs: f32, rhs: &Vec3| -> Vec3 {
    Vec3::new(lhs * rhs.0, lhs * rhs.1, lhs * rhs.2)
});

impl_op_ex!(* |lhs: &Vec3, rhs: &Vec3| -> Vec3 {
    Vec3::new(lhs.0 * rhs.0, lhs.1 * rhs.1, lhs.2 * rhs.2)
});

impl_op_ex!(/ |lhs: &Vec3, rhs: f32| -> Vec3 {
    Vec3::new(lhs.0 / rhs, lhs.1 / rhs, lhs.2 / rhs)
});
