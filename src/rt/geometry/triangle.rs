use glam::Vec2;

use crate::{
    rt::{
        geometry::{
            aabb::Aabb,
            hit_record::HitRecord,
        },
        ray::Ray,
    },
    util::{
        interval::Interval,
        types::{
            Point,
            Vector,
        },
    },
};

#[derive(Clone)]
pub struct Triangle {
    pub v0: Point,
    pub v1: Point,
    pub v2: Point,
    pub uv0: Vec2,
    pub uv1: Vec2,
    pub uv2: Vec2,
    pub e01: Vector,
    pub e02: Vector,
    pub normal: Vector,
    pub aabb: Aabb,
}

impl Triangle {
    pub fn new(v0: Point, v1: Point, v2: Point) -> Self {
        let uv0 = Vec2::new(0.0, 0.0);
        let uv1 = Vec2::new(1.0, 0.0);
        let uv2 = Vec2::new(0.0, 1.0);

        Self::new_with_uvs(v0, v1, v2, uv0, uv1, uv2)
    }

    pub fn new_with_uvs(v0: Point, v1: Point, v2: Point, uv0: Vec2, uv1: Vec2, uv2: Vec2) -> Self {
        let e01 = v1 - v0;
        let e02 = v2 - v0;
        let normal = e01.cross(e02).normalize();

        let aabb = Aabb::from_points(vec![v0, v1, v2]);

        Self {
            v0,
            v1,
            v2,
            uv0,
            uv1,
            uv2,
            e01,
            e02,
            normal,
            aabb,
        }
    }

    pub fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let h = ray.dir.cross(self.e02);
        let a = self.e01.dot(h);

        if a.abs() < 1e-8 {
            // Ray is parallel to plane, no hit
            return None;
        }

        let f = 1.0 / a;
        let s = ray.orig - self.v0;
        let u = f * (s.dot(h));

        if u < 0.0 || u > 1.0 {
            // Outside of triangle, no hit
            return None;
        }

        let q = s.cross(self.e01);
        let v = f * ray.dir.dot(q);

        if v < 0.0 || (u + v) > 1.0 {
            // Outside of triangle, no hit
            return None;
        }

        let t = f * self.e02.dot(q);

        if !ray_t.contains(t) {
            return None;
        }

        let point = ray.at(t);
        let front_face = a >= 0.0;
        let normal = if front_face { self.normal } else { -self.normal };

        let w = 1.0 - u - v;
        let uv = self.uv0 * w + self.uv1 * u + self.uv2 * v;

        Some(HitRecord::new(point, normal, front_face, t, uv[0], uv[1]))
    }
}
