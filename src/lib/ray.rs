use nalgebra::{Point3, Vector3};

pub struct Ray {
    pub orig: Point3<f64>,
    pub dir: Vector3<f64>,
    pub time: f64,
}

impl Ray {
    pub fn new(orig: Point3<f64>, dir: Vector3<f64>, time: f64) -> Self {
        Self { orig, dir, time }
    }

    pub fn at(&self, t: f64) -> Point3<f64> {
        return self.orig + self.dir * t;
    }
}
