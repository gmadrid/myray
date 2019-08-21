use std::f32;
use std::ops;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::errors::*;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Vec3(f32, f32, f32); // x, y, z in cartesian coordinates

impl Vec3 {
    pub fn cartesian(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3(x, y, z)
    }

    pub fn cylindrical(r: f32, t: f32, z: f32) -> Vec3 {
        let t_rad = t * f32::consts::PI / 180.0;
        Vec3(r * f32::cos(t_rad), r * f32::sin(t_rad), z)
    }

    pub fn spherical(r: f32, t: f32, p: f32) -> Vec3 {
        let t_rad = t * f32::consts::PI / 180.0;
        let p_rad = p * f32::consts::PI / 180.0;
        Vec3(
            r * f32::sin(t_rad) * f32::cos(p_rad),
            r * f32::sin(t_rad) * f32::sin(p_rad),
            r * f32::cos(t_rad),
        )
    }

    pub fn origin() -> Vec3 {
        Vec3::cartesian(0.0, 0.0, 0.0)
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

    pub fn unit_vector(&self) -> Result<Vec3> {
        let length = self.length();
        if length == 0.0 {
            Err(ErrorKind::InvalidParam(
                0.0,
                "Cannot take unit_vector of a zero-length vector.".to_string(),
            )
            .into())
        } else {
            Ok(self / self.length())
        }
    }
}

pub fn dot(a: &Vec3, b: &Vec3) -> f32 {
    a.0 * b.0 + a.1 * b.1 + a.2 * b.2
}

pub fn cross(lhs: &Vec3, rhs: &Vec3) -> Vec3 {
    Vec3::cartesian(
        lhs.1 * rhs.2 - lhs.2 * rhs.1,
        -(lhs.0 * rhs.2 - lhs.2 * rhs.0),
        lhs.0 * rhs.1 - lhs.1 * rhs.0,
    )
}

fn parse_cylindrical(s: &str) -> Result<Vec3> {
    let (r, t, z) = split_three(s)?;
    Ok(Vec3::cylindrical(r, t, z))
}

fn parse_spherical(s: &str) -> Result<Vec3> {
    let (r, t, p) = split_three(s)?;
    Ok(Vec3::spherical(r, t, p))
}

fn parse_cartesian(s: &str) -> Result<Vec3> {
    let (x, y, z) = split_three(s)?;
    Ok(Vec3::cartesian(x, y, z))
}

fn split_three(s: &str) -> Result<(f32, f32, f32)> {
    let pieces = s.split(',').collect::<Vec<&str>>();
    if pieces.len() != 3 {
        return Err(
            ErrorKind::ParseError(s.to_string(), "Must have 3 components.".to_string()).into(),
        );
    }

    Ok((
        f32::from_str(pieces[0].trim())?,
        f32::from_str(pieces[1].trim())?,
        f32::from_str(pieces[2].trim())?,
    ))
}

impl FromStr for Vec3 {
    type Err = Error;

    fn from_str(s: &str) -> Result<Vec3> {
        //  3,  4,    5   cartesian
        // r5, 45.0,  3   cylindrical
        // s3, 45.0, 30.0 spherical
        let trimmed = s.trim();

        if trimmed.starts_with('r') {
            parse_cylindrical(&s[1..])
        } else if trimmed.starts_with('s') {
            parse_spherical(&s[1..])
        } else {
            parse_cartesian(s)
        }
    }
}

impl_op_ex!(+ |lhs: &Vec3, rhs: &Vec3| -> Vec3 {
    Vec3::cartesian(lhs.0 + rhs.0, lhs.1 + rhs.1, lhs.2 + rhs.2)
});

impl_op_ex!(-|val: &Vec3| -> Vec3 { Vec3::cartesian(-val.0, -val.1, -val.2) });

impl_op_ex!(-|lhs: &Vec3, rhs: &Vec3| -> Vec3 {
    Vec3::cartesian(lhs.0 - rhs.0, lhs.1 - rhs.1, lhs.2 - rhs.2)
});

impl_op_ex_commutative!(*|lhs: f32, rhs: &Vec3| -> Vec3 {
    Vec3::cartesian(lhs * rhs.0, lhs * rhs.1, lhs * rhs.2)
});

impl_op_ex!(*|lhs: &Vec3, rhs: &Vec3| -> Vec3 {
    Vec3::cartesian(lhs.0 * rhs.0, lhs.1 * rhs.1, lhs.2 * rhs.2)
});

impl_op_ex!(/ |lhs: &Vec3, rhs: f32| -> Vec3 {
    Vec3::cartesian(lhs.0 / rhs, lhs.1 / rhs, lhs.2 / rhs)
});

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_str() {
        assert_eq!(
            Vec3::cartesian(4.3, -1.0, 0.0),
            Vec3::from_str("4.3, -1, 0").unwrap()
        );
    }

}
