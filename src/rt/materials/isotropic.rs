use std::sync::Arc;

use super::material::ScatterRecord;
use crate::{
    rt::{
        geometry::hit_record::HitRecord,
        pdf::{
            Pdf,
            SpherePdf,
        },
        ray::Ray,
        textures::{
            solid_color::SolidColor,
            texture::Texture,
        },
    },
    util::{
        color::Color,
        types::{
            Float,
            PI,
        },
    },
};

pub struct Isotropic {
    pub texture: Arc<Texture>,
}

impl Isotropic {
    pub fn new(texture: Arc<Texture>) -> Self {
        Self { texture }
    }

    pub fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        Some(ScatterRecord {
            attenuation: self.texture.value(rec.u, rec.v, &rec.point),
            pdf: Arc::new(Pdf::Sphere(SpherePdf::new())),
            skip_pdf_ray: None,
        })
    }

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
