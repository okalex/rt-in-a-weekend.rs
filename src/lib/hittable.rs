use std::sync::Arc;
use crate::lib::aabb::AABB;
use crate::lib::interval::Interval;
use crate::lib::material::Material;
use crate::lib::ray::Ray;
use crate::lib::vec3::Vec3;

pub struct HitRecord {
  pub point: Vec3,
  pub normal: Vec3,
  pub t: f64,
  pub u: f64,
  pub v: f64,
  pub front_face: bool,
  pub mat: Arc<dyn Material>,
}

impl HitRecord {

  pub fn new(point: Vec3, normal: Vec3, front_face: bool, t: f64, u: f64, v:f64, mat: Arc<dyn Material>) -> Self {
    Self { point, normal, t, u, v, front_face, mat }
  }

  pub fn get_front_face(ray: &Ray, outward_normal: Vec3) -> (bool, Vec3) {
    let front_face = ray.dir.dot(&outward_normal) < 0.0;
    let face_normal = if front_face { outward_normal } else { -outward_normal };
    (front_face, face_normal)
  }

}

pub trait Hittable: Send + Sync {
  fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord>;
  fn bounding_box(&self) -> AABB;
}
