use crate::lib::random;
use std::ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub};

#[derive(Clone, Copy)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }

    pub fn new_arr(e: [f64; 3]) -> Vec3 {
        Vec3 { e }
    }

    pub fn fill(val: f64) -> Vec3 {
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
                return p / lensq.sqrt();
            }
        }
    }

    pub fn rand_on_hemisphere(normal: &Vec3) -> Vec3 {
        let on_unit_sphere = Self::rand_unit();
        if on_unit_sphere.is_on_hemisphere(normal) {
            on_unit_sphere
        } else {
            -on_unit_sphere
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

    pub fn min(a: Vec3, b: Vec3) -> Vec3 {
        Vec3::new(
            f64::min(a.x(), b.x()),
            f64::min(a.y(), b.y()),
            f64::min(a.z(), b.z()),
        )
    }

    pub fn max(a: Vec3, b: Vec3) -> Vec3 {
        Vec3::new(
            f64::max(a.x(), b.x()),
            f64::max(a.y(), b.y()),
            f64::max(a.z(), b.z()),
        )
    }

    pub fn x(&self) -> f64 {
        self[0]
    }

    pub fn y(&self) -> f64 {
        self[1]
    }

    pub fn z(&self) -> f64 {
        self[2]
    }

    pub fn sum(&self) -> f64 {
        self[0] + self[1] + self[2]
    }

    pub fn square(&self) -> Vec3 {
        *self * *self
    }

    pub fn length_squared(&self) -> f64 {
        self.square().sum()
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, b: &Vec3) -> f64 {
        self.e[0] * b.e[0] + self.e[1] * b.e[1] + self.e[2] * b.e[2]
    }

    pub fn cross(&self, b: &Vec3) -> Vec3 {
        Self::new(
            self.e[1] * b.e[2] - self.e[2] * b.e[1],
            self.e[2] * b.e[0] - self.e[0] * b.e[2],
            self.e[0] * b.e[1] - self.e[1] * b.e[0],
        )
    }

    pub fn inverse(&self) -> Vec3 {
        Self::new(1.0 / self.e[0], 1.0 / self.e[1], 1.0 / self.e[2])
    }

    pub fn unit(&self) -> Vec3 {
        *self / self.length()
    }

    pub fn is_on_hemisphere(&self, normal: &Vec3) -> bool {
        self.dot(normal) > 0.0
    }

    pub fn all_are_less_than(&self, limit: f64) -> bool {
        (self.e[0].abs() < limit) && (self.e[1].abs() < limit) && (self.e[2].abs() < limit)
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, idx: usize) -> &f64 {
        &self.e[idx]
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
        Vec3::new(-self.x(), -self.y(), -self.z())
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, b: Self) -> Self {
        Vec3::new(self.x() + b.x(), self.y() + b.y(), self.z() + b.z())
    }
}

impl Add<f64> for Vec3 {
    type Output = Self;

    fn add(self, b: f64) -> Self {
        self + Vec3::fill(b)
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, b: Self) -> Self {
        Vec3::new(self.x() - b.x(), self.y() - b.y(), self.z() - b.z())
    }
}

impl Sub<f64> for Vec3 {
    type Output = Self;

    fn sub(self, b: f64) -> Self {
        self - Vec3::fill(b)
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, b: Self) -> Self {
        Vec3::new(self.x() * b.x(), self.y() * b.y(), self.z() * b.z())
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, b: f64) -> Self {
        self * Vec3::fill(b)
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, b: Self) -> Self {
        Vec3::new(self.x() / b.x(), self.y() / b.y(), self.z() / b.z())
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, b: f64) -> Self {
        self / Vec3::fill(b)
    }
}
