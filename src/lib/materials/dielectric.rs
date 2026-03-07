use crate::lib::color::Color;
use crate::lib::hittable::HitRecord;
use crate::lib::random::rand;
use crate::lib::ray::Ray;

use super::material::{Material, reflect, reflectance, refract, Scattered};

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
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scattered> {
        let ri = if rec.front_face {
            1.0 / self.refraction_idx
        } else {
            self.refraction_idx
        };

        let unit_dir = r_in.dir.unit();
        let cos_theta = f64::min((-unit_dir).dot(&rec.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = (ri * sin_theta) > 1.0;
        let direction = if cannot_refract || reflectance(cos_theta, ri) > rand() {
            reflect(&unit_dir, &rec.normal)
        } else {
            refract(&unit_dir, &rec.normal, ri)
        };

        Some(Scattered {
            ray: Ray::new(rec.point, direction, r_in.time),
            attenuation: Color::white(),
        })
    }
}
