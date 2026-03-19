use std::{cell::Cell, sync::Arc};

use parry3d_f64::{
    bounding_volume::Aabb,
    partitioning::{Bvh, BvhBuildStrategy},
};

use crate::rt::{
    interval::Interval,
    materials::material::Material,
    objects::{hit_record::HitRecord, hittable::Hittable, hittable_list::HittableList},
    ray::Ray,
    types::{Float},
};

pub struct Scene {
    pub objects: Arc<Vec<Arc<Hittable>>>,
    pub bvh: Arc<Bvh>,
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

    fn build_bvh(objects: Arc<Vec<Arc<Hittable>>>) -> Bvh {
        // TODO: This needs to recurse into container objects (HittableList, RotateY, Translate, Mesh)
        let aabbs: Vec<Aabb> = objects.iter().map(|obj| *obj.bounding_box()).collect();
        Bvh::from_leaves(BvhBuildStrategy::Ploc, &aabbs)
    }

    pub fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let parry_ray = ray.to_parry3d();

        let current_hit: Cell<Option<HitRecord>> = Cell::new(None);

        #[rustfmt::skip]
        self.bvh.cast_ray(
            &parry_ray,
            ray_t.max as f64,
            |leaf_id, best_hit| {
                match self.objects.get(leaf_id as usize) {
                    Some(object) => {
                        match object.hit(ray, ray_t.update_max(best_hit as Float)) {
                            Some(hit_record) => {
                                if hit_record.t < best_hit as Float {
                                    let toi = hit_record.t;
                                    current_hit.set(Some(hit_record));
                                    Some(toi as f64)
                                } else {
                                    None
                                }
                            },
                            None => None,
                        }
                    }
                    None => None,
                }
            }
        );

        current_hit.into_inner()
    }

    pub fn get_material(&self, mat_idx: usize) -> &Material {
        &self.materials[mat_idx]
    }
}
