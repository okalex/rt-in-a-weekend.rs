use std::sync::Arc;
use crate::lib::hittable::{Hittable, HitRecord};
use crate::lib::interval::Interval;
use crate::lib::ray::Ray;

pub struct Scene {
  objects: Vec<Arc<dyn Hittable>>,
}

impl Scene {

  pub fn new(objects: Vec<Arc<dyn Hittable>>) -> Scene {
    Scene {
      objects: objects,
    }
  }

  pub fn add(&mut self, object: Arc<dyn Hittable>) {
    self.objects.push(object);
  }

  pub fn hit(&self, ray: &Ray, ray_t: Interval) -> HitRecord {
    let mut closest_so_far = ray_t.max;
    let mut hit_record = HitRecord::none();
    for object in &self.objects {
      let temp_record = object.hit(ray, ray_t.update_max(closest_so_far));
      if temp_record.is_hit {
        hit_record = temp_record;
        closest_so_far = hit_record.t;
      }
    }
    return hit_record;
  }
}
