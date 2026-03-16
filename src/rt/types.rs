use std::f64::{INFINITY as INF64, consts::PI as PI64};

use glam::Vec3;

pub type Float = f32;
pub type Int = i32;
pub type Uint = u32;
pub type Vector = Vec3;
pub type Point = Vec3;

pub const PI: Float = PI64 as Float;
pub const INFINITY: Float = INF64 as Float;

pub fn new_parry_vec(arr: [Float; 3]) -> parry3d_f64::math::Vec3 {
    parry3d_f64::math::Vec3::new(arr[0] as f64, arr[1] as f64, arr[2] as f64)
}

pub fn to_parry_vec(vec: Vector) -> parry3d_f64::math::Vec3 {
    new_parry_vec([vec.x, vec.y, vec.z])
}

pub fn from_parry_vec(vec: parry3d_f64::math::Vec3) -> Vector {
    Vector::new(vec.x as Float, vec.y as Float, vec.z as Float)
}
