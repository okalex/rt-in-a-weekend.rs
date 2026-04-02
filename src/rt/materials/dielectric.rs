use super::material::{ScatterRecord, reflectance, refract};
use crate::{
    rt::{geometry::hit_record::HitRecord, ray::Ray},
    util::{color::Color, random::rand, types::Float, vector_ext::VectorExt},
};

pub struct Dielectric {
    pub albedo: Color,
    pub refraction_idx: Float,
}

impl Dielectric {
    pub fn new(albedo: Color, refraction_idx: Float) -> Self {
        Self { albedo, refraction_idx }
    }

    pub fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let ri = if hit_record.front_face {
            1.0 / self.refraction_idx
        } else {
            self.refraction_idx
        };

        let unit_dir = r_in.dir.normalize();
        let cos_theta = Float::min((-unit_dir).dot(hit_record.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = (ri * sin_theta) > 1.0;
        let (attenuation, direction) = if cannot_refract || reflectance(cos_theta, ri) > rand() {
            (Color::white(), VectorExt::reflect(-unit_dir, hit_record.normal))
        } else {
            (self.albedo, refract(unit_dir, hit_record.normal, ri))
        };

        Some(ScatterRecord::skip_pdf(
            attenuation,
            Ray::new(hit_record.point, direction, r_in.time),
        ))
    }
}
