use crate::rt::color::Color;
use crate::rt::perlin::Perlin;
use crate::rt::types::{Float, Point, Vector};

pub struct PerlinNoise {
    scale: Float,
    noise: Perlin,
}

impl PerlinNoise {
    pub fn new(scale: Float) -> Self {
        Self {
            scale,
            noise: Perlin::new(),
        }
    }

    #[allow(unused_variables)]
    pub fn value(&self, u: Float, v: Float, point: &Point) -> Color {
        let scaled = *point * self.scale;
        // let noise = 0.5 * (1.0 + self.noise.noise(&point.scale(self.scale))); // Perlin noise
        let noise = self.noise.turb(&scaled, 7); // Turbulent noise
        // let noise = 0.5 * (1.0 + (self.scale * point.z() + 10.0 * self.noise.turb(point, 7)).sin());
        Color::wrap_vec(Vector::from_element(1.0) * noise)
    }
}
