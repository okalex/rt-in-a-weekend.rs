use std::f64::consts::PI;
use std::sync::Arc;

use crate::rt::objects::hittable::HitRecord;
use crate::rt::pdf::SpherePdf;
use crate::rt::ray::Ray;
use crate::rt::textures::solid_color::SolidColor;
use crate::rt::{color::Color, textures::texture::Texture};

use super::material::{Material, ScatterRecord};

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
    #[allow(unused)]
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        Some(ScatterRecord {
            attenuation: self.texture.value(rec.u, rec.v, &rec.point),
            pdf: Arc::new(SpherePdf::new()),
            skip_pdf_ray: None,
        })
    }

    #[allow(unused)]
    fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        1.0 / (4.0 * PI)
    }
}
