use crate::{
    rt::{
        geometry::{
            hit_record::HitRecord,
            scene::PrimitiveId,
        },
        ray::Ray,
    },
    util::{
        interval::Interval,
        random::rand,
        types::{
            Float,
            Vector,
            INFINITY,
        },
    },
};

#[derive(Clone)]
pub struct ConstantMedium {
    pub boundary_id: PrimitiveId,
    pub neg_inv_density: Float,
}

impl ConstantMedium {
    pub fn new(boundary_id: PrimitiveId, density: Float) -> Self {
        Self {
            boundary_id,
            neg_inv_density: -1.0 / density,
        }
    }

    pub fn hit<F>(&self, hit_fn: F, ray: &Ray, ray_t: Interval) -> Option<HitRecord>
    where
        F: Fn(&Ray, Interval) -> Option<HitRecord>,
    {
        let result = hit_fn(ray, Interval::universe()).and_then(|rec1| {
            let exit_interval = Interval::new(rec1.t + 0.001, INFINITY);
            match hit_fn(ray, exit_interval) {
                None => None,
                Some(rec2) => Some((rec1, rec2)),
            }
        });

        if result.is_none() {
            return None;
        }

        let (rec1, rec2) = result.unwrap();
        let mut entry = Float::max(rec1.t, ray_t.min);
        let exit = Float::min(rec2.t, ray_t.max);

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
        let normal = Vector::new(1.0, 0.0, 0.0); // arbitrary
        Some(HitRecord::new(point, normal, true, t, 0.0, 0.0))
    }
}
