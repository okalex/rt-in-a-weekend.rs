use std::sync::Arc;

use crate::lib::aabb::AABB;
use crate::lib::hittable::{HitRecord, Hittable};
use crate::lib::interval::Interval;
use crate::lib::material::Material;
use crate::lib::ray::Ray;
use crate::lib::vec3::Vec3;

pub struct Quad {
    q: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    pub mat: Arc<dyn Material>,
    bbox: AABB,
    normal: Vec3,
    d: f64,
}

impl Quad {
    pub fn new(q: Vec3, u: Vec3, v: Vec3, mat: Arc<dyn Material>) -> Self {
        let diag1 = AABB::from_vecs(q, q + u + v);
        let diag2 = AABB::from_vecs(q + u, q + v);
        let bbox = AABB::from_boxes(&diag1, &diag2);
        let n = u.cross(&v);
        let normal = n.unit();
        let d = normal.dot(&q);
        let w = n / n.dot(&n);
        Self {
            q,
            u,
            v,
            w,
            mat,
            bbox,
            normal,
            d,
        }
    }
}

impl Hittable for Quad {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let denom = self.normal.dot(&ray.dir);

        // Ray is parallel to plane, no hit
        if denom.abs() < 1e-8 {
            return None;
        }

        // t is outside ray interval, no hit
        let t = (self.d - self.normal.dot(&ray.orig)) / denom;
        if !ray_t.contains(t) {
            return None;
        }

        let intersection = ray.at(t);

        let planar_hit_vec = intersection - self.q;
        let alpha = self.w.dot(&planar_hit_vec.cross(&self.v));
        let beta = self.w.dot(&self.u.cross(&planar_hit_vec));

        // Check if is interior
        let unit_interval = Interval::new(0.0, 1.0);
        if !unit_interval.contains(alpha) || !unit_interval.contains(beta) {
            return None;
        }

        let (front_face, face_normal) = HitRecord::get_front_face(ray, self.normal);

        Some(HitRecord::new(
            intersection,
            face_normal,
            front_face,
            t,
            alpha,
            beta,
            Arc::clone(&self.mat),
        ))
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
