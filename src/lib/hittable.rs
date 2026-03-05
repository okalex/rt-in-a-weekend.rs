use std::sync::Arc;
use crate::lib::interval::Interval;
use crate::lib::material::{DefaultMaterial, Material};
use crate::lib::ray::Ray;
use crate::lib::vec3::Vec3;

pub struct HitRecord {
  pub is_hit: bool,
  pub point: Vec3,
  pub normal: Vec3,
  pub t: f64,
  pub front_face: bool,
  pub mat: Arc<dyn Material>,
}

impl HitRecord {

  pub fn new(point: &Vec3, normal: &Vec3, front_face: bool, t: f64, mat: Arc<dyn Material>) -> Self {
    Self {
      is_hit: true,
      point: *point,
      normal: if front_face { *normal } else { -*normal },
      t: t,
      front_face: front_face,
      mat: mat,
    }
  }

  pub fn none() -> Self {
    let mat: Arc<dyn Material> = Arc::new(DefaultMaterial::new());
    Self {
      is_hit: false,
      point: Vec3::zeroes(),
      normal: Vec3::zeroes(),
      t: 0.0,
      front_face: true,
      mat: Arc::clone(&mat),
    }
  }

}

pub trait Hittable: Send + Sync {
  fn hit(&self, ray: &Ray, ray_t: Interval) -> HitRecord;
}
