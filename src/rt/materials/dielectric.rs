use std::sync::Arc;

use crate::rt::{color::Color, pdf::SpherePdf};
use crate::rt::objects::hittable::HitRecord;
use crate::rt::random::rand;
use crate::rt::ray::Ray;

use super::material::{Material, ScatterRecord, reflect, reflectance, refract};

pub struct Dielectric {
    refraction_idx: f64,
}

impl Dielectric {
    pub fn new(refraction_idx: f64) -> Self {
        Self {
            refraction_idx: refraction_idx,
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let ri = if rec.front_face {
            1.0 / self.refraction_idx
        } else {
            self.refraction_idx
        };

        let unit_dir = r_in.dir.normalize();
        let cos_theta = f64::min((-unit_dir).dot(&rec.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = (ri * sin_theta) > 1.0;
        let direction = if cannot_refract || reflectance(cos_theta, ri) > rand() {
            reflect(&unit_dir, &rec.normal)
        } else {
            refract(&unit_dir, &rec.normal, ri)
        };

        Some(ScatterRecord {
            attenuation: Color::white(),
            pdf: Arc::new(SpherePdf::new()), // TODO
            skip_pdf_ray: Some(Ray::new(rec.point, direction, r_in.time)),
        })
    }
}
