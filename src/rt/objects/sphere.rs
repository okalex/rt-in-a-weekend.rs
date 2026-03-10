use std::f64::consts::PI;
use std::sync::Arc;

use nalgebra::{Point3, Vector3};
use parry3d_f64::bounding_volume::{Aabb, BoundingVolume};

use super::hittable::{HitRecord, Hittable};
use crate::rt::interval::Interval;
use crate::rt::materials::material::Material;
use crate::rt::ray::Ray;
use crate::rt::util::to_parry_vec;

pub struct Sphere {
    pub center: Ray,
    pub radius: f64,
    pub mat: Arc<dyn Material>,
    bbox: Aabb,
}

impl Sphere {
    pub fn new(center: Ray, radius: f64, mat: Arc<dyn Material>, bbox: Aabb) -> Sphere {
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
        let points = vec![
            to_parry_vec((center - rvec).coords),
            to_parry_vec((center + rvec).coords),
        ];
        let bbox = Aabb::from_points(points);
        Self::new(ray, radius, mat, bbox)
    }

    pub fn moving(
        center1: Point3<f64>,
        center2: Point3<f64>,
        radius: f64,
        mat: Arc<dyn Material>,
    ) -> Sphere {
        let ray = Ray::new(center1, center2 - center1, 0.0);
        let rvec = Vector3::from_element(radius);
        let box1 = {
            let points = vec![
                to_parry_vec((ray.at(0.0) - rvec).coords),
                to_parry_vec((ray.at(0.0) + rvec).coords),
            ];
            Aabb::from_points(points)
        };
        let box2 = {
            let points = vec![
                to_parry_vec((ray.at(1.0) - rvec).coords),
                to_parry_vec((ray.at(1.0) + rvec).coords),
            ];
            Aabb::from_points(points)
        };
        let bbox = box1.merged(&box2);
        Self::new(ray, radius, mat, bbox)
    }

    pub fn get_uv(normal: &Vector3<f64>) -> (f64, f64) {
        let u = 0.5 + (-normal.z).atan2(normal.x) / (2.0 * PI);
        let v = 0.5 + normal.y.asin() / PI;
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
        let (u, v) = Sphere::get_uv(&face_normal); // Why is this not &point?

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

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
