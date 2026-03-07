use crate::lib::color::Color;
use crate::lib::perlin::Perlin;
use crate::lib::vec3::Vec3;

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
    fn value(&self, u: f64, v: f64, point: &Vec3) -> Color {
        let scaled = *point * self.scale;
        // let noise = 0.5 * (1.0 + self.noise.noise(&point.scale(self.scale))); // Perlin noise
        let noise = self.noise.turb(&scaled, 7); // Turbulent noise
        // let noise = 0.5 * (1.0 + (self.scale * point.z() + 10.0 * self.noise.turb(point, 7)).sin());
        Color::wrap_vec(Vec3::ones() * noise)
    }
}
