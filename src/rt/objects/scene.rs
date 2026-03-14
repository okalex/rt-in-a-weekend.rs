use std::sync::Arc;

use crate::rt::{
    interval::Interval,
    objects::{
        bvh_node::BvhNode,
        hittable::{HitRecord, Hittable},
        hittable_list::HittableList,
    },
    ray::Ray,
};

pub struct Scene {
    objects: Arc<BvhNode>,
    pub lights: Arc<HittableList>,
}

impl Scene {
    pub fn new(objects: HittableList, lights: HittableList) -> Self {
        Self {
            objects: Arc::new(BvhNode::from_list(objects)),
            lights: Arc::new(lights),
        }
    }

    pub fn no_lights(objects: HittableList) -> Self {
        Self {
            objects: Arc::new(BvhNode::from_list(objects)),
            lights: Arc::new(HittableList::new()),
        }
    }

    pub fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        self.objects.hit(ray, ray_t)
    }
}
