use crate::errors::*;

pub fn range_check(val: f32, min: f32, max: f32) -> Result<()> {
    if val < min || val > max {
        Err(ErrorKind::OutOfRange(val, min, max).into())
    } else {
        Ok(())
    }
}
