use std::sync::Arc;

use crate::lib::aabb::AABB;
use crate::lib::hittable::{Hittable, HitRecord};
use crate::lib::interval::Interval;
use crate::lib::ray::Ray;

pub struct Scene {
  pub objects: Vec<Arc<dyn Hittable>>,
  pub bbox: AABB,
}

impl Scene {

  pub fn new() -> Scene {
    Scene {
      objects: vec![],
      bbox: AABB::empty(),
    }
  }

  pub fn new_obj(object: Arc<dyn Hittable>) -> Scene {
    let bbox = object.bounding_box();
    Scene {
      objects: vec![object],
      bbox,
    }
  }

  pub fn add(&mut self, object: Arc<dyn Hittable>) {
    self.bbox = AABB::from_boxes(&self.bbox, &object.bounding_box());
    self.objects.push(object);
  }

  pub fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
    let mut closest_so_far = ray_t.max;
    let mut hit_record: Option<HitRecord> = None;

    for object in &self.objects {
      match object.hit(ray, ray_t.update_max(closest_so_far)) {
        Some(rec) => {
          closest_so_far = rec.t;
          hit_record = Some(rec);
        },
        None => {},
      };
    }

    return hit_record;
  }
}
