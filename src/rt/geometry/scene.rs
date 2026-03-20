use std::{sync::Arc, time::Duration};

use glam::Vec3A;
use obvhs::{
    Boundable, BvhBuildParams,
    bvh2::{Bvh2, builder::build_bvh2},
    ray::RayHit,
};

use crate::rt::{
    interval::Interval,
    materials::material::Material,
    geometry::{hit_record::HitRecord, hittable::Hittable, hittable_list::HittableList},
    ray::Ray,
    types::INFINITY,
};

pub struct Scene {
    pub objects: Arc<Vec<Arc<Hittable>>>,
    pub bvh: Arc<Bvh2>,
    pub materials: Vec<Material>,
    pub lights: Arc<HittableList>,
}

impl Scene {
    pub fn new(objects: HittableList, materials: Vec<Material>, lights: HittableList) -> Self {
        let objs = Arc::new(objects.objects);
        let bvh = Arc::new(Self::build_bvh(Arc::clone(&objs)));
        Self {
            objects: Arc::clone(&objs),
            bvh,
            materials,
            lights: Arc::new(lights),
        }
    }

    pub fn no_lights(objects: HittableList, materials: Vec<Material>) -> Self {
        Self::new(objects, materials, HittableList::new())
    }

    fn build_bvh(objects: Arc<Vec<Arc<Hittable>>>) -> Bvh2 {
        // TODO: This needs to recurse into container objects (HittableList, RotateY, Translate, Mesh)
        let mut build_time = Duration::default();
        let aabbs: Vec<obvhs::aabb::Aabb> = objects.iter().map(|o| o.aabb()).collect();
        build_bvh2(&aabbs, BvhBuildParams::fastest_build(), &mut build_time)
    }

    pub fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let obvhs_ray = obvhs::ray::Ray::new(
            Vec3A::from(ray.orig),
            Vec3A::from(ray.dir),
            ray_t.min,
            ray_t.max,
        );
        let mut rec = RayHit::none();
        let mut hit_record: Option<HitRecord> = None;
        let mut closest_t = ray_t.max;

        self.bvh.ray_traverse(obvhs_ray, &mut rec, |_r, prim_idx| {
            let obj_idx = self.bvh.primitive_indices[prim_idx] as usize;
            let object = &self.objects[obj_idx];
            match object.hit(ray, ray_t.update_max(closest_t)) {
                Some(hit) => {
                    closest_t = hit.t;
                    hit_record = Some(hit);
                    closest_t
                }
                None => INFINITY,
            }
        });

        hit_record
    }

    pub fn get_material(&self, mat_idx: usize) -> &Material {
        &self.materials[mat_idx]
    }
}
