use std::sync::Arc;
use crate::lib::aabb::AABB;
use crate::lib::hittable::{Hittable, HitRecord};
use crate::lib::interval::Interval;
use crate::lib::material::Material;
use crate::lib::ray::Ray;
use crate::lib::vec3::Vec3;

pub struct Sphere {
  pub center: Ray,
  pub radius: f64,
  pub mat: Arc<dyn Material>,
  bbox: AABB,
}

impl Sphere {

  pub fn new(center: Ray, radius: f64, mat: Arc<dyn Material>, bbox: AABB) -> Sphere {
    Sphere {
      center,
      radius,
      mat,
      bbox,
    }
  }

  pub fn stationary(center: Vec3, radius: f64, mat: Arc<dyn Material>) -> Sphere {
    let ray = Ray::new(center, Vec3::zeroes(), 0.0);
    let rvec = Vec3::new(radius, radius, radius);
    let bbox = AABB::from_vecs(center - rvec, center + rvec);
    Self::new(ray, radius, mat, bbox)
  }

  pub fn moving(center1: Vec3, center2: Vec3, radius: f64, mat: Arc<dyn Material>) -> Sphere {
    let ray = Ray::new(center1, center2 - center1, 0.0);
    let rvec = Vec3::new(radius, radius, radius);
    let box1 = AABB::from_vecs(ray.at(0.0) - rvec, ray.at(0.0) + rvec);
    let box2 = AABB::from_vecs(ray.at(1.0) - rvec, ray.at(1.0) + rvec);
    let bbox = AABB::from_boxes(&box1, &box2);
    Self::new(ray, radius, mat, bbox)
  }

  pub fn new_arr(center: [f64; 3], radius: f64, mat: Arc<dyn Material>) -> Sphere {
    Self::stationary(Vec3::new_arr(center), radius, mat)
  }

}

impl Hittable for Sphere {

  fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
    let curr_center = self.center.at(ray.time);
    let oc = curr_center - ray.orig;
    let a = ray.dir.length_squared();
    let h = ray.dir.dot(&oc);
    let c = oc.length_squared() - self.radius * self.radius;
    let discriminant = (h * h) - (a * c);

    if discriminant < 0.0 {
      return None;
    }

    let sqrtd = discriminant.sqrt();
    let mut root = (h - sqrtd) / a;
    if !ray_t.surrounds(root) {
      root = (h + sqrtd) / a;
      if !ray_t.surrounds(root) {
        return None;
      }
    }

    let point = ray.at(root);
    let normal = (point - curr_center).scale(1.0 / self.radius);
    let front_face = ray.dir.dot(&normal) < 0.0;
    return Some(HitRecord::new(&point, &normal, front_face, root, Arc::clone(&self.mat)));
  }

  fn bounding_box(&self) -> AABB {
    self.bbox
  }

}
