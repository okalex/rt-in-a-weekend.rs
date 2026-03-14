use std::sync::Arc;

use nalgebra::{Point3, Vector3};
use parry3d_f64::bounding_volume::{Aabb, BoundingVolume};

use super::hittable::Hittable;
use crate::rt::interval::Interval;
use crate::rt::objects::hit_record::HitRecord;
use crate::rt::random::rand_int;
use crate::rt::ray::Ray;

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

    pub fn pdf_value(&self, origin: &Point3<f64>, direction: &Vector3<f64>) -> f64 {
        let weight = 1.0 / self.objects.len() as f64;
        let mut sum = 0.0;
        for object in &self.objects {
            sum += weight * object.pdf_value(origin, direction);
        }

        sum
    }

    pub fn random(&self, origin: &Point3<f64>) -> Vector3<f64> {
        let int_size = self.objects.len() as i32;

        self.objects[rand_int(0, int_size - 1) as usize].random(origin)
    }
}
