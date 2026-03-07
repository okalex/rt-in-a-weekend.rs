use std::sync::Arc;

use crate::lib::hittable::HitRecord;
use crate::lib::ray::Ray;
use crate::lib::textures::solid_color::SolidColor;
use crate::lib::vec3::Vec3;
use crate::lib::{color::Color, textures::texture::Texture};

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

    pub fn from_color(albedo: Color) -> Self {
        let texture: Arc<dyn Texture> = Arc::new(SolidColor::new(albedo));
        Self::new(texture)
    }
}

impl Material for Isotropic {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scattered> {
        let scattered = Ray::new(rec.point, Vec3::rand_unit(), r_in.time);
        let attenuation = self.texture.value(rec.u, rec.v, &rec.point);
        Some(Scattered {
            ray: scattered,
            attenuation,
        })
    }
}
