use parry3d_f64::math::Vec3;

use crate::rt::{
    geometry::{
        aabb::Aabb,
        hit_record::HitRecord,
    },
    interval::Interval,
    ray::Ray,
    types::{
        Float,
        Point,
        Vector,
    },
};

pub struct Triangle {
    pub a: Vector,
    pub b: Vector,
    pub c: Vector,
    pub e1: Vector,
    pub e2: Vector,
    pub normal: Vector,
    pub aabb: Aabb,
    parry_aabb: parry3d_f64::bounding_volume::Aabb,
}

impl Triangle {
    pub fn new(a: Point, b: Point, c: Point) -> Self {
        let aabb = Aabb::from_points(vec![a, b, c]);
        let parry_aabb = aabb.to_parry3d();

        let e1 = b - a;
        let e2 = c - a;
        let normal = e1.cross(e2).normalize();

        Self {
            a,
            b,
            c,
            e1,
            e2,
            normal,
            aabb,
            parry_aabb,
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
        let h = ray.dir.cross(self.e2);
        let a = self.e1.dot(h);

        if a.abs() < 1e-8 {
            // Ray is parallel to plane, no hit
            return None;
        }

        let f = 1.0 / a;
        let s = ray.orig - self.a;
        let u = f * (s.dot(h));

        if u < 0.0 || u > 1.0 {
            // Outside of triangle, no hit
            return None;
        }

        let q = s.cross(self.e1);
        let v = f * ray.dir.dot(q);

        if v < 0.0 || (u + v) > 1.0 {
            // Outside of triangle, no hit
            return None;
        }

        let t = f * self.e2.dot(q);

        if !ray_t.contains(t) {
            return None;
        }

        let point = ray.at(t);
        let front_face = a >= 0.0;
        let normal = if front_face { self.normal } else { -self.normal };

        Some(HitRecord::new(point, normal, front_face, t, u, v))
    }

    pub fn bounding_box(&self) -> &parry3d_f64::bounding_volume::Aabb {
        &self.parry_aabb
    }
}
