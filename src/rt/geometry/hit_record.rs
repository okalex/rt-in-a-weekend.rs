use crate::{
    rt::ray::Ray,
    util::types::{
        Float,
        Point,
        Vector,
    },
};

pub struct HitRecord {
    pub point: Point,
    pub normal: Vector,
    pub t: Float,
    pub u: Float,
    pub v: Float,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(point: Point, normal: Vector, front_face: bool, t: Float, u: Float, v: Float) -> Self {
        Self {
            point,
            normal,
            t,
            u,
            v,
            front_face,
        }
    }

    pub fn get_front_face(ray: &Ray, outward_normal: Vector) -> (bool, Vector) {
        let front_face = ray.dir.dot(outward_normal) < 0.0;
        let face_normal = if front_face { outward_normal } else { -outward_normal };
        (front_face, face_normal)
    }
}
