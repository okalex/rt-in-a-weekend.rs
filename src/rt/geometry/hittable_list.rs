use std::sync::Arc;

use parry3d_f64::bounding_volume::{
    Aabb,
    BoundingVolume,
};

use super::hittable::Hittable;
use crate::rt::{
    geometry::hit_record::HitRecord,
    interval::Interval,
    ray::Ray,
};

pub struct HittableList {
    pub objects: Vec<Arc<Hittable>>,
    pub bbox: Aabb,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: vec![],
            bbox: Aabb::new_invalid(),
        }
    }

    pub fn add(&mut self, object: Hittable) {
        self.bbox = self.bbox.merged(object.bounding_box());
        self.objects.push(Arc::new(object));
    }

    pub fn add_arc(&mut self, object: Arc<Hittable>) {
        self.bbox = self.bbox.merged(object.bounding_box());
        self.objects.push(object);
    }

    pub fn len(&self) -> usize {
        self.objects.len()
    }

    pub fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut closest_so_far = ray_t.max;
        let mut hit_record: Option<HitRecord> = None;

        for object in &self.objects {
            match object.hit(ray, ray_t.update_max(closest_so_far)) {
                None => {}
                Some(rec) => {
                    closest_so_far = rec.t;
                    hit_record = Some(rec);
                }
            };
        }

        return hit_record;
    }

    pub fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
