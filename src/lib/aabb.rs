use crate::lib::interval::Interval;
use crate::lib::ray::Ray;
use crate::lib::vec3::Vec3;

#[derive(Clone, Copy)]
pub struct AABB {
  pub x: Interval,
  pub y: Interval,
  pub z: Interval,
}

impl AABB {

  pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
    Self { x, y, z }
  }

  pub fn empty() -> Self {
    Self::new(
      Interval::empty(),
      Interval::empty(),
      Interval::empty(),
    )
  }

  pub fn from_vecs(a: Vec3, b: Vec3) -> Self {
    Self::new(
      if a.x() <= b.x() { Interval::new(a.x(), b.x()) } else { Interval::new(b.x(), a.x()) },
      if a.y() <= b.y() { Interval::new(a.y(), b.y()) } else { Interval::new(b.y(), a.y()) },
      if a.z() <= b.z() { Interval::new(a.z(), b.z()) } else { Interval::new(b.z(), a.z()) },
    )
  }

  pub fn from_boxes(box0: &AABB, box1: &AABB) -> AABB {
    Self::new(
      Interval::union(&box0.x, &box1.x),
      Interval::union(&box0.y, &box1.y),
      Interval::union(&box0.z, &box1.z),
    )
  }

  pub fn axis_interval(&self, n: usize) -> &Interval {
    if n == 1 { return &self.y; }
    if n == 2 { return &self.z; }
    return &self.x;
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
        if t0 > ray_t_copy.min { ray_t_copy.min = t0; }
        if t1 < ray_t_copy.max { ray_t_copy.max = t1; }
      } else {
        if t1 > ray_t_copy.min { ray_t_copy.min = t1; }
        if t0 < ray_t_copy.max { ray_t_copy.max = t0; }
      }

      if ray_t_copy.max <= ray_t_copy.min {
        return false;
      }
    }

    return true;
  }

}
