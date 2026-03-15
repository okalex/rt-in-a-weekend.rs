use std::sync::Arc;

use crate::rt::color::Color;
use crate::rt::objects::hit_record::HitRecord;
use crate::rt::ray::Ray;
use crate::rt::textures::solid_color::SolidColor;
use crate::rt::textures::texture::Texture;

pub struct DiffuseLight {
    texture: Arc<Texture>,
}

impl DiffuseLight {
    pub fn new(texture: Arc<Texture>) -> Self {
        Self { texture }
    }

    pub fn from_color(color: Color) -> Self {
        let arc = Arc::new(Texture::Solid(SolidColor::new(color)));
        Self::new(arc)
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

impl Clone for DiffuseLight {
    fn clone(&self) -> Self {
        Self {
            texture: Arc::clone(&self.texture),
        }
    }
}
