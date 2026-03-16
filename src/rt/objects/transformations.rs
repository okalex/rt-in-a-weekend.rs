use parry3d_f64::{
    bounding_volume::Aabb,
    math::{Pose, Vec3},
};

use crate::rt::{
    interval::Interval,
    objects::{hit_record::HitRecord, hittable::Hittable},
    ray::Ray,
    types::{Float, Point, Vector, new_vec3},
    util::{degrees_to_radians, to_parry_vec},
};

pub struct Translate {
    object: Box<Hittable>,
    offset: Vector,
    bbox: Aabb,
}

impl Translate {
    pub fn new(object: Hittable, offset: Vector) -> Self {
        let bbox = object.bounding_box().translated(to_parry_vec(offset));
        Self {
            object: Box::new(object),
            offset,
            bbox,
        }
    }

    pub fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let offset_ray = Ray::new(ray.orig - self.offset, ray.dir, ray.time);

        self.object.hit(&offset_ray, ray_t).map(|hit_record| {
            let offset_point = hit_record.point + self.offset;
            hit_record.set_point(offset_point)
        })
    }

    pub fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}

pub struct RotateY {
    object: Box<Hittable>,
    sin_theta: Float,
    cos_theta: Float,
    bbox: Aabb,
}

impl RotateY {
    pub fn new(object: Hittable, angle: Float) -> Self {
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        let bbox = object.bounding_box().clone();
        let rotation = Pose::rotation(new_vec3([0.0, degrees_to_radians(angle), 0.0]));

        Self {
            object: Box::new(object),
            sin_theta,
            cos_theta,
            bbox: bbox.transform_by(&rotation),
        }
    }

    fn rotate_y(vec: &Vector, sin_theta: Float, cos_theta: Float) -> Vector {
        Vector::new(
            cos_theta * vec.x - sin_theta * vec.z,
            vec.y,
            sin_theta * vec.x + cos_theta * vec.z,
        )
    }

    pub fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let orig = Point::from(Self::rotate_y(
            &ray.orig.coords,
            self.sin_theta,
            self.cos_theta,
        ));
        let dir = Self::rotate_y(&ray.dir, self.sin_theta, self.cos_theta);
        let rotated_ray = Ray::new(orig, dir, ray.time);

        self.object.hit(&rotated_ray, ray_t).map(|hit_record| {
            let new_point = Point::from(Self::rotate_y(
                &hit_record.point.coords,
                -self.sin_theta,
                self.cos_theta,
            ));
            let new_normal = Self::rotate_y(&hit_record.normal, -self.sin_theta, self.cos_theta);
            hit_record.set_point(new_point).set_normal(new_normal)
        })
    }

    pub fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}

pub fn rotate_y(object: Hittable, degrees: Float) -> Hittable {
    Hittable::RotateY(RotateY::new(object, degrees))
}

pub fn translate(object: Hittable, offset: [Float; 3]) -> Hittable {
    Hittable::Translate(Translate::new(object, Vector::from(offset)))
}
