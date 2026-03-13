use nalgebra::{Point3, Vector3};
use parry3d_f64::bounding_volume::Aabb;
use parry3d_f64::math::{Pose, Vec3};

use crate::rt::interval::Interval;
use crate::rt::materials::material::Material;
use crate::rt::ray::Ray;
use crate::rt::util::{degrees_to_radians, to_parry_vec};
use core::f64;
use std::sync::Arc;

pub struct HitRecord {
    pub point: Point3<f64>,
    pub normal: Vector3<f64>,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
    pub mat: Arc<dyn Material>,
}

impl HitRecord {
    pub fn new(
        point: Point3<f64>,
        normal: Vector3<f64>,
        front_face: bool,
        t: f64,
        u: f64,
        v: f64,
        mat: Arc<dyn Material>,
    ) -> Self {
        Self {
            point,
            normal,
            t,
            u,
            v,
            front_face,
            mat,
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
            Arc::clone(&self.mat),
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
            Arc::clone(&self.mat),
        )
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord>;
    fn bounding_box(&self) -> &Aabb;

    #[allow(unused)]
    fn pdf_value(&self, origin: &Point3<f64>, direction: &Vector3<f64>) -> f64 {
        0.0
    }

    #[allow(unused)]
    fn random(&self, origin: &Point3<f64>) -> Vector3<f64> {
        Vector3::new(1.0, 0.0, 0.0)
    }
}

pub struct Translate {
    object: Arc<dyn Hittable>,
    offset: Vector3<f64>,
    bbox: Aabb,
}

impl Translate {
    pub fn new(object: Arc<dyn Hittable>, offset: Vector3<f64>) -> Self {
        Self {
            object: Arc::clone(&object),
            offset,
            bbox: object.bounding_box().translated(to_parry_vec(offset)),
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let offset_ray = Ray::new(ray.orig - self.offset, ray.dir, ray.time);

        self.object.hit(&offset_ray, ray_t).map(|hit_record| {
            let offset_point = hit_record.point + self.offset;
            hit_record.set_point(offset_point)
        })
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}

pub struct RotateY {
    object: Arc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Aabb,
}

impl RotateY {
    pub fn new(object: Arc<dyn Hittable>, angle: f64) -> Self {
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        let bbox = object.bounding_box();
        let rotation = Pose::rotation(Vec3::new(0.0, degrees_to_radians(angle), 0.0));

        Self {
            object: Arc::clone(&object),
            sin_theta,
            cos_theta,
            bbox: bbox.transform_by(&rotation),
        }
    }

    fn rotate_y(vec: &Vector3<f64>, sin_theta: f64, cos_theta: f64) -> Vector3<f64> {
        Vector3::new(
            cos_theta * vec.x - sin_theta * vec.z,
            vec.y,
            sin_theta * vec.x + cos_theta * vec.z,
        )
    }
}

impl Hittable for RotateY {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let orig = Point3::from(Self::rotate_y(
            &ray.orig.coords,
            self.sin_theta,
            self.cos_theta,
        ));
        let dir = Self::rotate_y(&ray.dir, self.sin_theta, self.cos_theta);
        let rotated_ray = Ray::new(orig, dir, ray.time);

        self.object.hit(&rotated_ray, ray_t).map(|hit_record| {
            let new_point = Point3::from(Self::rotate_y(
                &hit_record.point.coords,
                -self.sin_theta,
                self.cos_theta,
            ));
            let new_normal = Self::rotate_y(&hit_record.normal, -self.sin_theta, self.cos_theta);
            hit_record.set_point(new_point).set_normal(new_normal)
        })
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}

pub fn rotate_y(object: Arc<dyn Hittable>, degrees: f64) -> Arc<dyn Hittable> {
    Arc::new(RotateY::new(object, degrees))
}

pub fn translate(object: Arc<dyn Hittable>, offset: [f64; 3]) -> Arc<dyn Hittable> {
    Arc::new(Translate::new(object, Vector3::from(offset)))
}
