use std::sync::Arc;

use nalgebra::Point3;

use crate::rt::color::Color;

use super::solid_color::SolidColor;
use super::texture::Texture;

pub struct Checkered {
    inv_scale: f64,
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
}

impl Checkered {
    pub fn new(scale: f64, even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> Self {
        let inv_scale = 1.0 / scale;
        Self {
            inv_scale,
            even,
            odd,
        }
    }

    pub fn from_colors(scale: f64, even_color: Color, odd_color: Color) -> Self {
        let even: Arc<dyn Texture> = Arc::new(SolidColor::new(even_color));
        let odd: Arc<dyn Texture> = Arc::new(SolidColor::new(odd_color));
        Self::new(scale, even, odd)
    }

    pub fn from_color_values(scale: f64, even_color: [f64; 3], odd_color: [f64; 3]) -> Self {
        Self::from_colors(
            scale,
            Color::from_arr(even_color),
            Color::from_arr(odd_color),
        )
    }
}

impl Texture for Checkered {
    fn value(&self, u: f64, v: f64, point: &Point3<f64>) -> Color {
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

impl Clone for Checkered {
    fn clone(&self) -> Self {
        Self {
            inv_scale: self.inv_scale,
            even: Arc::clone(&self.even),
            odd: Arc::clone(&self.odd),
        }
    }
}
