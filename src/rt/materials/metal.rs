use crate::rt::{color::Color, random::rand_unit_vector};
use crate::rt::hittable::HitRecord;
use crate::rt::ray::Ray;

use super::material::{Material, reflect, Scattered};

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
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scattered> {
        let reflected = reflect(&r_in.dir, &rec.normal).normalize() + rand_unit_vector() * self.fuzz;

        Some(Scattered {
            ray: Ray::new(rec.point, reflected, r_in.time),
            attenuation: self.albedo,
        })
    }
}
