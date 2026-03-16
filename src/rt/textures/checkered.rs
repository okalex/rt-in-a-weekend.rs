use std::sync::Arc;

use crate::rt::{
    color::Color,
    textures::{solid_color::SolidColor, texture::Texture},
    types::{Float, Int, Point},
};

pub struct Checkered {
    inv_scale: Float,
    even: Arc<Texture>,
    odd: Arc<Texture>,
}

impl Checkered {
    pub fn new(scale: Float, even: Color, odd: Color) -> Self {
        let inv_scale = 1.0 / scale;
        Self {
            inv_scale,
            even: Arc::new(Texture::Solid(SolidColor::new(even))),
            odd: Arc::new(Texture::Solid(SolidColor::new(odd))),
        }
    }

    pub fn from_color_values(scale: Float, even_color: [Float; 3], odd_color: [Float; 3]) -> Self {
        Self::new(
            scale,
            Color::from_arr(even_color),
            Color::from_arr(odd_color),
        )
    }

    pub fn value(&self, u: Float, v: Float, point: &Point) -> Color {
        let x_int = (self.inv_scale * point.x).floor() as Int;
        let y_int = (self.inv_scale * point.y).floor() as Int;
        let z_int = (self.inv_scale * point.z).floor() as Int;
        let is_even = (x_int + y_int + z_int) % 2 == 0;
        if is_even {
            self.even.value(u, v, point)
        } else {
            self.odd.value(u, v, point)
        }
    }
}
