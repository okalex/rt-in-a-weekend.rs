use parry3d_f64::bounding_volume::Aabb;
use parry3d_f64::math::Vec3;
use parry3d_f64::query::RayCast;
use parry3d_f64::shape::Triangle as Parry3dTriangle;

use crate::rt::interval::Interval;
use crate::rt::objects::hit_record::HitRecord;
use crate::rt::ray::Ray;
use crate::rt::types::{Float, from_parry_vec, new_parry_vec};

pub struct Triangle {
    underlying: Parry3dTriangle,
    bbox: Aabb,
    pub mat_idx: usize,
}

impl Triangle {
    pub fn new(a: [Float; 3], b: [Float; 3], c: [Float; 3], mat_idx: usize) -> Self {
        let underlying = Parry3dTriangle::new(new_parry_vec(a), new_parry_vec(b), new_parry_vec(c));
        Self {
            underlying,
            bbox: underlying.local_aabb(),
            mat_idx,
        }
    }

    pub fn barycentric_coords(a: &Vec3, b: &Vec3, c: &Vec3, p: &Vec3) -> (Float, Float, Float) {
        let v0 = b - a;
        let v1 = c - a;
        let v2 = p - a;
        let d00 = v0.dot(v0);
        let d01 = v0.dot(v1);
        let d11 = v1.dot(v1);
        let d20 = v2.dot(v0);
        let d21 = v2.dot(v1);
        let denom = d00 * d11 - d01 * d01;
        let v = (d11 * d20 - d01 * d21) / denom;
        let w = (d00 * d21 - d01 * d20) / denom;
        let u = 1.0 - v - w;
        (u as Float, v as Float, w as Float)
    }

    pub fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let r = ray.to_parry3d();
        match self
            .underlying
            .cast_local_ray_and_get_normal(&r, ray_t.max as f64, true)
        {
            Some(intersection) if intersection.time_of_impact >= (ray_t.min as f64) => {
                let normal = intersection.normal;
                let front_face = r.dir.dot(normal) >= 0.0;
                Some(HitRecord::new(
                    ray.at(intersection.time_of_impact as Float),
                    from_parry_vec(normal),
                    front_face,
                    intersection.time_of_impact as Float,
                    0.0, // TODO
                    0.0, // TODO
                    self.mat_idx,
                ))
            }

            _ => None,
        }
    }

    pub fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
