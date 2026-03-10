use nalgebra::Point3;

use crate::lib::color::Color;
use crate::lib::image::Image;
use crate::lib::interval::Interval;

use super::texture::Texture;

pub struct ImageMap {
    image: Image,
}

impl ImageMap {
    pub fn new(filename: &str) -> Self {
        let image = Image::load(filename);
        Self { image }
    }
}

impl Texture for ImageMap {
    fn value(&self, u: f64, v: f64, point: &Point3<f64>) -> Color {
        if self.image.height <= 0 {
            return Color::new(0.0, 1.0, 1.0);
        }

        let interval = Interval::new(0.0, 1.0);
        let u_clamped = interval.clamp(u);
        let v_clamped = 1.0 - interval.clamp(v);

        let i = (u_clamped * (self.image.width as f64)) as u32;
        let j = (v_clamped * (self.image.height as f64)) as u32;

        self.image.pixel_data(i, j)
    }
}
