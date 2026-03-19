use std::sync::Arc;

use crate::rt::color::Color;
use crate::rt::objects::hit_record::HitRecord;
use crate::rt::ray::Ray;
use crate::rt::textures::solid_color::SolidColor;
use crate::rt::textures::texture::Texture;
use crate::rt::types::Float;

pub struct Emissive {
    pub texture: Arc<Texture>,
}

impl Emissive {
    pub fn new(texture: Arc<Texture>) -> Self {
        Self { texture }
    }

    #[allow(unused)]
    pub fn emitted(&self, r_in: &Ray, hit_record: &HitRecord) -> Color {
        if hit_record.front_face {
            self.texture
                .value(hit_record.u, hit_record.v, &hit_record.point)
        } else {
            Color::black()
        }
    }
}

impl From<Color> for Emissive {
    fn from(color: Color) -> Self {
        let arc = Arc::new(Texture::Solid(SolidColor::new(color)));
        Self::new(arc)
    }
}

impl From<[Float; 3]> for Emissive {
    fn from(color: [Float; 3]) -> Self {
        let arc = Arc::new(Texture::Solid(SolidColor::from(color)));
        Self::new(arc)
    }
}

impl Clone for Emissive {
    fn clone(&self) -> Self {
        Self {
            texture: Arc::clone(&self.texture),
        }
    }
}
