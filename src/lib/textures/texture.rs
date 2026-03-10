use nalgebra::Point3;

use crate::lib::color::Color;

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, point: &Point3<f64>) -> Color;
}
