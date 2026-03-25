use std::ops::{Add, Div, Mul, Sub};

use crate::util::{
    interval::Interval,
    random::rand,
    types::{Float, Vector},
};

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub base: Vector,
}

impl Color {
    pub fn wrap_vec(base: Vector) -> Color {
        Color { base }
    }

    pub fn new(r: Float, g: Float, b: Float) -> Color {
        Self::wrap_vec(Vector::new(r, g, b))
    }

    #[allow(dead_code)]
    pub fn from_u8(values: [u8; 3]) -> Color {
        Self::new(from_u8(values[0]), from_u8(values[1]), from_u8(values[2]))
    }

    #[allow(dead_code)]
    pub fn rand() -> Color {
        Self::new(rand(), rand(), rand())
    }

    pub fn fill(c: Float) -> Color {
        Self::new(c, c, c)
    }

    pub fn black() -> Color {
        Self::fill(0.0)
    }

    pub fn white() -> Color {
        Self::fill(1.0)
    }

    pub fn r(&self) -> Float {
        self.base.x
    }

    pub fn g(&self) -> Float {
        self.base.y
    }

    pub fn b(&self) -> Float {
        self.base.z
    }

    pub fn to_gamma(&self) -> Color {
        let r = if self.r().is_nan() { 0.0 } else { self.r() };
        let g = if self.r().is_nan() { 0.0 } else { self.g() };
        let b = if self.r().is_nan() { 0.0 } else { self.b() };
        Self::new(linear_to_gamma(r), linear_to_gamma(g), linear_to_gamma(b))
    }

    pub fn to_linear(&self) -> Color {
        Self::new(gamma_to_linear(self.r()), gamma_to_linear(self.g()), gamma_to_linear(self.b()))
    }

    pub fn to_u8(&self) -> [u8; 3] {
        return [to_u8(self.r()), to_u8(self.g()), to_u8(self.b())];
    }

    #[allow(dead_code)]
    pub fn is_black(&self) -> bool {
        to_u8(self.r()) == 0 && to_u8(self.g()) == 0 && to_u8(self.b()) == 0
    }
}

impl From<[Float; 3]> for Color {
    fn from(color: [Float; 3]) -> Self {
        Self::new(color[0], color[1], color[2])
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, b: Self) -> Self {
        Self::wrap_vec(self.base + b.base)
    }
}

impl Add<Float> for Color {
    type Output = Self;

    fn add(self, b: Float) -> Self {
        self + Self::fill(b)
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, b: Self) -> Self {
        Self::wrap_vec(self.base - b.base)
    }
}

impl Sub<Float> for Color {
    type Output = Self;

    fn sub(self, b: Float) -> Self {
        self - Self::fill(b)
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, b: Self) -> Self {
        Self::wrap_vec(self.base * b.base)
    }
}

impl Mul<Float> for Color {
    type Output = Self;

    fn mul(self, b: Float) -> Self {
        Self::wrap_vec(self.base * b)
    }
}

impl Div for Color {
    type Output = Self;

    fn div(self, b: Self) -> Self {
        Self::wrap_vec(self.base / b.base)
    }
}

impl Div<Float> for Color {
    type Output = Self;

    fn div(self, b: Float) -> Self {
        Self::wrap_vec(self.base / b)
    }
}

pub fn to_u8(real: Float) -> u8 {
    let intensity = Interval::new(0.0, 0.999);
    return (256.0 * intensity.clamp(real)) as u8;
}

pub fn from_u8(i: u8) -> Float {
    return i as Float / 255.0;
}

fn linear_to_gamma(linear: Float) -> Float {
    if linear > 0.0 {
        return linear.sqrt();
    }
    return 0.0;
}

fn gamma_to_linear(gamma: Float) -> Float {
    return gamma * gamma;
}
