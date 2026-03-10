use std::f64::consts::PI;

use nalgebra::Vector3;
use parry3d_f64::math::Vec3;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    return degrees * PI / 180.0;
}

pub fn to_parry_vec(vec: Vector3<f64>) -> Vec3 {
    parry3d_f64::math::Vec3::new(vec.x, vec.y, vec.z)
}
