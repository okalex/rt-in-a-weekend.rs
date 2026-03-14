use std::sync::Arc;

use nalgebra::{Point3, Vector3};
use nalgebra_glm::{max2, min2};
use parry3d_f64::bounding_volume::{Aabb, BoundingVolume};

use super::hittable::{HitRecord, Hittable};
use super::quad::Quad;
use crate::rt::interval::Interval;
use crate::rt::materials::material::Material;
use crate::rt::random::rand_int;
use crate::rt::ray::Ray;

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
    pub bbox: Aabb,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: vec![],
            bbox: Aabb::new_invalid(),
        }
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.bbox = self.bbox.merged(object.bounding_box());
        self.objects.push(object);
    }

    pub fn len(&self) -> usize {
        self.objects.len()
    }
}

impl Hittable for HittableList {
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

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }

    #[allow(unused)]
    fn pdf_value(&self, origin: &Point3<f64>, direction: &Vector3<f64>) -> f64 {
        let weight = 1.0 / self.objects.len() as f64;
        let mut sum = 0.0;
        for object in &self.objects {
            sum += weight * object.pdf_value(origin, direction);
        }

        sum
    }

    #[allow(unused)]
    fn random(&self, origin: &Point3<f64>) -> Vector3<f64> {
        let int_size = self.objects.len() as i32;

        self.objects[rand_int(0, int_size - 1) as usize].random(origin)
    }
}

pub struct Box3d {}

impl Box3d {
    pub fn new(a: Vector3<f64>, b: Vector3<f64>, mat: Arc<dyn Material>) -> HittableList {
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

        let mut scene = HittableList::new();
        scene.add(Arc::clone(&quad1));
        scene.add(Arc::clone(&quad2));
        scene.add(Arc::clone(&quad3));
        scene.add(Arc::clone(&quad4));
        scene.add(Arc::clone(&quad5));
        scene.add(Arc::clone(&quad6));
        scene
    }
}
