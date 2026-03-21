use crate::rt::{
    geometry::{
        aabb::Aabb, hit_record::HitRecord, sphere::Sphere
    }, interval::Interval, ray::Ray, types::{
        Float,
        Point,
    }
};

pub enum Primitive {
    Sphere(Sphere),
}

impl Primitive {
    pub fn sphere(center: Point, radius: Float) -> Primitive {
        Self::Sphere(Sphere::stationary(center, radius))
    }

    pub fn aabb(&self) -> Aabb {
        match self {
            Self::Sphere(sphere) => sphere.aabb
        }
    }

    pub fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        match self {
            Self::Sphere(s) => s.hit(ray, ray_t)
        }
    }
}
