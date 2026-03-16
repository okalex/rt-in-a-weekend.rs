use crate::rt::{
    color::Color,
    types::{Float, Point},
};

pub struct SolidColor {
    albedo: Color,
}

impl SolidColor {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }

    #[allow(dead_code)]
    pub fn from_arr(color: [Float; 3]) -> Self {
        Self::new(Color::from_arr(color))
    }

    #[allow(unused_variables)]
    pub fn value(&self, u: Float, v: Float, point: &Point) -> Color {
        self.albedo
    }
}
