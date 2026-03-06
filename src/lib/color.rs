use crate::lib::interval::Interval;
use crate::lib::random::rand;
use crate::lib::vec3::Vec3;
use std::ops::{Add, Mul, Sub};

#[derive(Clone, Copy)]
pub struct Color {
    base: Vec3,
}

impl Color {
    pub fn wrap_vec(base: Vec3) -> Color {
        Color { base }
    }

    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Self::wrap_vec(Vec3::new(r, g, b))
    }

    pub fn from_arr(values: [f64; 3]) -> Color {
        Self::wrap_vec(Vec3::new_arr(values))
    }

    pub fn from_u8(values: [u8; 3]) -> Color {
        Self::new(from_u8(values[0]), from_u8(values[1]), from_u8(values[2]))
    }

    pub fn rand() -> Color {
        Self::new(rand(), rand(), rand())
    }

    pub fn zeroes() -> Color {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn r(&self) -> f64 {
        self.base.x()
    }

    pub fn g(&self) -> f64 {
        self.base.y()
    }

    pub fn b(&self) -> f64 {
        self.base.z()
    }

    pub fn to_gamma(&self) -> Color {
        Self::new(
            linear_to_gamma(self.r()),
            linear_to_gamma(self.g()),
            linear_to_gamma(self.b()),
        )
    }

    pub fn scale(&self, scale_factor: f64) -> Color {
        Color {
            base: self.base.scale(scale_factor),
        }
    }

    pub fn to_string(&self) -> String {
        let color = self.to_u8();
        return format!("{} {} {} ", color[0], color[1], color[2]);
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

impl Sub for Color {
    type Output = Self;

    fn sub(self, b: Self) -> Self {
        Self::wrap_vec(self.base - b.base)
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, b: Self) -> Self {
        Self::wrap_vec(self.base * b.base)
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
