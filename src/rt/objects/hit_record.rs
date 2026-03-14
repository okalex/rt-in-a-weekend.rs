use nalgebra::{Point3, Vector3};

use crate::rt::ray::Ray;

pub struct HitRecord {
    pub point: Point3<f64>,
    pub normal: Vector3<f64>,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
    pub mat_idx: usize,
}

impl HitRecord {
    pub fn new(
        point: Point3<f64>,
        normal: Vector3<f64>,
        front_face: bool,
        t: f64,
        u: f64,
        v: f64,
        mat_idx: usize,
    ) -> Self {
        Self {
            point,
            normal,
            t,
            u,
            v,
            front_face,
            mat_idx,
        }
    }

    pub fn get_front_face(ray: &Ray, outward_normal: Vector3<f64>) -> (bool, Vector3<f64>) {
        let front_face = ray.dir.dot(&outward_normal) < 0.0;
        let face_normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        (front_face, face_normal)
    }

    pub fn set_point(&self, new_point: Point3<f64>) -> Self {
        Self::new(
            new_point,
            self.normal,
            self.front_face,
            self.t,
            self.u,
            self.v,
            self.mat_idx,
        )
    }

    pub fn set_normal(&self, new_normal: Vector3<f64>) -> Self {
        Self::new(
            self.point,
            new_normal,
            self.front_face,
            self.t,
            self.u,
            self.v,
            self.mat_idx,
        )
    }
}
