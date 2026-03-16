use crate::rt::types::{Float, PI};

pub fn degrees_to_radians(degrees: Float) -> Float {
    return degrees * PI / 180.0;
}
