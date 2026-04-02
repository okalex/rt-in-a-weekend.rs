use std::sync::Arc;

use super::material::ScatterRecord;
use crate::{
    rt::{
        geometry::hit_record::HitRecord,
        pdf::Pdf,
        ray::Ray,
        textures::{solid_color::SolidColor, texture::Texture},
    },
    util::{
        color::Color,
        types::{Float, PI, Vector},
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
    pub fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        Some(ScatterRecord::with_pdf(
            self.texture.value(hit_record.u, hit_record.v, &hit_record.point),
            Arc::new(Pdf::sphere()),
        ))
    }

    #[allow(unused_variables)]
    pub fn brdf(&self, r_in: &Ray, hit_record: &HitRecord, scattered_dir: &Vector) -> Color {
        let attenuation = self.texture.value(hit_record.u, hit_record.v, &hit_record.point);
        attenuation / (4.0 * PI)
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
