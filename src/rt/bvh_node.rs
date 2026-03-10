use std::cmp::Ordering;
use std::sync::Arc;

use crate::rt::aabb::AABB;
use crate::rt::hittable::{HitRecord, Hittable};
use crate::rt::interval::Interval;
use crate::rt::ray::Ray;
use crate::rt::scene::Scene;

pub struct BvhNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: AABB,
}

impl BvhNode {
    pub fn new(left: Arc<dyn Hittable>, right: Arc<dyn Hittable>) -> BvhNode {
        let bbox = AABB::from_boxes(&left.bounding_box(), &right.bounding_box());
        BvhNode { left, right, bbox }
    }

    pub fn construct(objects: &mut Vec<Arc<dyn Hittable>>, start: usize, end: usize) -> BvhNode {
        let mut bbox = AABB::empty();
        for object_idx in start..end {
            bbox = AABB::from_boxes(&bbox, &objects[object_idx].bounding_box())
        }
        let axis_idx = bbox.longest_axis();

        let object_span = end - start;
        if object_span == 1 {
            Self::new(Arc::clone(&objects[start]), Arc::clone(&objects[start]))
        } else if object_span == 2 {
            Self::new(Arc::clone(&objects[start]), Arc::clone(&objects[start + 1]))
        } else {
            objects[start..end].sort_by(Self::box_compare(axis_idx));
            let mid = start + object_span / 2;
            let left = Arc::new(Self::construct(objects, start, mid));
            let right = Arc::new(Self::construct(objects, mid, end));
            Self::new(left, right)
        }
    }

    pub fn from_scene(scene: Scene) -> BvhNode {
        let mut objects = scene.objects;
        let len = objects.len();
        Self::construct(&mut objects, 0, len)
    }

    fn box_compare(
        axis_idx: usize,
    ) -> impl FnMut(&Arc<dyn Hittable>, &Arc<dyn Hittable>) -> Ordering {
        move |a, b| {
            let a_min = a.bounding_box().axis_interval(axis_idx).min;
            let b_min = b.bounding_box().axis_interval(axis_idx).min;
            a_min.partial_cmp(&b_min).unwrap_or(Ordering::Equal)
        }
    }
}

impl Hittable for BvhNode {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        if !self.bbox.hit(ray, ray_t) {
            return None;
        }

        let rec_left = self.left.hit(ray, ray_t);
        let int_max = match rec_left.as_ref() {
            Some(rec) => rec.t,
            None => ray_t.max,
        };
        let rec_right = self.right.hit(ray, Interval::new(ray_t.min, int_max));

        rec_right.or(rec_left)
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
