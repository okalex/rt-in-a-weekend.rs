use std::sync::Arc;

use crate::rt::{
    interval::Interval,
    materials::material::Material,
    objects::{bvh_node::BvhNode, hit_record::HitRecord, hittable_list::HittableList},
    ray::Ray,
};

pub struct Scene {
    pub objects: Arc<BvhNode>,
    pub materials: Vec<Material>,
    pub lights: Arc<HittableList>,
}

impl Scene {
    pub fn new(objects: HittableList, materials: Vec<Material>, lights: HittableList) -> Self {
        Self {
            objects: Arc::new(BvhNode::from(objects)),
            materials,
            lights: Arc::new(lights),
        }
    }

    pub fn no_lights(objects: HittableList, materials: Vec<Material>) -> Self {
        Self {
            objects: Arc::new(BvhNode::from(objects)),
            materials,
            lights: Arc::new(HittableList::new()),
        }
    }

    pub fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        self.objects.hit(ray, ray_t)
    }

    pub fn get_material(&self, mat_idx: usize) -> &Material {
        &self.materials[mat_idx]
    }
}
