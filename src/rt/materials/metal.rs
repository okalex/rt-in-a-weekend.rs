use std::sync::Arc;

use crate::rt::objects::hit_record::HitRecord;
use crate::rt::pdf::SpherePdf;
use crate::rt::ray::Ray;
use crate::rt::{color::Color, random::rand_unit_vector};

use super::material::{ScatterRecord, reflect};

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: [f64; 3], fuzz: f64) -> Self {
        return Self {
            albedo: Color::from_arr(albedo),
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        };
    }

    pub fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let reflected =
            reflect(&r_in.dir, &rec.normal).normalize() + rand_unit_vector() * self.fuzz;

        Some(ScatterRecord {
            attenuation: self.albedo,
            pdf: Arc::new(SpherePdf::new()), // This isn't actually used - this field should probably be an option
            skip_pdf_ray: Some(Ray::new(rec.point, reflected, r_in.time)),
        })
    }
}
