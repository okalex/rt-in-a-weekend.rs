use std::f64::{INFINITY as INF64, consts::PI as PI64};

use nalgebra::{Point3, Vector3};
use parry3d_f64::math::Vec3;

pub type Float = f32;
pub type Int = i32;
pub type Uint = u32;
pub type Vector = Vector3<Float>;
pub type Point = Point3<Float>;

pub const PI: Float = PI64 as Float;
pub const INFINITY: Float = INF64 as Float;

pub fn new_vec3(arr: [Float; 3]) -> Vec3 {
    Vec3::new(arr[0] as f64, arr[1] as f64, arr[2] as f64)
}
