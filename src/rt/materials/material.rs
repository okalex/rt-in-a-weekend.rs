use std::sync::Arc;

use nalgebra::Vector3;

use crate::rt::color::Color;
use crate::rt::objects::hittable::HitRecord;
use crate::rt::pdf::Pdf;
use crate::rt::ray::Ray;

pub struct ScatterRecord {
    pub attenuation: Color,
    pub pdf: Arc<dyn Pdf>,
    pub skip_pdf_ray: Option<Ray>,
}

pub trait Material: Send + Sync {
    #[allow(unused)]
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        None
    }

    #[allow(unused)]
    fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        0.0
    }

    #[allow(unused)]
    fn emitted(&self, r_in: &Ray, hit_record: &HitRecord) -> Color {
        Color::black()
    }
}

pub struct EmptyMaterial {}

impl EmptyMaterial {
    pub fn new() -> Self {
        Self {}
    }
}

impl Material for EmptyMaterial {}

pub fn reflectance(cosine: f64, refraction_idx: f64) -> f64 {
    let r0_tmp = (1.0 - refraction_idx) / (1.0 + refraction_idx);
    let r0 = r0_tmp * r0_tmp;
    return r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0);
}

pub fn reflect(vec: &Vector3<f64>, normal: &Vector3<f64>) -> Vector3<f64> {
    *vec - *normal * vec.dot(normal) * 2.0
}

pub fn refract(vec: &Vector3<f64>, n: &Vector3<f64>, etai_over_etat: f64) -> Vector3<f64> {
    let cos_theta = f64::min(-vec.dot(n), 1.0);
    let r_out_perp = (*vec + *n * cos_theta) * etai_over_etat;
    let r_out_parallel = *n * (-(1.0 - r_out_perp.magnitude_squared()).abs().sqrt());
    return r_out_perp + r_out_parallel;
}
