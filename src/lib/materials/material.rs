use nalgebra::{Point3, Vector3};

use crate::lib::color::Color;
use crate::lib::hittable::HitRecord;
use crate::lib::ray::Ray;

pub struct Scattered {
    pub ray: Ray,
    pub attenuation: Color,
}

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scattered> {
        None
    }

    fn emitted(&self, u: f64, v: f64, point: &Point3<f64>) -> Color {
        Color::black()
    }
}

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
