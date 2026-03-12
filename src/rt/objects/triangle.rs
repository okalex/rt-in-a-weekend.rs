use std::sync::Arc;

use parry3d_f64::bounding_volume::Aabb;
use parry3d_f64::math::Vec3;
use parry3d_f64::query::RayCast;
use parry3d_f64::shape::Triangle as Parry3dTriangle;

use crate::rt::interval::Interval;
use crate::rt::materials::material::Material;
use crate::rt::objects::hittable::{HitRecord, Hittable};
use crate::rt::ray::Ray;
use crate::rt::util::from_parry_vec;

pub struct Triangle {
    underlying: Parry3dTriangle,
    bbox: Aabb,
    pub mat: Arc<dyn Material>,
}

impl Triangle {
    pub fn new(a: [f64; 3], b: [f64; 3], c: [f64; 3], mat: Arc<dyn Material>) -> Self {
        let underlying = Parry3dTriangle::new(Vec3::from(a), Vec3::from(b), Vec3::from(c));
        Self {
            underlying,
            bbox: underlying.local_aabb(),
            mat,
        }
    }

    pub fn barycentric_coords(a: &Vec3, b: &Vec3, c: &Vec3, p: &Vec3) -> (f64, f64, f64) {
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
        (u, v, w)
    }
}

impl Hittable for Triangle {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let r = ray.to_parry3d();
        match self
            .underlying
            .cast_local_ray_and_get_normal(&r, ray_t.max, true)
        {
            Some(intersection) if intersection.time_of_impact >= ray_t.min => {
                let normal = intersection.normal;
                let front_face = r.dir.dot(normal) >= 0.0;
                Some(HitRecord::new(
                    ray.at(intersection.time_of_impact),
                    from_parry_vec(normal),
                    front_face,
                    intersection.time_of_impact,
                    0.0, // TODO
                    0.0, // TODO
                    Arc::clone(&self.mat),
                ))
            }

            _ => None,
        }
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
