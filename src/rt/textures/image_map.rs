use std::sync::Arc;

use crate::util::{
    color::Color,
    image::Image,
    interval::Interval,
    types::{
        Float,
        Point,
        Uint,
    },
};

pub struct ImageMap {
    image: Arc<Image>,
    scale_factor: Float,
}

impl ImageMap {
    pub fn new(filename: &str, scale_factor: Float) -> Self {
        let image = Arc::new(Image::load(filename));
        Self { image, scale_factor }
    }

    #[allow(unused_variables)]
    pub fn value(&self, u: Float, v: Float, point: &Point) -> Color {
        if self.image.height <= 0 {
            return Color::new(0.0, 1.0, 1.0);
        }

        let interval = Interval::new(0.0, 1.0);
        let u_clamped = interval.clamp(u);
        let v_clamped = 1.0 - interval.clamp(v);

        let i = ((self.scale_factor * u_clamped * (self.image.width as Float)) as Uint) % self.image.width;
        let j = ((self.scale_factor * v_clamped * (self.image.height as Float)) as Uint) % self.image.height;

        self.image.pixel_data(i, j)
    }
}
