use std::sync::Arc;

use nalgebra::Vector3;

use crate::rt::color::Color;
use crate::rt::materials::material::{Material, Scattered};
use crate::rt::objects::hittable::HitRecord;
use crate::rt::random::rand_unit_vector;
use crate::rt::ray::Ray;
use crate::rt::textures::solid_color::SolidColor;
use crate::rt::textures::texture::Texture;

pub struct Lambertian {
    texture: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(texture: Arc<dyn Texture>) -> Self {
        Self { texture }
    }

    pub fn from_color(color: Color) -> Self {
        let arc_color: Arc<dyn Texture> = Arc::new(SolidColor::new(color));
        Self::new(arc_color)
    }

    pub fn from_color_values(color_values: [f64; 3]) -> Self {
        Self::from_color(Color::from_arr(color_values))
    }

    fn all_are_less_than(vec: Vector3<f64>, limit: f64) -> bool {
        (vec.x.abs() < limit) && (vec.y.abs() < limit) && (vec.z.abs() < limit)
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scattered> {
        let mut scatter_dir = rec.normal + rand_unit_vector();
        if Self::all_are_less_than(scatter_dir, 1e-8) {
            scatter_dir = rec.normal;
        }

        Some(Scattered {
            ray: Ray::new(rec.point, scatter_dir, r_in.time),
            attenuation: self.texture.value(rec.u, rec.v, &rec.point),
        })
    }
}

impl Clone for Lambertian {
    fn clone(&self) -> Self {
        Self {
            texture: Arc::clone(&self.texture),
        }
    }
}
