use glam::Vec3A;
use obvhs::Boundable;

use crate::{
    rt::{
        geometry::{
            constant_medium::ConstantMedium,
            hit_record::HitRecord,
            hittable_list::HittableList,
            mesh::Mesh,
            quad::Quad,
            sphere::Sphere,
            triangle::Triangle,
        },
        ray::Ray,
    },
    util::interval::Interval,
};

pub enum Hittable {
    ConstantMedium(ConstantMedium),
    HittableList(HittableList),
    Mesh(Mesh),
    Quad(Quad),
    Sphere(Sphere),
    Triangle(Triangle),
}

impl Hittable {
    pub fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        match self {
            Self::ConstantMedium(obj) => obj.hit(ray, ray_t),
            Self::HittableList(obj) => obj.hit(ray, ray_t),
            Self::Mesh(obj) => obj.hit(ray, ray_t),
            Self::Quad(obj) => obj.hit(ray, ray_t),
            Self::Sphere(obj) => obj.hit(ray, ray_t),
            Self::Triangle(obj) => obj.hit(ray, ray_t),
        }
    }

    pub fn bounding_box(&self) -> &parry3d_f64::bounding_volume::Aabb {
        match self {
            Self::ConstantMedium(obj) => obj.bounding_box(),
            Self::HittableList(obj) => obj.bounding_box(),
            Self::Mesh(obj) => obj.bounding_box(),
            Self::Quad(obj) => obj.bounding_box(),
            Self::Sphere(obj) => obj.bounding_box(),
            Self::Triangle(obj) => obj.bounding_box(),
        }
    }
}

impl Boundable for Hittable {
    fn aabb(&self) -> obvhs::aabb::Aabb {
        let parry_aabb = self.bounding_box();
        obvhs_aabb_from(parry_aabb)
    }
}

pub fn obvhs_aabb_from(parry_aabb: &parry3d_f64::bounding_volume::Aabb) -> obvhs::aabb::Aabb {
    let min = vec3a_from(parry_aabb.mins);
    let max = vec3a_from(parry_aabb.maxs);
    obvhs::aabb::Aabb::new(min, max)
}

fn vec3a_from(dvec3: parry3d_f64::glamx::DVec3) -> Vec3A {
    Vec3A::new(dvec3.x as f32, dvec3.y as f32, dvec3.z as f32)
}
