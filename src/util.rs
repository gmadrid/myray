use crate::errors::*;
use crate::unit_random::unit_random;
use crate::vec3::Vec3;

pub fn range_check(val: f32, min: f32, max: f32) -> Result<()> {
    if val < min || val > max {
        Err(ErrorKind::OutOfRange(val, min, max).into())
    } else {
        Ok(())
    }
}

pub fn random_in_unit_sphere() -> Vec3 {
    let offset = Vec3::cartesian(1.0, 1.0, 1.0);
    loop {
        let p = 2.0 * Vec3::cartesian(unit_random(), unit_random(), unit_random()) - offset;
        if p.squared_length() >= 1.0 {
            return p;
        }
    }
}

pub fn if_then<F, T>(cond: bool, f: F) -> Option<T>
where
    F: FnOnce() -> Option<T>,
{
    if cond {
        f()
    } else {
        None
    }
}
