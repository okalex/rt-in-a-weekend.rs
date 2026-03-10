use std::sync::Arc;

use nalgebra::Point3;

use crate::rt::color::Color;
use crate::rt::textures::solid_color::SolidColor;
use crate::rt::textures::texture::Texture;

use super::material::Material;

pub struct DiffuseLight {
    texture: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(texture: Arc<dyn Texture>) -> Self {
        Self { texture }
    }

    pub fn from_color(color: Color) -> Self {
        let arc: Arc<dyn Texture> = Arc::new(SolidColor::new(color));
        Self::new(Arc::clone(&arc))
    }
}

impl Material for DiffuseLight {
    fn emitted(&self, u: f64, v: f64, point: &Point3<f64>) -> Color {
        self.texture.value(u, v, point)
    }
}

impl Clone for DiffuseLight {
    fn clone(&self) -> Self {
        Self {
            texture: Arc::clone(&self.texture),
        }
    }
}
