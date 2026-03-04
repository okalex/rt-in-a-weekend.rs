use crate::lib::vec3;

pub struct Ray {
  orig: vec3::Vec3,
  dir: vec3::Vec3,
}

impl Ray {
  pub fn orig(&self) -> &vec3::Vec3 {
    return &self.orig;
  }

  pub fn dir(&self) -> &vec3::Vec3 {
    return &self.dir;
  }
  
  pub fn at(&self, t: f64) -> vec3::Vec3 {
    return self.orig + self.dir.scale(t);
  }
}

pub fn new(orig: vec3::Vec3, dir: vec3::Vec3) -> Ray {
  return Ray {
    orig: orig,
    dir: dir,
  };
}
