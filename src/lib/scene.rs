use std::sync::Arc;

use crate::lib::aabb::AABB;
use crate::lib::hittable::{HitRecord, Hittable};
use crate::lib::interval::Interval;
use crate::lib::material::Material;
use crate::lib::quad::Quad;
use crate::lib::ray::Ray;
use crate::lib::vec3::Vec3;

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
}

impl Hittable for Scene {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
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

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

pub struct Box3d {}

impl Box3d {
    pub fn new(a: Vec3, b: Vec3, mat: Arc<dyn Material>) -> Scene {
        let min = Vec3::min(a, b);
        let max = Vec3::max(a, b);

        let dx = Vec3::new(max.x() - min.x(), 0.0, 0.0);
        let dy = Vec3::new(0.0, max.y() - min.y(), 0.0);
        let dz = Vec3::new(0.0, 0.0, max.z() - min.z());

        let quad1: Arc<dyn Hittable> = Arc::new(Quad::new(
            Vec3::new(min.x(), min.y(), max.z()),
            dx,
            dy,
            Arc::clone(&mat),
        ));
        let quad2: Arc<dyn Hittable> = Arc::new(Quad::new(
            Vec3::new(max.x(), min.y(), max.z()),
            -dz,
            dy,
            Arc::clone(&mat),
        ));
        let quad3: Arc<dyn Hittable> = Arc::new(Quad::new(
            Vec3::new(max.x(), min.y(), min.z()),
            -dx,
            dy,
            Arc::clone(&mat),
        ));
        let quad4: Arc<dyn Hittable> = Arc::new(Quad::new(
            Vec3::new(min.x(), min.y(), min.z()),
            dz,
            dy,
            Arc::clone(&mat),
        ));
        let quad5: Arc<dyn Hittable> = Arc::new(Quad::new(
            Vec3::new(min.x(), max.y(), max.z()),
            dx,
            -dz,
            Arc::clone(&mat),
        ));
        let quad6: Arc<dyn Hittable> = Arc::new(Quad::new(
            Vec3::new(min.x(), min.y(), min.z()),
            dx,
            dz,
            Arc::clone(&mat),
        ));

        let mut sides = Scene::new();
        sides.add(Arc::clone(&quad1));
        sides.add(Arc::clone(&quad2));
        sides.add(Arc::clone(&quad3));
        sides.add(Arc::clone(&quad4));
        sides.add(Arc::clone(&quad5));
        sides.add(Arc::clone(&quad6));
        sides
    }
}
