use nalgebra::{Point3, Vector3};
use parry3d_f64::bounding_volume::Aabb;

use crate::rt::interval::Interval;
use crate::rt::objects::bvh_node::BvhNode;
use crate::rt::objects::constant_medium::ConstantMedium;
use crate::rt::objects::hit_record::HitRecord;
use crate::rt::objects::hittable_list::HittableList;
use crate::rt::objects::mesh::Mesh;
use crate::rt::objects::quad::Quad;
use crate::rt::objects::sphere::Sphere;
use crate::rt::objects::transformations::{RotateY, Translate};
use crate::rt::objects::triangle::Triangle;
use crate::rt::ray::Ray;
use core::f64;



pub enum Hittable {
    BvhNode(BvhNode),
    ConstantMedium(ConstantMedium),
    HittableList(HittableList),
    Mesh(Mesh),
    Quad(Quad),
    RotateY(RotateY),
    Sphere(Sphere),
    Translate(Translate),
    Triangle(Triangle),
}

impl Hittable {
    pub fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        match self {
            Self::BvhNode(obj) => obj.hit(ray, ray_t),
            Self::ConstantMedium(obj) => obj.hit(ray, ray_t),
            Self::HittableList(obj) => obj.hit(ray, ray_t),
            Self::Mesh(obj) => obj.hit(ray, ray_t),
            Self::Quad(obj) => obj.hit(ray, ray_t),
            Self::RotateY(obj) => obj.hit(ray, ray_t),
            Self::Sphere(obj) => obj.hit(ray, ray_t),
            Self::Translate(obj) => obj.hit(ray, ray_t),
            Self::Triangle(obj) => obj.hit(ray, ray_t),
        }
    }

    pub fn bounding_box(&self) -> &Aabb {
        match self {
            Self::BvhNode(obj) => obj.bounding_box(),
            Self::ConstantMedium(obj) => obj.bounding_box(),
            Self::HittableList(obj) => obj.bounding_box(),
            Self::Mesh(obj) => obj.bounding_box(),
            Self::Quad(obj) => obj.bounding_box(),
            Self::RotateY(obj) => obj.bounding_box(),
            Self::Sphere(obj) => obj.bounding_box(),
            Self::Translate(obj) => obj.bounding_box(),
            Self::Triangle(obj) => obj.bounding_box(),
        }
    }

    pub fn pdf_value(&self, origin: &Point3<f64>, direction: &Vector3<f64>) -> f64 {
        let default = 0.0;
        match self {
            Self::BvhNode(_) => default,
            Self::ConstantMedium(_) => default,
            Self::HittableList(obj) => obj.pdf_value(origin, direction),
            Self::Mesh(_) => default,
            Self::Quad(obj) => obj.pdf_value(origin, direction),
            Self::RotateY(_) => default,
            Self::Sphere(obj) => obj.pdf_value(origin, direction),
            Self::Translate(_) => default,
            Self::Triangle(_) => default,
        }
    }

    pub fn random(&self, origin: &Point3<f64>) -> Vector3<f64> {
        let default = Vector3::new(1.0, 0.0, 0.0);
        match self {
            Self::BvhNode(_) => default,
            Self::ConstantMedium(_) => default,
            Self::HittableList(obj) => obj.random(origin),
            Self::Mesh(_) => default,
            Self::Quad(obj) => obj.random(origin),
            Self::RotateY(_) => default,
            Self::Sphere(obj) => obj.random(origin),
            Self::Translate(_) => default,
            Self::Triangle(_) => default,
        }
    }
}


