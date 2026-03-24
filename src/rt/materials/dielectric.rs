use std::sync::Arc;

use super::material::{ScatterRecord, reflect, reflectance, refract};
use crate::{
    rt::{
        geometry::hit_record::HitRecord,
        pdf::{Pdf, SpherePdf},
        ray::Ray,
    },
    util::{color::Color, random::rand, types::Float},
};

pub struct Dielectric {
    pub albedo: Color,
    pub refraction_idx: Float,
}

impl Dielectric {
    pub fn new(albedo: Color, refraction_idx: Float) -> Self {
        Self { albedo, refraction_idx }
    }

    pub fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let ri = if rec.front_face {
            1.0 / self.refraction_idx
        } else {
            self.refraction_idx
        };

        let unit_dir = r_in.dir.normalize();
        let cos_theta = Float::min((-unit_dir).dot(rec.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = (ri * sin_theta) > 1.0;
        let (attenuation, direction) = if cannot_refract || reflectance(cos_theta, ri) > rand() {
            (Color::white(), reflect(unit_dir, rec.normal))
        } else {
            (self.albedo, refract(unit_dir, rec.normal, ri))
        };

        Some(ScatterRecord {
            attenuation: attenuation,
            pdf: Arc::new(Pdf::Sphere(SpherePdf::new())), // TODO
            skip_pdf_ray: Some(Ray::new(rec.point, direction, r_in.time)),
        })
    }
}
