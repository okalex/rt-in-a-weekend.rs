use crate::lib::random;
use std::ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub};

pub type Float = f64; // Change to f32 for less precision/better performance

#[derive(Clone, Copy)]
pub struct Vec3 {
    e: [Float; 3],
}

impl Vec3 {
    pub fn new(x: Float, y: Float, z: Float) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }

    pub fn new_arr(e: [f64; 3]) -> Vec3 {
        Vec3 { e }
    }

    pub fn fill(val: Float) -> Vec3 {
        Self::new(val, val, val)
    }

    pub fn zeroes() -> Vec3 {
        Self::fill(0.0)
    }

    pub fn ones() -> Vec3 {
        Self::fill(1.0)
    }

    pub fn rand() -> Vec3 {
        Self::new(random::rand(), random::rand(), random::rand())
    }

    pub fn rand_range(min: f64, max: f64) -> Vec3 {
        Self::new(
            random::rand_range(min, max),
            random::rand_range(min, max),
            random::rand_range(min, max),
        )
    }

    pub fn rand_unit() -> Vec3 {
        loop {
            let p = Self::rand_range(-1.0, 1.0);
            let lensq = p.length_squared();
            if 1e-160 < lensq && lensq <= 1.0 {
                return p.scale(1.0 / lensq.sqrt());
            }
        }
    }

    pub fn rand_on_hemisphere(normal: &Vec3) -> Vec3 {
        let on_unit_sphere = Self::rand_unit();
        if on_unit_sphere.is_on_hemisphere(normal) {
            return on_unit_sphere;
        } else {
            return -on_unit_sphere;
        }
    }

    pub fn rand_in_unit_disk() -> Vec3 {
        loop {
            let p = Self::new(
                random::rand_range(-1.0, 1.0),
                random::rand_range(-1.0, 1.0),
                0.0,
            );
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

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
        return Self::new(
            self.e[0] * self.e[0],
            self.e[1] * self.e[1],
            self.e[2] * self.e[2],
        );
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
        Self::new(
            self.e[1] * b.e[2] - self.e[2] * b.e[1],
            self.e[2] * b.e[0] - self.e[0] * b.e[2],
            self.e[0] * b.e[1] - self.e[1] * b.e[0],
        )
    }

    pub fn scale(&self, b: Float) -> Vec3 {
        Self::new(self.e[0] * b, self.e[1] * b, self.e[2] * b)
    }

    pub fn inverse(&self) -> Vec3 {
        Self::new(1.0 / self.e[0], 1.0 / self.e[1], 1.0 / self.e[2])
    }

    pub fn unit(&self) -> Vec3 {
        return self.scale(1.0 / self.length());
    }

    pub fn is_on_hemisphere(&self, normal: &Vec3) -> bool {
        return self.dot(normal) > 0.0;
    }

    pub fn near_zero(&self) -> bool {
        let limit = 1e-8;
        return (self.e[0].abs() < limit) && (self.e[1].abs() < limit) && (self.e[2].abs() < limit);
    }

    pub fn reflect(&self, normal: &Vec3) -> Vec3 {
        return *self - normal.scale(self.dot(normal)).scale(2.0);
    }

    pub fn refract(&self, n: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = f64::min(-self.dot(n), 1.0);
        let r_out_perp = (*self + n.scale(cos_theta)).scale(etai_over_etat);
        let r_out_parallel = n.scale(-(1.0 - r_out_perp.length_squared()).abs().sqrt());
        return r_out_perp + r_out_parallel;
    }

    pub fn rotate_y(&self, sin_theta: f64, cos_theta: f64) -> Vec3 {
        Self::new(
            cos_theta * self.x() - sin_theta * self.z(),
            self.y(),
            sin_theta * self.x() + cos_theta * self.z(),
        )
    }
}

impl Index<usize> for Vec3 {
    type Output = Float;

    fn index(&self, idx: usize) -> &Float {
        return &self.e[idx];
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        return Vec3::new(-self.x(), -self.y(), -self.z());
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, b: Self) -> Self {
        return Vec3::new(self.x() + b.x(), self.y() + b.y(), self.z() + b.z());
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, b: Self) -> Self {
        return Vec3::new(self.x() - b.x(), self.y() - b.y(), self.z() - b.z());
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, b: Self) -> Self {
        return Vec3::new(self.x() * b.x(), self.y() * b.y(), self.z() * b.z());
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, b: Self) -> Self {
        return Vec3::new(self.x() / b.x(), self.y() / b.y(), self.z() / b.z());
    }
}
