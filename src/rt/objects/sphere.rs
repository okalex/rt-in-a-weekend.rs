use core::f64;
use std::f64::consts::PI;
use std::sync::Arc;

use nalgebra::{Point3, Vector3};
use parry3d_f64::bounding_volume::{Aabb, BoundingVolume};

use super::hittable::{HitRecord, Hittable};
use crate::rt::interval::Interval;
use crate::rt::materials::material::Material;
use crate::rt::onb::Onb;
use crate::rt::random::rand;
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

    fn rand_to_sphere(radius: f64, dist_sqrd: f64) -> Vector3<f64> {
        let r1 = rand();
        let r2 = rand();
        let z = 1.0 + r2 * ((1.0 - radius * radius / dist_sqrd).sqrt() - 1.0);

        let phi = 2.0 * PI * r1;
        let tmp = (1.0 - z * z).sqrt();
        let x = phi.cos() * tmp;
        let y = phi.sin() * tmp;

        Vector3::new(x, y, z)
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

    fn pdf_value(&self, origin: &Point3<f64>, direction: &Vector3<f64>) -> f64 {
        // This method only works for stationary spheres.

        let ray = Ray::new(*origin, *direction, 0.0);
        let interval = Interval::new(0.001, f64::INFINITY);
        match self.hit(&ray, interval) {
            None => 0.0,
            Some(_) => {
                let dist_sqrd = (self.center.at(0.0) - origin).magnitude_squared();
                let cos_theta_max = (1.0 - self.radius * self.radius / dist_sqrd).sqrt();
                let solid_angle = 2.0 * PI * (1.0 - cos_theta_max);
                1.0 / solid_angle
            }
        }
    }

    fn random(&self, origin: &Point3<f64>) -> Vector3<f64> {
        let direction = self.center.at(0.0) - origin;
        let dist_sqrd = direction.magnitude_squared();
        let uvw = Onb::new(&direction);
        uvw.transform(Self::rand_to_sphere(self.radius, dist_sqrd))
    }
}
