use nalgebra::Point3;

use crate::rt::color::Color;
use crate::rt::image::Image;
use crate::rt::interval::Interval;

use super::texture::Texture;

pub struct ImageMap {
    image: Image,
    scale_factor: f64,
}

impl ImageMap {
    pub fn new(filename: &str, scale_factor: f64) -> Self {
        let image = Image::load(filename);
        Self {
            image,
            scale_factor,
        }
    }
}

impl Texture for ImageMap {
    #[allow(unused)]
    fn value(&self, u: f64, v: f64, point: &Point3<f64>) -> Color {
        if self.image.height <= 0 {
            return Color::new(0.0, 1.0, 1.0);
        }

        let interval = Interval::new(0.0, 1.0);
        let u_clamped = interval.clamp(u);
        let v_clamped = 1.0 - interval.clamp(v);

        let i =
            ((self.scale_factor * u_clamped * (self.image.width as f64)) as u32) % self.image.width;
        let j = ((self.scale_factor * v_clamped * (self.image.height as f64)) as u32)
            % self.image.height;

        self.image.pixel_data(i, j)
    }
}
