use std::sync::Arc;

use crate::rt::objects::hit_record::HitRecord;
use crate::rt::pdf::{Pdf, SpherePdf};
use crate::rt::ray::Ray;
use crate::rt::textures::solid_color::SolidColor;
use crate::rt::types::{Float, PI};
use crate::rt::{color::Color, textures::texture::Texture};

use super::material::ScatterRecord;

pub struct Isotropic {
    texture: Arc<Texture>,
}

impl Isotropic {
    pub fn new(texture: Arc<Texture>) -> Self {
        Self { texture }
    }

    #[allow(unused)]
    pub fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        Some(ScatterRecord {
            attenuation: self.texture.value(rec.u, rec.v, &rec.point),
            pdf: Arc::new(Pdf::Sphere(SpherePdf::new())),
            skip_pdf_ray: None,
        })
    }

    #[allow(unused)]
    pub fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> Float {
        1.0 / (4.0 * PI)
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
