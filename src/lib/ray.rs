use crate::lib::vec3::Vec3;

pub struct Ray {
  pub orig: Vec3,
  pub dir: Vec3,
}

impl Ray {
  pub fn new(orig: Vec3, dir: Vec3) -> Ray {
    Ray {
      orig: orig,
      dir: dir,
    }
  }
  
  pub fn at(&self, t: f64) -> Vec3 {
    return self.orig + self.dir.scale(t);
  }
}
