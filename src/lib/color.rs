use std::ops::{Add, Sub, Mul};
use crate::lib::{vec3, interval, random};

#[derive(Clone, Copy)]
pub struct Color {
  base: vec3::Vec3,
}

impl Color {
  pub fn base(&self) -> &vec3::Vec3 {
    return &self.base;
  }

  pub fn r(&self) -> f64 {
    return self.base.x();
  }

  pub fn g(&self) -> f64 {
    return self.base.y();
  }

  pub fn b(&self) -> f64 {
    return self.base.z();
  }

  pub fn to_gamma(&self) -> Color {
    return new_vec(
      linear_to_gamma(self.r()),
      linear_to_gamma(self.g()),
      linear_to_gamma(self.b()),
    );
  }

  pub fn scale(&self, scale_factor: f64) -> Color {
    return Color {
      base: self.base.scale(scale_factor),
    };
  }

  pub fn to_string(&self) -> String {
    let color = self.to_u8();
    return format!("{} {} {} ", color[0], color[1], color[2]);
  }

  pub fn to_u8(&self) -> [u8; 3] {
    return [
      to_u8(self.r()),
      to_u8(self.g()),
      to_u8(self.b()),
    ];
  }

}

impl Add for Color {
  type Output = Self;

  fn add(self, b: Self) -> Self {
    return wrap_vec(*self.base() + *b.base());
  }
}

impl Sub for Color {
  type Output = Self;

  fn sub(self, b: Self) -> Self {
    return wrap_vec(*self.base() - *b.base());
  }
}

impl Mul for Color {
  type Output = Self;

  fn mul(self, b: Self) -> Self {
    return wrap_vec(*self.base() * *b.base());
  }
}

pub fn wrap_vec(vec: vec3::Vec3) -> Color {
  return Color {
    base: vec
  };
}

pub fn new_vec(r: f64, g: f64, b: f64) -> Color {
  return wrap_vec(vec3::new(r, g, b));
}

pub fn rand() -> Color {
  return new_vec(random::rand(), random::rand(), random::rand());
}

pub fn to_u8(real: f64) -> u8 {
  let intensity = interval::new(0.0, 0.999);
  return (256.0 * intensity.clamp(real)) as u8;
}

fn linear_to_gamma(linear: f64) -> f64 {
  if linear > 0.0 {
    return linear.sqrt();
  }
  return 0.0;
}
