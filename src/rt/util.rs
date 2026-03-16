use parry3d_f64::math::Vec3;

use crate::rt::types::{Float, PI, Vector, new_vec3};

pub fn degrees_to_radians(degrees: Float) -> Float {
    return degrees * PI / 180.0;
}

pub fn to_parry_vec(vec: Vector) -> Vec3 {
    new_vec3([vec.x, vec.y, vec.z])
}

pub fn from_parry_vec(vec: Vec3) -> Vector {
    Vector::new(vec.x as Float, vec.y as Float, vec.z as Float)
}
