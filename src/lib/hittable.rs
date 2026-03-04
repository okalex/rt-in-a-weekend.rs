use std::sync::Arc;
use crate::lib::{vec3, ray, interval, material};

pub struct HitRecord {
  pub is_hit: bool,
  pub point: vec3::Vec3,
  pub normal: vec3::Vec3,
  pub t: f64,
  pub front_face: bool,
  pub mat: Arc<dyn material::Material>,
}

pub fn no_hit() -> HitRecord {
  let mat: Arc<dyn material::Material> = Arc::new(material::default_material());
  return HitRecord {
    is_hit: false,
    point: vec3::zeroes(),
    normal: vec3::zeroes(),
    t: 0.0,
    front_face: true,
    mat: Arc::clone(&mat),
  };
}

pub fn hit_record(point: &vec3::Vec3, normal: &vec3::Vec3, front_face: bool, t: f64, mat: Arc<dyn material::Material>) -> HitRecord {
  return HitRecord {
    is_hit: true,
    point: *point,
    normal: if front_face { *normal } else { -*normal },
    t: t,
    front_face: front_face,
    mat: mat,
  };
}

pub trait Hittable: Send + Sync {
  fn hit(&self, ray: &ray::Ray, ray_t: interval::Interval) -> HitRecord;
}
