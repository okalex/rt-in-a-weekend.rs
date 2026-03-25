use std::sync::Arc;

use super::material::ScatterRecord;
use crate::{
    rt::{
        geometry::hit_record::HitRecord,
        pdf::{Pdf, SpherePdf},
        ray::Ray,
        textures::{solid_color::SolidColor, texture::Texture},
    },
    util::{
        color::Color,
        types::{Float, Vector},
    },
};

pub struct Isotropic {
    pub texture: Arc<Texture>,
}

impl Isotropic {
    pub fn new(texture: Arc<Texture>) -> Self {
        Self { texture }
    }

    #[allow(unused_variables)]
    pub fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        Some(ScatterRecord::with_pdf(
            self.texture.value(rec.u, rec.v, &rec.point),
            Arc::new(Pdf::Sphere(SpherePdf::new())),
        ))
    }

    #[allow(unused_variables)]
    pub fn pdf_value(&self, r_in: &Ray, rec: &HitRecord, scattered_dir: &Vector) -> Float {
        let pdf = SpherePdf::new();
        pdf.value(scattered_dir)
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
