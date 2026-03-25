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
        types::{Float, Vector},
    },
};

pub struct Lambertian {
    pub texture: Arc<Texture>,
}

impl Lambertian {
    pub fn new(texture: Arc<Texture>) -> Self {
        Self { texture }
    }

    #[allow(dead_code)]
    fn all_are_less_than(vec: Vector, limit: Float) -> bool {
        (vec.x.abs() < limit) && (vec.y.abs() < limit) && (vec.z.abs() < limit)
    }

    #[allow(unused)]
    pub fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        Some(ScatterRecord::with_pdf(
            self.texture.value(rec.u, rec.v, &rec.point),
            Arc::new(Pdf::Cosine(CosinePdf::new(&rec.normal))),
        ))
    }

    #[allow(unused)]
    pub fn pdf_value(&self, r_in: &Ray, rec: &HitRecord, scattered_dir: &Vector) -> Float {
        let pdf = CosinePdf::new(&rec.normal);
        return pdf.value(scattered_dir);
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
