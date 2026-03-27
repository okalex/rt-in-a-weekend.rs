use std::sync::Arc;

use crate::{
    rt::{
        geometry::hit_record::HitRecord,
        materials::material::ScatterRecord,
        pdf::{CosinePdf, Pdf},
        ray::Ray,
        textures::{solid_color::SolidColor, texture::Texture},
    },
    util::{
        color::Color,
        types::{Float, PI, Vector},
    },
};

pub struct Lambertian {
    pub texture: Arc<Texture>,
}

impl Lambertian {
    pub fn new(texture: Arc<Texture>) -> Self {
        Self { texture }
    }

    #[allow(unused_variables)]
    pub fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        Some(ScatterRecord::with_pdf(
            self.texture.value(hit_record.u, hit_record.v, &hit_record.point),
            Arc::new(Pdf::Cosine(CosinePdf::new(&hit_record.normal))),
        ))
    }

    #[allow(unused_variables)]
    pub fn brdf(&self, r_in: &Ray, hit_record: &HitRecord, scattered_dir: &Vector) -> Color {
        let attenuation = self.texture.value(hit_record.u, hit_record.v, &hit_record.point);
        attenuation / PI
    }
}

impl From<Color> for Lambertian {
    fn from(color: Color) -> Self {
        let arc_color = Arc::new(Texture::Solid(SolidColor::new(color)));
        Self::new(arc_color)
    }
}

impl From<[Float; 3]> for Lambertian {
    fn from(color_values: [Float; 3]) -> Self {
        Self::from(Color::from(color_values))
    }
}

impl Clone for Lambertian {
    fn clone(&self) -> Self {
        Self {
            texture: Arc::clone(&self.texture),
        }
    }
}
