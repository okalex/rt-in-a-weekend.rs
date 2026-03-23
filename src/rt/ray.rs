use glam::{
    Mat4,
    Vec3A,
};

use crate::util::{
    interval::Interval,
    types::{
        Float,
        Point,
        Vector,
    },
};

#[derive(Clone)]
pub struct Ray {
    pub orig: Point,
    pub dir: Vector,
    pub time: Float,
}

impl Ray {
    pub fn new(orig: Point, dir: Vector, time: Float) -> Self {
        Self { orig, dir, time }
    }

    pub fn at(&self, t: Float) -> Point {
        return self.orig + self.dir * t;
    }

    pub fn transform(&self, transform: Mat4) -> Self {
        let orig = transform.transform_point3(self.orig);
        let dir = transform.transform_vector3(self.dir);
        Self::new(orig, dir, self.time)
    }

    pub fn to_obvhs(&self, ray_t: Interval) -> obvhs::ray::Ray {
        obvhs::ray::Ray::new(Vec3A::from(self.orig), Vec3A::from(self.dir), ray_t.min, ray_t.max)
    }
}
