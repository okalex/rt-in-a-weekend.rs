use std::sync::Arc;

use super::material::{
    reflect,
    ScatterRecord,
};
use crate::{
    rt::{
        geometry::hit_record::HitRecord,
        pdf::{
            Pdf,
            SpherePdf,
        },
        ray::Ray,
    },
    util::{
        color::Color,
        random::rand_unit_vector,
        types::Float,
    },
};

pub struct Metal {
    pub albedo: Color,
    pub fuzz: Float,
}

impl Metal {
    pub fn new(albedo: [Float; 3], fuzz: Float) -> Self {
        return Self {
            albedo: Color::from(albedo),
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        };
    }

    pub fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let reflected = reflect(r_in.dir, rec.normal).normalize() + rand_unit_vector() * self.fuzz;

        Some(ScatterRecord {
            attenuation: self.albedo,
            pdf: Arc::new(Pdf::Sphere(SpherePdf::new())), // This isn't actually used - this field should probably be an option
            skip_pdf_ray: Some(Ray::new(rec.point, reflected, r_in.time)),
        })
    }
}
