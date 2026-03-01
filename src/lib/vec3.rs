use std::ops::{Add, Div, Mul, Neg, Sub};

pub type Float = f64; // Change to f32 for less precision/better performance

pub struct Vec3 {
  e: [Float; 3],
}

impl Vec3 {
  pub fn to_string(&self) -> String {
    return format!("{} {} {}", self.x(), self.y(), self.z());
  }

  pub fn x(&self) -> Float {
    return self.e[0];
  }

  pub fn y(&self) -> Float {
    return self.e[1];
  }

  pub fn z(&self) -> Float {
    return self.e[2];
  }

  pub fn sum(&self) -> Float {
    return self.e[0] + self.e[1] + self.e[2];
  }

  pub fn square(&self) -> Vec3 {
    return new(self.e[0] * self.e[0], self.e[1] * self.e[1], self.e[2] * self.e[2]);
  }

  pub fn length_squared(&self) -> Float {
    return self.square().sum();
  }

  pub fn length(&self) -> Float {
    return self.length_squared().sqrt();
  }

  pub fn dot(&self, b: &Vec3) -> Float {
    return self.e[0] * b.e[0] + self.e[1] * b.e[1] + self.e[2] * b.e[2];
  }

  pub fn cross(&self, b: &Vec3) -> Vec3 {
    return new(
      self.e[1] * b.e[2] - self.e[2] * b.e[1],
      self.e[2] * b.e[0] - self.e[0] * b.e[2],
      self.e[0] * b.e[1] - self.e[1] * b.e[0]
    );
  }
}

// impl Index<usize> for Vec3 {
//   type Output = Float;

//   fn index(&self, idx: usize) -> &Float {
//     return &self[idx];
//   }
// }

impl Neg for Vec3 {
  type Output = Self;

  fn neg(self) -> Self {
    return new(-self.x(), -self.y(), -self.z());
  }
}

impl Add for Vec3 {
  type Output = Self;

  fn add(self, b: Self) -> Self {
    return new(self.x() + b.x(), self.y() + b.y(), self.z() + b.z());
  }
}

impl Sub for Vec3 {
  type Output = Self;

  fn sub(self, b: Self) -> Self {
    return new(self.x() - b.x(), self.y() - b.y(), self.z() - b.z());
  }
}

impl Mul for Vec3 {
  type Output = Self;

  fn mul(self, b: Self) -> Self {
    return new(self.x() * b.x(), self.y() * b.y(), self.z() * b.z());
  }
}

impl Div for Vec3 {
  type Output = Self;

  fn div(self, b: Self) -> Self {
    return new(self.x() / b.x(), self.y() / b.y(), self.z() / b.z());
  }
}

pub fn new(x: Float, y: Float, z: Float) -> Vec3 {
  return Vec3 {
    e: [x, y, z],
  };
}

pub fn fill(val: Float) -> Vec3 {
  return new(val, val, val);
}

pub fn zeroes() -> Vec3 {
  return fill(0.0);
}

pub fn ones() -> Vec3 {
  return fill(1.0);
}
