use std::sync::Arc;

use super::material::ScatterRecord;
use crate::{
    rt::{
        geometry::hit_record::HitRecord,
        ray::Ray,
        textures::{solid_color::SolidColor, texture::Texture},
    },
    util::{
        color::Color, random::rand_unit_vector, types::{Float}
    },
};

pub struct Isotropic {
    pub texture: Arc<Texture>,
}

impl Isotropic {
    pub fn new(texture: Arc<Texture>) -> Self {
        Self { texture }
    }

    pub fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        Some(ScatterRecord::skip_pdf(
            self.texture.value(hit_record.u, hit_record.v, &hit_record.point),
            Ray::new(hit_record.point, rand_unit_vector(), r_in.time),
        ))
    }
}

impl From<Color> for Isotropic {
    fn from(albedo: Color) -> Self {
        let texture = Arc::new(Texture::Solid(SolidColor::new(albedo)));
        Self::new(texture)
    }
}

impl From<[Float; 3]> for Isotropic {
    fn from(albedo: [Float; 3]) -> Self {
        let texture = Arc::new(Texture::Solid(SolidColor::from(albedo)));
        Self::new(texture)
    }
}
