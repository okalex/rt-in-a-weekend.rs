use std::cmp::Ordering;
use std::sync::Arc;

use parry3d_f64::bounding_volume::{Aabb, BoundingVolume};
use parry3d_f64::query::RayCast;

use super::hittable::{HitRecord, Hittable};
use super::hittable_list::HittableList;
use crate::rt::interval::Interval;
use crate::rt::ray::Ray;

pub struct BvhNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: Aabb,
}

impl BvhNode {
    pub fn new(left: Arc<dyn Hittable>, right: Arc<dyn Hittable>) -> BvhNode {
        let bbox = left.bounding_box().merged(right.bounding_box());
        BvhNode { left, right, bbox }
    }

    pub fn construct(objects: &mut Vec<Arc<dyn Hittable>>, start: usize, end: usize) -> BvhNode {
        let mut bbox = Aabb::new_invalid();
        for object_idx in start..end {
            bbox = bbox.merged(&objects[object_idx].bounding_box());
        }
        let axis_idx = longest_axis(&bbox);

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

    pub fn from_list(list: HittableList) -> BvhNode {
        let mut objects = list.objects;
        let len = objects.len();
        Self::construct(&mut objects, 0, len)
    }

    fn box_compare(
        axis_idx: usize,
    ) -> impl FnMut(&Arc<dyn Hittable>, &Arc<dyn Hittable>) -> Ordering {
        move |a, b| {
            let a_min = a.bounding_box().mins[axis_idx];
            let b_min = b.bounding_box().mins[axis_idx];
            a_min.partial_cmp(&b_min).unwrap_or(Ordering::Equal)
        }
    }
}

impl Hittable for BvhNode {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let r = ray.to_parry3d();
        match self.bbox.cast_local_ray(&r, ray_t.max, false) {
            Some(toi) if toi >= ray_t.min => {}
            _ => return None,
        }

        let rec_left = self.left.hit(ray, ray_t);
        let int_max = match rec_left.as_ref() {
            Some(rec) => rec.t,
            None => ray_t.max,
        };
        let rec_right = self.right.hit(ray, Interval::new(ray_t.min, int_max));

        match (rec_left, rec_right) {
            (None, None) => None,
            (Some(l), None) => Some(l),
            (None, Some(r)) => Some(r),
            (Some(_), Some(r)) => Some(r), // Favor the right since it's closer
        }
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}

fn longest_axis(bbox: &Aabb) -> usize {
    let extents = bbox.extents(); // Or aabb.maxs - aabb.mins
    if extents.x > extents.y && extents.x > extents.z {
        0 // X-axis is longest
    } else if extents.y > extents.z {
        1 // Y-axis is longest
    } else {
        2 // Z-axis is longest
    }
}
