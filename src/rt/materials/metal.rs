use super::material::ScatterRecord;
use crate::{
    rt::{geometry::hit_record::HitRecord, ray::Ray},
    util::{color::Color, random::rand_on_hemisphere, types::Float, vector_ext::VectorExt},
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

    pub fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let reflected = VectorExt::reflect(-r_in.dir, hit_record.normal).normalize()
            + rand_on_hemisphere(hit_record.normal) * self.fuzz;

        Some(ScatterRecord::skip_pdf(
            self.albedo,
            Ray::new(hit_record.point, reflected, r_in.time),
        ))
    }
}
