use std::sync::Arc;
use crate::lib::hittable::{Hittable, HitRecord};
use crate::lib::interval::Interval;
use crate::lib::material::Material;
use crate::lib::ray::Ray;
use crate::lib::vec3::Vec3;

pub struct Sphere {
  pub center: Vec3,
  pub radius: f64,
  pub mat: Arc<dyn Material>,
}

impl Sphere {

  pub fn new(center: Vec3, radius: f64, mat: Arc<dyn Material>) -> Sphere {
    Sphere {
      center: center,
      radius: radius,
      mat: mat,
    }
  }

  pub fn new_arr(center: [f64; 3], radius: f64, mat: Arc<dyn Material>) -> Sphere {
    Self::new(Vec3::new_arr(center), radius, mat)
  }

}

impl Hittable for Sphere {

  fn hit(&self, ray: &Ray, ray_t: Interval) -> HitRecord {
    let oc = self.center - ray.orig;
    let a = ray.dir.length_squared();
    let h = ray.dir.dot(&oc);
    let c = oc.length_squared() - self.radius * self.radius;
    let discriminant = (h * h) - (a * c);

    if discriminant < 0.0 {
      return HitRecord::none();
    }

    let sqrtd = discriminant.sqrt();
    let mut root = (h - sqrtd) / a;
    if !ray_t.surrounds(root) {
      root = (h + sqrtd) / a;
      if !ray_t.surrounds(root) {
        return HitRecord::none();
      }
    }

    let point = ray.at(root);
    let normal = (point - self.center).scale(1.0 / self.radius);
    let front_face = ray.dir.dot(&normal) < 0.0;
    return HitRecord::new(&point, &normal, front_face, root, Arc::clone(&self.mat));
  }

}
