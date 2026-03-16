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

    #[allow(unused_variables)]
    pub fn value(&self, u: Float, v: Float, point: &Point) -> Color {
        self.albedo
    }
}

impl From<[Float; 3]> for SolidColor {
    fn from(color: [Float; 3]) -> Self {
        Self::new(Color::from(color))
    }
}
