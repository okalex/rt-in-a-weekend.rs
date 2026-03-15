use std::sync::Arc;

use nalgebra::Point3;

use crate::rt::{
    color::Color,
    textures::{solid_color::SolidColor, texture::Texture},
};

pub struct Checkered {
    inv_scale: f64,
    even: Arc<Texture>,
    odd: Arc<Texture>,
}

impl Checkered {
    pub fn new(scale: f64, even: Color, odd: Color) -> Self {
        let inv_scale = 1.0 / scale;
        Self {
            inv_scale,
            even: Arc::new(Texture::Solid(SolidColor::new(even))),
            odd: Arc::new(Texture::Solid(SolidColor::new(odd))),
        }
    }

    pub fn from_color_values(scale: f64, even_color: [f64; 3], odd_color: [f64; 3]) -> Self {
        Self::new(
            scale,
            Color::from_arr(even_color),
            Color::from_arr(odd_color),
        )
    }

    pub fn value(&self, u: f64, v: f64, point: &Point3<f64>) -> Color {
        let x_int = (self.inv_scale * point.x).floor() as i64;
        let y_int = (self.inv_scale * point.y).floor() as i64;
        let z_int = (self.inv_scale * point.z).floor() as i64;
        let is_even = (x_int + y_int + z_int) % 2 == 0;
        if is_even {
            self.even.value(u, v, point)
        } else {
            self.odd.value(u, v, point)
        }
    }
}
