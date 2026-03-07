use std::sync::Arc;

use crate::lib::color::Color;
use crate::lib::hittable::HitRecord;
use crate::lib::materials::material::{Material, Scattered};
use crate::lib::ray::Ray;
use crate::lib::textures::solid_color::SolidColor;
use crate::lib::textures::texture::Texture;
use crate::lib::vec3::Vec3;

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
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scattered> {
        let mut scatter_dir = rec.normal + Vec3::rand_unit();
        if scatter_dir.all_are_less_than(1e-8) {
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
