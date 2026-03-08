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
    fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut rec1: HitRecord = HitRecord::empty(Arc::clone(&rec.mat));
        let mut rec2: HitRecord = HitRecord::empty(Arc::clone(&rec.mat));

        if !self.boundary.hit(ray, Interval::universe(), &mut rec1) {
            return false;
        }

        if !self.boundary.hit(ray, Interval::new(rec1.t + 0.0001, f64::INFINITY), &mut rec2) {
            return false;
        }

        if rec1.t < ray_t.min { rec1.t = ray_t.min; }
        if rec2.t > ray_t.max { rec1.t = ray_t.max; }

        if rec1.t >= rec2.t {
            return false;
        }

        if rec1.t < 0.0 { rec1.t = 0.0; }

        let ray_len = ray.dir.length();
        let dist_inside = (rec2.t - rec1.t) * ray_len;
        let hit_dist = self.neg_inv_density * rand().ln();

        if hit_dist > dist_inside {
            return false;
        }

        rec.t = rec1.t + hit_dist / ray_len;
        rec.point = ray.at(rec.t);
        rec.normal = Vec3::new(1.0, 0.0, 0.0);
        rec.front_face = true;
        rec.mat = Arc::clone(&self.phase_fn);

        true
    }

    fn bounding_box(&self) -> AABB {
        self.boundary.bounding_box()
    }
}
