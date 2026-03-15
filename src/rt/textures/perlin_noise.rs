use nalgebra::{Point3, Vector3};

use crate::rt::color::Color;
use crate::rt::perlin::Perlin;

pub struct PerlinNoise {
    scale: f64,
    noise: Perlin,
}

impl PerlinNoise {
    pub fn new(scale: f64) -> Self {
        Self {
            scale,
            noise: Perlin::new(),
        }
    }

    #[allow(unused_variables)]
    pub fn value(&self, u: f64, v: f64, point: &Point3<f64>) -> Color {
        let scaled = *point * self.scale;
        // let noise = 0.5 * (1.0 + self.noise.noise(&point.scale(self.scale))); // Perlin noise
        let noise = self.noise.turb(&scaled, 7); // Turbulent noise
        // let noise = 0.5 * (1.0 + (self.scale * point.z() + 10.0 * self.noise.turb(point, 7)).sin());
        Color::wrap_vec(Vector3::from_element(1.0) * noise)
    }
}
