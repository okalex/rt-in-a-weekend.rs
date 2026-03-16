use std::sync::Arc;

use crate::rt::color::Color;
use crate::rt::materials::material::ScatterRecord;
use crate::rt::objects::hit_record::HitRecord;
use crate::rt::pdf::{CosinePdf, Pdf};
use crate::rt::ray::Ray;
use crate::rt::textures::solid_color::SolidColor;
use crate::rt::textures::texture::Texture;
use crate::rt::types::{Float, PI, Vector};

pub struct Lambertian {
    texture: Arc<Texture>,
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
        Some(ScatterRecord {
            attenuation: self.texture.value(rec.u, rec.v, &rec.point),
            pdf: Arc::new(Pdf::Cosine(CosinePdf::new(&rec.normal))),
            skip_pdf_ray: None,
        })
    }

    #[allow(unused)]
    pub fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> Float {
        let cos_theta = rec.normal.dot(scattered.dir.normalize());
        if cos_theta >= 0.0 {
            cos_theta / PI
        } else {
            0.0
        }
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
