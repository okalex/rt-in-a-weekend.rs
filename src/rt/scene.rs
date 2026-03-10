use std::sync::Arc;

use nalgebra::{Point3, Vector3};
use nalgebra_glm::{max2, min2};

use crate::rt::aabb::AABB;
use crate::rt::hittable::{HitRecord, Hittable};
use crate::rt::interval::Interval;
use crate::rt::materials::material::Material;
use crate::rt::quad::Quad;
use crate::rt::ray::Ray;

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
    pub fn new(a: Vector3<f64>, b: Vector3<f64>, mat: Arc<dyn Material>) -> Scene {
        let min = min2(&a, &b);
        let max = max2(&a, &b);

        let dx = Vector3::new(max.x - min.x, 0.0, 0.0);
        let dy = Vector3::new(0.0, max.y - min.y, 0.0);
        let dz = Vector3::new(0.0, 0.0, max.z - min.z);

        let quad1: Arc<dyn Hittable> = Arc::new(Quad::new(
            Point3::new(min.x, min.y, max.z),
            dx,
            dy,
            Arc::clone(&mat),
        ));
        let quad2: Arc<dyn Hittable> = Arc::new(Quad::new(
            Point3::new(max.x, min.y, max.z),
            -dz,
            dy,
            Arc::clone(&mat),
        ));
        let quad3: Arc<dyn Hittable> = Arc::new(Quad::new(
            Point3::new(max.x, min.y, min.z),
            -dx,
            dy,
            Arc::clone(&mat),
        ));
        let quad4: Arc<dyn Hittable> = Arc::new(Quad::new(
            Point3::new(min.x, min.y, min.z),
            dz,
            dy,
            Arc::clone(&mat),
        ));
        let quad5: Arc<dyn Hittable> = Arc::new(Quad::new(
            Point3::new(min.x, max.y, max.z),
            dx,
            -dz,
            Arc::clone(&mat),
        ));
        let quad6: Arc<dyn Hittable> = Arc::new(Quad::new(
            Point3::new(min.x, min.y, min.z),
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
