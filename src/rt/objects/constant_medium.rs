use std::sync::Arc;

use nalgebra::Vector3;
use parry3d_f64::bounding_volume::Aabb;

use super::hittable::Hittable;
use crate::rt::interval::Interval;
use crate::rt::objects::hit_record::HitRecord;
use crate::rt::random::rand;
use crate::rt::ray::Ray;

pub struct ConstantMedium {
    boundary: Arc<Hittable>,
    neg_inv_density: f64,
    phase_fn_mat_idx: usize,
}

impl ConstantMedium {
    pub fn new(boundary: Arc<Hittable>, density: f64, mat_idx: usize) -> Self {
        Self {
            boundary: boundary,
            neg_inv_density: -1.0 / density,
            phase_fn_mat_idx: mat_idx,
        }
    }

    pub fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
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

        let ray_len = ray.dir.magnitude();
        let dist_inside = (exit - entry) * ray_len;
        let hit_dist = self.neg_inv_density * rand().ln();

        if hit_dist > dist_inside {
            return None;
        }

        let t = entry + hit_dist / ray_len;
        let point = ray.at(t);
        let normal = Vector3::new(1.0, 0.0, 0.0); // arbitrary
        Some(HitRecord::new(
            point,
            normal,
            true,
            t,
            0.0,
            0.0,
            self.phase_fn_mat_idx,
        ))
    }

    pub fn bounding_box(&self) -> &Aabb {
        self.boundary.bounding_box()
    }
}
