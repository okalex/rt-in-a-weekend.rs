use nalgebra::{Point3, Vector3};

use crate::rt::color::Color;
use crate::rt::perlin::Perlin;

use super::texture::Texture;

pub struct Noise {
    scale: f64,
    noise: Perlin,
}

impl Noise {
    pub fn new(scale: f64) -> Self {
        Self {
            scale,
            noise: Perlin::new(),
        }
    }
}

impl Texture for Noise {
    #[allow(unused)]
    fn value(&self, u: f64, v: f64, point: &Point3<f64>) -> Color {
        let scaled = *point * self.scale;
        // let noise = 0.5 * (1.0 + self.noise.noise(&point.scale(self.scale))); // Perlin noise
        let noise = self.noise.turb(&scaled, 7); // Turbulent noise
        // let noise = 0.5 * (1.0 + (self.scale * point.z() + 10.0 * self.noise.turb(point, 7)).sin());
        Color::wrap_vec(Vector3::from_element(1.0) * noise)
    }
}
