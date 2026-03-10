use std::sync::Arc;

use nalgebra::{Point3, Vector3};

use crate::lib::aabb::AABB;
use crate::lib::hittable::{HitRecord, Hittable};
use crate::lib::interval::Interval;
use crate::lib::materials::material::Material;
use crate::lib::ray::Ray;

pub struct Quad {
    q: Point3<f64>,
    u: Vector3<f64>,
    v: Vector3<f64>,
    w: Vector3<f64>,
    pub mat: Arc<dyn Material>,
    bbox: AABB,
    normal: Vector3<f64>,
    d: f64,
}

impl Quad {
    pub fn new(q: Point3<f64>, u: Vector3<f64>, v: Vector3<f64>, mat: Arc<dyn Material>) -> Self {
        let diag1 = AABB::from_points(q, q + u + v);
        let diag2 = AABB::from_points(q + u, q + v);
        let bbox = AABB::from_boxes(&diag1, &diag2);
        let n = u.cross(&v);
        let normal = n.normalize();
        let d = normal.dot(&q.coords);
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

    pub fn from_arr(q: [f64; 3], u: [f64; 3], v: [f64; 3], mat: Arc<dyn Material>) -> Self {
        Self::new(Point3::from(q), Vector3::from(u), Vector3::from(v), mat)
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
        let t = (self.d - self.normal.dot(&ray.orig.coords)) / denom;
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
