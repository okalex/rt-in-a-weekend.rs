use super::material::{reflect, ScatterRecord};
use crate::{
    rt::{geometry::hit_record::HitRecord, ray::Ray},
    util::{color::Color, random::rand_on_hemisphere, types::Float},
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
        let reflected = reflect(r_in.dir, rec.normal).normalize() + rand_on_hemisphere(rec.normal) * self.fuzz;

        Some(ScatterRecord::skip_pdf(self.albedo, Ray::new(rec.point, reflected, r_in.time)))
    }
}
