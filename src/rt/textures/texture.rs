use crate::{
    rt::textures::{checkered::Checkered, image_map::ImageMap, perlin_noise::PerlinNoise, solid_color::SolidColor},
    util::{
        color::Color,
        types::{Float, Point},
    },
};

pub enum Texture {
    Checkered(Checkered),
    ImageMap(ImageMap),
    Perlin(PerlinNoise),
    Solid(SolidColor),
}

impl Texture {
    pub fn value(&self, u: Float, v: Float, point: &Point) -> Color {
        match self {
            Self::Checkered(tex) => tex.value(u, v, point),
            Self::ImageMap(tex) => tex.value(u, v, point),
            Self::Perlin(tex) => tex.value(u, v, point),
            Self::Solid(tex) => tex.value(u, v, point),
        }
    }
}
