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
