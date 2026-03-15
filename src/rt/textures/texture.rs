use nalgebra::Point3;

use crate::rt::{
    color::Color,
    textures::{
        checkered::Checkered, image_map::ImageMap, perlin_noise::PerlinNoise,
        solid_color::SolidColor,
    },
};

pub enum Texture {
    Checkered(Checkered),
    ImageMap(ImageMap),
    Perlin(PerlinNoise),
    Solid(SolidColor),
}

impl Texture {
    pub fn value(&self, u: f64, v: f64, point: &Point3<f64>) -> Color {
        match self {
            Self::Checkered(tex) => tex.value(u, v, point),
            Self::ImageMap(tex) => tex.value(u, v, point),
            Self::Perlin(tex) => tex.value(u, v, point),
            Self::Solid(tex) => tex.value(u, v, point),
        }
    }
}
