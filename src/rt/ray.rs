use crate::rt::types::{Float, Point, Vector, to_parry_vec};

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

    pub fn to_parry3d(&self) -> parry3d_f64::query::Ray {
        parry3d_f64::query::Ray::new(to_parry_vec(self.orig), to_parry_vec(self.dir))
    }
}
