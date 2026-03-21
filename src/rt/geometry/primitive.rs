use crate::rt::{
    geometry::{
        aabb::Aabb,
        hit_record::HitRecord,
        quad::Quad,
        sphere::Sphere,
    },
    interval::Interval,
    ray::Ray,
    types::{
        Float,
        Point,
        Vector,
    },
};

pub enum Primitive {
    Sphere(Sphere),
    Quad(Quad),
}

impl Primitive {
    pub fn sphere(center: Point, radius: Float) -> Primitive {
        Self::Sphere(Sphere::stationary(center, radius))
    }

    pub fn quad(q: Point, u: Vector, v: Vector) -> Primitive {
        Self::Quad(Quad::new(q, u, v))
    }

    pub fn aabb(&self) -> Aabb {
        match self {
            Self::Sphere(s) => s.aabb,
            Self::Quad(q) => q.aabb,
        }
    }

    pub fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        match self {
            Self::Sphere(s) => s.hit(ray, ray_t),
            Self::Quad(q) => q.hit(ray, ray_t),
        }
    }
}
