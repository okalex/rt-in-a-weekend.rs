use std::f64::consts::PI;
use std::sync::Arc;

use nalgebra::Vector3;

use crate::rt::color::Color;
use crate::rt::materials::material::ScatterRecord;
use crate::rt::objects::hit_record::HitRecord;
use crate::rt::pdf::CosinePdf;
use crate::rt::ray::Ray;
use crate::rt::textures::solid_color::SolidColor;
use crate::rt::textures::texture::Texture;

pub struct Lambertian {
    texture: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(texture: Arc<dyn Texture>) -> Self {
        Self { texture }
    }

    pub fn from_color(color: Color) -> Self {
        let arc_color: Arc<dyn Texture> = Arc::new(SolidColor::new(color));
        Self::new(arc_color)
    }

    pub fn from_color_values(color_values: [f64; 3]) -> Self {
        Self::from_color(Color::from_arr(color_values))
    }

    #[allow(dead_code)]
    fn all_are_less_than(vec: Vector3<f64>, limit: f64) -> bool {
        (vec.x.abs() < limit) && (vec.y.abs() < limit) && (vec.z.abs() < limit)
    }

    #[allow(unused)]
    pub fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        Some(ScatterRecord {
            attenuation: self.texture.value(rec.u, rec.v, &rec.point),
            pdf: Arc::new(CosinePdf::new(&rec.normal)),
            skip_pdf_ray: None,
        })
    }

    #[allow(unused)]
    pub fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        let cos_theta = rec.normal.dot(&scattered.dir.normalize());
        if cos_theta >= 0.0 {
            cos_theta / PI
        } else {
            0.0
        }
    }
}

impl Clone for Lambertian {
    fn clone(&self) -> Self {
        Self {
            texture: Arc::clone(&self.texture),
        }
    }
}
