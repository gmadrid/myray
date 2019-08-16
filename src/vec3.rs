use std::f32;
use std::ops;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::errors::*;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Vec3(f32, f32, f32);

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3(x, y, z)
    }

    pub fn origin() -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
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

pub fn cross(lhs: &Vec3, rhs: &Vec3) -> Vec3 {
    Vec3::new(
        lhs.1 * rhs.2 - lhs.2 * rhs.1,
        -(lhs.0 * rhs.2 - lhs.2 * rhs.0),
        lhs.0 * rhs.1 - lhs.1 * rhs.0,
    )
}

impl FromStr for Vec3 {
    type Err = Error;

    fn from_str(s: &str) -> Result<Vec3> {
        let trimmed = s.trim();
        if !trimmed.starts_with('(') {
            return Err(
                ErrorKind::ParseError(s.to_string(), "Must start with '('".to_string()).into(),
            );
        }

        if !trimmed.ends_with(')') {
            return Err(
                ErrorKind::ParseError(s.to_string(), "Must end with ')'".to_string()).into(),
            );
        }

        let no_parens = &s[1..(s.len() - 1)];
        let pieces = no_parens.split(',').collect::<Vec<&str>>();

        if pieces.len() != 3 {
            return Err(ErrorKind::ParseError(
                s.to_string(),
                "Must have 3 components.".to_string(),
            )
            .into());
        }

        let x = f32::from_str(pieces[0].trim())?;
        let y = f32::from_str(pieces[1].trim())?;
        let z = f32::from_str(pieces[2].trim())?;

        Ok(Vec3::new(x, y, z))
    }
}

impl_op_ex!(+ |lhs: &Vec3, rhs: &Vec3| -> Vec3 {
    Vec3::new(lhs.0 + rhs.0, lhs.1 + rhs.1, lhs.2 + rhs.2)
});

impl_op_ex!(-|val: &Vec3| -> Vec3 { Vec3::new(-val.0, -val.1, -val.2) });

impl_op_ex!(-|lhs: &Vec3, rhs: &Vec3| -> Vec3 {
    Vec3::new(lhs.0 - rhs.0, lhs.1 - rhs.1, lhs.2 - rhs.2)
});

impl_op_ex_commutative!(*|lhs: f32, rhs: &Vec3| -> Vec3 {
    Vec3::new(lhs * rhs.0, lhs * rhs.1, lhs * rhs.2)
});

impl_op_ex!(*|lhs: &Vec3, rhs: &Vec3| -> Vec3 {
    Vec3::new(lhs.0 * rhs.0, lhs.1 * rhs.1, lhs.2 * rhs.2)
});

impl_op_ex!(/ |lhs: &Vec3, rhs: f32| -> Vec3 {
    Vec3::new(lhs.0 / rhs, lhs.1 / rhs, lhs.2 / rhs)
});

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_str() {
        assert_eq!(
            Vec3::new(4.3, -1.0, 0.0),
            Vec3::from_str("(4.3, -1, 0)").unwrap()
        );
    }

}
