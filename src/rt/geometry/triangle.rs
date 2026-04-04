use glam::Vec2;

use crate::{
    rt::{
        geometry::{aabb::Aabb, hit_record::HitRecord},
        ray::Ray,
    },
    util::{
        interval::Interval,
        types::{Float, INFINITY, Point, Vector},
    },
};

#[derive(Clone)]
pub struct Triangle {
    pub v: [Point; 3],
    pub uv: [Vec2; 3],
    pub normal: [Vector; 3],
    pub e01: Vector,
    pub e02: Vector,
    pub area: Float,
    pub aabb: Aabb,
}

impl Triangle {
    pub fn new(v0: Point, v1: Point, v2: Point) -> Self {
        Self::new_with_uvs([v0, v1, v2], None, None)
    }

    pub fn new_with_uvs(v: [Point; 3], uv: Option<[Vec2; 3]>, normal: Option<[Vector; 3]>) -> Self {
        let uv = uv.unwrap_or([Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0), Vec2::new(0.0, 1.0)]);

        let e01 = v[1] - v[0];
        let e02 = v[2] - v[0];
        let cross = e01.cross(e02);
        let n = e01.cross(e02).normalize();
        let normal = normal.unwrap_or([n, n, n]);
        let area = cross.length() / 2.0;

        let aabb = Aabb::from_points(Vec::from(v));

        Self {
            v,
            uv,
            normal,
            e01,
            e02,
            area,
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
        let s = ray.orig - self.v[0];
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

        let w = 1.0 - u - v;
        let uv = self.uv[0] * w + self.uv[1] * u + self.uv[2] * v;

        let point = ray.at(t);
        let front_face = a >= 0.0;
        let mut normal = u * self.normal[0] + v * self.normal[1] + w * self.normal[2];
        normal = if front_face { normal } else { -normal };

        Some(HitRecord::new(point, normal, front_face, t, uv[0], uv[1]))
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
}
