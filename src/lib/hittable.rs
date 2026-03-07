use crate::lib::aabb::AABB;
use crate::lib::interval::Interval;
use crate::lib::material::Material;
use crate::lib::ray::Ray;
use crate::lib::util::degrees_to_radians;
use crate::lib::vec3::Vec3;
use core::f64;
use std::sync::Arc;

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
    pub mat: Arc<dyn Material>,
}

impl HitRecord {
    pub fn new(
        point: Vec3,
        normal: Vec3,
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

    pub fn get_front_face(ray: &Ray, outward_normal: Vec3) -> (bool, Vec3) {
        let front_face = ray.dir.dot(&outward_normal) < 0.0;
        let face_normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        (front_face, face_normal)
    }

    pub fn set_point(&self, new_point: Vec3) -> Self {
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

    pub fn set_normal(&self, new_normal: Vec3) -> Self {
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
    fn bounding_box(&self) -> AABB;
}

pub struct Translate {
    object: Arc<dyn Hittable>,
    offset: Vec3,
    bbox: AABB,
}

impl Translate {
    pub fn new(object: Arc<dyn Hittable>, offset: Vec3) -> Self {
        Self {
            object: Arc::clone(&object),
            offset,
            bbox: object.bounding_box().offset(offset),
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

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

pub struct RotateY {
    object: Arc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: AABB,
}

impl RotateY {
    pub fn new(object: Arc<dyn Hittable>, angle: f64) -> Self {
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        let bbox = object.bounding_box();
        let mut min = Vec3::fill(f64::INFINITY);
        let mut max = Vec3::fill(-f64::INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let fi = i as f64;
                    let fj = j as f64;
                    let fk = k as f64;

                    let x = fi * bbox.x.max + (1.0 - fi) * bbox.x.min;
                    let y = fj * bbox.y.max + (1.0 - fj) * bbox.y.min;
                    let z = fk * bbox.z.max + (1.0 - fk) * bbox.z.min;

                    let new_x = cos_theta * x + sin_theta * z;
                    let new_z = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(new_x, y, new_z);

                    for c in 0..3 {
                        min[c] = f64::min(min[c], tester[c]);
                        max[c] = f64::max(max[c], tester[c]);
                    }
                }
            }
        }

        Self {
            object: Arc::clone(&object),
            sin_theta,
            cos_theta,
            bbox: AABB::from_vecs(min, max),
        }
    }

    fn rotate_y(vec: &Vec3, sin_theta: f64, cos_theta: f64) -> Vec3 {
        Vec3::new(
            cos_theta * vec.x() - sin_theta * vec.z(),
            vec.y(),
            sin_theta * vec.x() + cos_theta * vec.z(),
        )
    }
}

impl Hittable for RotateY {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let orig = Self::rotate_y(&ray.orig, self.sin_theta, self.cos_theta);
        let dir = Self::rotate_y(&ray.dir, self.sin_theta, self.cos_theta);
        let rotated_ray = Ray::new(orig, dir, ray.time);

        self.object.hit(&rotated_ray, ray_t).map(|hit_record| {
            let new_point = Self::rotate_y(&hit_record.point, -self.sin_theta, self.cos_theta);
            let new_normal = Self::rotate_y(&hit_record.normal, -self.sin_theta, self.cos_theta);
            hit_record.set_point(new_point).set_normal(new_normal)
        })
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

pub fn rotate_y(object: Arc<dyn Hittable>, angle: f64) -> Arc<dyn Hittable> {
    Arc::new(RotateY::new(object, angle))
}

pub fn translate(object: Arc<dyn Hittable>, offset: [f64; 3]) -> Arc<dyn Hittable> {
    Arc::new(Translate::new(object, Vec3::new_arr(offset)))
}
