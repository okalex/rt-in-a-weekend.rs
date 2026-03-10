use nalgebra::{Point3, Vector3};

use crate::rt::interval::Interval;
use crate::rt::ray::Ray;

#[derive(Clone, Copy)]
pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl AABB {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self {
            x: Self::pad_to_min(x),
            y: Self::pad_to_min(y),
            z: Self::pad_to_min(z),
        }
    }

    pub fn empty() -> Self {
        // Don't use ::new to avoid the default padding
        Self {
            x: Interval::empty(),
            y: Interval::empty(),
            z: Interval::empty(),
        }
    }

    pub fn from_points(a: Point3<f64>, b: Point3<f64>) -> Self {
        Self::new(
            Interval::new(a.x, b.x),
            Interval::new(a.y, b.y),
            Interval::new(a.z, b.z),
        )
    }

    pub fn from_vectors(a: Vector3<f64>, b: Vector3<f64>) -> Self {
        Self::new(
            Interval::new(a.x, b.x),
            Interval::new(a.y, b.y),
            Interval::new(a.z, b.z),
        )
    }

    pub fn from_boxes(box0: &AABB, box1: &AABB) -> AABB {
        Self::new(
            Interval::union(&box0.x, &box1.x),
            Interval::union(&box0.y, &box1.y),
            Interval::union(&box0.z, &box1.z),
        )
    }

    fn pad_to_min(interval: Interval) -> Interval {
        const DELTA: f64 = 0.0001;
        if interval.size() < DELTA {
            interval.expand(DELTA)
        } else {
            interval
        }
    }

    pub fn axis_interval(&self, n: usize) -> &Interval {
        match n {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!(),
        }
    }

    pub fn longest_axis(&self) -> usize {
        if self.x.size() > self.y.size() {
            if self.x.size() > self.z.size() { 0 } else { 2 }
        } else {
            if self.y.size() > self.z.size() { 1 } else { 2 }
        }
    }

    pub fn hit(&self, ray: &Ray, ray_t: Interval) -> bool {
        let mut ray_t_copy = ray_t;

        for axis_idx in 0..3 {
            let axis = self.axis_interval(axis_idx);
            let ad_inv = 1.0 / ray.dir[axis_idx];

            let t0 = (axis.min - ray.orig[axis_idx]) * ad_inv;
            let t1 = (axis.max - ray.orig[axis_idx]) * ad_inv;

            if t0 < t1 {
                if t0 > ray_t_copy.min {
                    ray_t_copy.min = t0;
                }
                if t1 < ray_t_copy.max {
                    ray_t_copy.max = t1;
                }
            } else {
                if t1 > ray_t_copy.min {
                    ray_t_copy.min = t1;
                }
                if t0 < ray_t_copy.max {
                    ray_t_copy.max = t0;
                }
            }

            if ray_t_copy.max <= ray_t_copy.min {
                return false;
            }
        }

        true
    }

    pub fn offset(&self, amount: Vector3<f64>) -> Self {
        Self {
            x: self.x + amount.x,
            y: self.y + amount.y,
            z: self.z + amount.z,
        }
    }
}
