use std::f64::consts::PI;
use std::sync::Arc;

use nalgebra::{Point3, Vector3};

use crate::rt::aabb::AABB;
use crate::rt::hittable::{HitRecord, Hittable};
use crate::rt::interval::Interval;
use crate::rt::materials::material::Material;
use crate::rt::ray::Ray;

pub struct Sphere {
    pub center: Ray,
    pub radius: f64,
    pub mat: Arc<dyn Material>,
    bbox: AABB,
}

impl Sphere {
    pub fn new(center: Ray, radius: f64, mat: Arc<dyn Material>, bbox: AABB) -> Sphere {
        Sphere {
            center,
            radius,
            mat,
            bbox,
        }
    }

    pub fn stationary(center: Point3<f64>, radius: f64, mat: Arc<dyn Material>) -> Sphere {
        let ray = Ray::new(center, Vector3::zeros(), 0.0);
        let rvec = Vector3::from_element(radius);
        let bbox = AABB::from_points(center - rvec, center + rvec);
        Self::new(ray, radius, mat, bbox)
    }

    pub fn moving(center1: Point3<f64>, center2: Point3<f64>, radius: f64, mat: Arc<dyn Material>) -> Sphere {
        let ray = Ray::new(center1, center2 - center1, 0.0);
        let rvec = Vector3::from_element(radius);
        let box1 = AABB::from_points(ray.at(0.0) - rvec, ray.at(0.0) + rvec);
        let box2 = AABB::from_points(ray.at(1.0) - rvec, ray.at(1.0) + rvec);
        let bbox = AABB::from_boxes(&box1, &box2);
        Self::new(ray, radius, mat, bbox)
    }

    pub fn get_uv(point: &Point3<f64>) -> (f64, f64) {
        let theta = (-point.y).acos();
        let phi = (-point.z).atan2(point.x) + PI;
        let u = phi / (2.0 * PI);
        let v = theta / PI;
        (u, v)
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let curr_center = self.center.at(ray.time);
        let oc = curr_center - ray.orig;
        let a = ray.dir.magnitude_squared();
        let h = ray.dir.dot(&oc);
        let c = oc.magnitude_squared() - self.radius * self.radius;
        let discriminant = (h * h) - (a * c);

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (h - sqrtd) / a;

        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let point = ray.at(root);
        let outward_normal = (point - curr_center) / self.radius;
        let (front_face, face_normal) = HitRecord::get_front_face(ray, outward_normal);
        let (u, v) = Sphere::get_uv(&Point3::from(face_normal)); // Why is this not &point?

        Some(HitRecord::new(
            point,
            face_normal,
            front_face,
            root,
            u,
            v,
            Arc::clone(&self.mat),
        ))
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
