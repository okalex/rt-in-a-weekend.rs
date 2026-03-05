use std::sync::Arc;
use crate::lib::{hittable, interval};
use crate::lib::ray::Ray;

pub struct Scene {
  objects: Vec<Arc<dyn hittable::Hittable>>,
}

impl Scene {
  pub fn add(&mut self, object: Arc<dyn hittable::Hittable>) {
    self.objects.push(object);
  }

  pub fn hit(&self, ray: &Ray, ray_t: interval::Interval) -> hittable::HitRecord {
    let mut closest_so_far = ray_t.max();
    let mut hit_record = hittable::no_hit();
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

pub fn new(objects: Vec<Arc<dyn hittable::Hittable>>) -> Scene {
  return Scene {
    objects: objects,
  }
}
