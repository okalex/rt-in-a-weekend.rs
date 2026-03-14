use std::sync::Arc;

use nalgebra::{Point3, Vector3};
use parry3d_f64::bounding_volume::Aabb;

use super::hittable::{HitRecord, Hittable};
use crate::rt::interval::Interval;
use crate::rt::materials::material::Material;
use crate::rt::random::rand;
use crate::rt::ray::Ray;
use crate::rt::util::to_parry_vec;

pub struct Quad {
    q: Point3<f64>,
    u: Vector3<f64>,
    v: Vector3<f64>,
    w: Vector3<f64>,
    pub mat: Arc<dyn Material>,
    bbox: Aabb,
    normal: Vector3<f64>,
    d: f64,
    area: f64,
}

impl Quad {
    pub fn new(q: Point3<f64>, u: Vector3<f64>, v: Vector3<f64>, mat: Arc<dyn Material>) -> Self {
        let points = vec![
            to_parry_vec(q.coords),
            to_parry_vec((q + u).coords),
            to_parry_vec((q + v).coords),
            to_parry_vec((q + u + v).coords),
        ];
        let bbox = Aabb::from_points(points);

        let n = u.cross(&v);
        let normal = n.normalize();
        let d = normal.dot(&q.coords);
        let w = n / n.dot(&n);
        let area = n.magnitude();

        Self {
            q,
            u,
            v,
            w,
            mat,
            bbox,
            normal,
            d,
            area,
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

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }

    fn pdf_value(&self, origin: &Point3<f64>, direction: &Vector3<f64>) -> f64 {
        let ray = Ray::new(*origin, *direction, 0.0);
        let interval = Interval::new(0.001, f64::INFINITY);
        match self.hit(&ray, interval) {
            None => 0.0,
            Some(hit_record) => {
                let dist_sqrd = hit_record.t * hit_record.t * direction.magnitude_squared();
                let cos = (direction.dot(&hit_record.normal) / direction.magnitude()).abs();
                dist_sqrd / (cos * self.area)
            }
        }
    }

    fn random(&self, origin: &Point3<f64>) -> Vector3<f64> {
        let p = self.q + (rand() * self.u) + (rand() * self.v);
        p - origin
    }
}
