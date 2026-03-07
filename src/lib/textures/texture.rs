use crate::lib::color::Color;
use crate::lib::vec3::Vec3;

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, point: &Vec3) -> Color;
}
