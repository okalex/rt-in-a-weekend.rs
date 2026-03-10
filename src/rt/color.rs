use nalgebra::Vector3;

use crate::rt::interval::Interval;
use crate::rt::random::rand;
use std::ops::{Add, Div, Mul, Sub};

type Base = Vector3<f64>;

#[derive(Clone, Copy)]
pub struct Color {
    base: Vector3<f64>,
}

impl Color {
    pub fn wrap_vec(base: Base) -> Color {
        Color { base }
    }

    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Self::wrap_vec(Base::new(r, g, b))
    }

    pub fn from_arr(values: [f64; 3]) -> Color {
        Self::wrap_vec(Base::from(values))
    }

    #[allow(dead_code)]
    pub fn from_u8(values: [u8; 3]) -> Color {
        Self::new(from_u8(values[0]), from_u8(values[1]), from_u8(values[2]))
    }

    #[allow(dead_code)]
    pub fn rand() -> Color {
        Self::new(rand(), rand(), rand())
    }

    pub fn fill(c: f64) -> Color {
        Self::new(c, c, c)
    }

    pub fn black() -> Color {
        Self::fill(0.0)
    }

    pub fn white() -> Color {
        Self::fill(1.0)
    }

    pub fn r(&self) -> f64 {
        self.base.x
    }

    pub fn g(&self) -> f64 {
        self.base.y
    }

    pub fn b(&self) -> f64 {
        self.base.z
    }

    pub fn to_gamma(&self) -> Color {
        Self::new(
            linear_to_gamma(self.r()),
            linear_to_gamma(self.g()),
            linear_to_gamma(self.b()),
        )
    }

    pub fn to_u8(&self) -> [u8; 3] {
        return [to_u8(self.r()), to_u8(self.g()), to_u8(self.b())];
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, b: Self) -> Self {
        Self::wrap_vec(self.base + b.base)
    }
}

impl Add<f64> for Color {
    type Output = Self;

    fn add(self, b: f64) -> Self {
        self + Self::fill(b)
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, b: Self) -> Self {
        Self::wrap_vec(self.base - b.base)
    }
}

impl Sub<f64> for Color {
    type Output = Self;

    fn sub(self, b: f64) -> Self {
        self - Self::fill(b)
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, b: Self) -> Self {
        Self::wrap_vec(self.base.component_mul(&b.base))
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, b: f64) -> Self {
        Self::wrap_vec(self.base * b)
    }
}

impl Div for Color {
    type Output = Self;

    fn div(self, b: Self) -> Self {
        Self::wrap_vec(self.base.component_div(&b.base))
    }
}

impl Div<f64> for Color {
    type Output = Self;

    fn div(self, b: f64) -> Self {
        Self::wrap_vec(self.base / b)
    }
}

pub fn to_u8(real: f64) -> u8 {
    let intensity = Interval::new(0.0, 0.999);
    return (256.0 * intensity.clamp(real)) as u8;
}

pub fn from_u8(i: u8) -> f64 {
    return i as f64 / 255.0;
}

fn linear_to_gamma(linear: f64) -> f64 {
    if linear > 0.0 {
        return linear.sqrt();
    }
    return 0.0;
}
