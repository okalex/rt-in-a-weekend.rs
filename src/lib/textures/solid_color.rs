use crate::lib::color::Color;
use crate::lib::vec3::Vec3;

use super::texture::Texture;

pub struct SolidColor {
    albedo: Color,
}

impl SolidColor {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }

    pub fn from_arr(color: [f64; 3]) -> Self {
        Self::new(Color::from_arr(color))
    }
}

impl Texture for SolidColor {
    fn value(&self, u: f64, v: f64, point: &Vec3) -> Color {
        self.albedo
    }
}
