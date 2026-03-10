use std::sync::Arc;

use crate::rt::objects::hittable::HitRecord;
use crate::rt::random::rand_unit_vector;
use crate::rt::ray::Ray;
use crate::rt::textures::solid_color::SolidColor;
use crate::rt::{color::Color, textures::texture::Texture};

use super::material::{Material, Scattered};

pub struct Isotropic {
    texture: Arc<dyn Texture>,
}

impl Isotropic {
    pub fn new(texture: Arc<dyn Texture>) -> Self {
        Self {
            texture: Arc::clone(&texture),
        }
    }

    #[allow(dead_code)]
    pub fn from_color(albedo: Color) -> Self {
        let texture: Arc<dyn Texture> = Arc::new(SolidColor::new(albedo));
        Self::new(texture)
    }
}

impl Material for Isotropic {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scattered> {
        let scattered = Ray::new(rec.point, rand_unit_vector(), r_in.time);
        let attenuation = self.texture.value(rec.u, rec.v, &rec.point);
        Some(Scattered {
            ray: scattered,
            attenuation,
        })
    }
}
