use nalgebra::Point3;

use crate::rt::color::Color;

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

    #[allow(unused_variables)]
    pub fn value(&self, u: f64, v: f64, point: &Point3<f64>) -> Color {
        self.albedo
    }
}
