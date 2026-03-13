use std::sync::Arc;

use nalgebra::Point3;

use crate::rt::color::Color;
use crate::rt::objects::hittable::HitRecord;
use crate::rt::ray::Ray;
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
    #[allow(unused)]
    fn emitted(&self, r_in: &Ray, hit_record: &HitRecord) -> Color {
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
