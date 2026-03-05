use crate::lib::vec3;

pub struct Ray {
  pub orig: vec3::Vec3,
  pub dir: vec3::Vec3,
}

impl Ray {
  pub fn new(orig: vec3::Vec3, dir: vec3::Vec3) -> Ray {
    Ray {
      orig: orig,
      dir: dir,
    }
  }
  
  pub fn at(&self, t: f64) -> vec3::Vec3 {
    return self.orig + self.dir.scale(t);
  }
}
