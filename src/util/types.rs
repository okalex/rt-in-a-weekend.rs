use std::f64::{
    consts::PI as PI64,
    INFINITY as INF64,
};

pub type Float = f32;
pub type Int = i32;
pub type Uint = u32;
pub type Vector = glam::Vec3;
pub type Point = glam::Vec3;

pub const PI: Float = PI64 as Float;
pub const INFINITY: Float = INF64 as Float;
