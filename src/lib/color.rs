use crate::lib::vec3;

pub struct Color {
  base: vec3::Vec3,
}

impl Color {
  pub fn r(&self) -> u8 {
    return (255.0 * self.base.x()) as u8;
  }

  pub fn g(&self) -> u8 {
    return (255.0 * self.base.y()) as u8;
  }

  pub fn b(&self) -> u8 {
    return (255.0 * self.base.z()) as u8;
  }

  pub fn to_string(&self) -> String {
    return format!("{} {} {}", self.r(), self.g(), self.b());
  }
}

pub fn wrap_vec(vec: vec3::Vec3) -> Color {
  return Color {
    base: vec
  };
}

pub fn new_vec(r: f64, g: f64, b: f64) -> Color {
  return wrap_vec(
    vec3::new(r, g, b)
  );
}
