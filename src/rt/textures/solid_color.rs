use nalgebra::Point3;

use crate::rt::color::Color;

use super::texture::Texture;

pub struct SolidColor {
    albedo: Color,
}

impl SolidColor {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }

    #[allow(dead_code)]
    pub fn from_arr(color: [f64; 3]) -> Self {
        Self::new(Color::from_arr(color))
    }
}

impl Texture for SolidColor {
    #[allow(unused)]
    fn value(&self, u: f64, v: f64, point: &Point3<f64>) -> Color {
        self.albedo
    }
}
