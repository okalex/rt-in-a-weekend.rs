use std::sync::Arc;

use crate::lib::aabb::AABB;
use crate::lib::color::Color;
use crate::lib::hittable::{HitRecord, Hittable};
use crate::lib::interval::Interval;
use crate::lib::materials::{isotropic::Isotropic, material::Material};
use crate::lib::random::rand;
use crate::lib::ray::Ray;
use crate::lib::textures::solid_color::SolidColor;
use crate::lib::textures::texture::Texture;
use crate::lib::vec3::Vec3;

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    neg_inv_density: f64,
    phase_fn: Arc<dyn Material>,
}

impl ConstantMedium {
    pub fn new(boundary: Arc<dyn Hittable>, density: f64, mat: Arc<dyn Material>) -> Self {
        Self {
            boundary: Arc::clone(&boundary),
            neg_inv_density: -1.0 / density,
            phase_fn: Arc::clone(&mat),
        }
    }

    pub fn from_texture(
        boundary: Arc<dyn Hittable>,
        density: f64,
        texture: Arc<dyn Texture>,
    ) -> Self {
        let mat: Arc<dyn Material> = Arc::new(Isotropic::new(texture));
        Self::new(Arc::clone(&boundary), density, Arc::clone(&mat))
    }

    pub fn from_color(boundary: Arc<dyn Hittable>, density: f64, albedo: Color) -> Self {
        let texture: Arc<dyn Texture> = Arc::new(SolidColor::new(albedo));
        Self::from_texture(Arc::clone(&boundary), density, Arc::clone(&texture))
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let result = self
            .boundary
            .hit(ray, Interval::universe())
            .and_then(|rec1| {
                let exit_interval = Interval::new(rec1.t + 0.001, f64::INFINITY);
                match self.boundary.hit(ray, exit_interval) {
                    None => None,
                    Some(rec2) => Some((rec1, rec2)),
                }
            });

        if result.is_none() {
            return None;
        }

        let (rec1, rec2) = result.unwrap();
        let mut entry = f64::max(rec1.t, ray_t.min);
        let exit = f64::min(rec2.t, ray_t.max);

        if entry >= exit {
            return None;
        }

        if entry < 0.0 {
            entry = 0.0;
        }

        let ray_len = ray.dir.length();
        let dist_inside = (exit - entry) * ray_len;
        let hit_dist = self.neg_inv_density * rand().ln();

        if hit_dist > dist_inside {
            return None;
        }

        let t = entry + hit_dist / ray_len;
        let point = ray.at(t);
        let normal = Vec3::new(1.0, 0.0, 0.0); // arbitrary
        Some(HitRecord::new(
            point,
            normal,
            true,
            t,
            0.0,
            0.0,
            Arc::clone(&self.phase_fn),
        ))
    }

    fn bounding_box(&self) -> AABB {
        self.boundary.bounding_box()
    }
}
