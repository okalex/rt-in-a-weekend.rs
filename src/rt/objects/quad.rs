use parry3d_f64::bounding_volume::Aabb;

use crate::rt::interval::Interval;
use crate::rt::objects::hit_record::HitRecord;
use crate::rt::random::rand;
use crate::rt::ray::Ray;
use crate::rt::types::{Float, INFINITY, Point, Vector, to_parry_vec};

pub struct Quad {
    q: Point,
    u: Vector,
    v: Vector,
    w: Vector,
    pub mat_idx: usize,
    bbox: Aabb,
    normal: Vector,
    d: Float,
    area: Float,
}

impl Quad {
    pub fn new(q: Point, u: Vector, v: Vector, mat_idx: usize) -> Self {
        let points = vec![
            to_parry_vec(q),
            to_parry_vec(q + u),
            to_parry_vec(q + v),
            to_parry_vec(q + u + v),
        ];
        let bbox = Aabb::from_points(points);

        let n = u.cross(v);
        let normal = n.normalize();
        let d = normal.dot(q);
        let w = n / n.dot(n);
        let area = n.length();

        Self {
            q,
            u,
            v,
            w,
            mat_idx,
            bbox,
            normal,
            d,
            area,
        }
    }

    pub fn from_arr(q: [Float; 3], u: [Float; 3], v: [Float; 3], mat_idx: usize) -> Self {
        Self::new(Point::from(q), Vector::from(u), Vector::from(v), mat_idx)
    }

    pub fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let denom = self.normal.dot(ray.dir);

        // Ray is parallel to plane, no hit
        if denom.abs() < 1e-8 {
            return None;
        }

        // t is outside ray interval, no hit
        let t = (self.d - self.normal.dot(ray.orig)) / denom;
        if !ray_t.contains(t) {
            return None;
        }

        let intersection = ray.at(t);

        let planar_hit_vec = intersection - self.q;
        let alpha = self.w.dot(planar_hit_vec.cross(self.v));
        let beta = self.w.dot(self.u.cross(planar_hit_vec));

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
            self.mat_idx,
        ))
    }

    pub fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }

    pub fn pdf_value(&self, origin: &Point, direction: &Vector) -> Float {
        let ray = Ray::new(*origin, *direction, 0.0);
        let interval = Interval::new(0.001, INFINITY);
        match self.hit(&ray, interval) {
            None => 0.0,
            Some(hit_record) => {
                let dist_sqrd = hit_record.t * hit_record.t * direction.length_squared();
                let cos = (direction.dot(hit_record.normal) / direction.length()).abs();
                dist_sqrd / (cos * self.area)
            }
        }
    }

    pub fn random(&self, origin: &Point) -> Vector {
        let p = self.q + (rand() * self.u) + (rand() * self.v);
        p - origin
    }
}
