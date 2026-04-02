use std::ops::{Add, Div, Mul, Sub};

use crate::util::{
    interval::Interval,
    random::rand,
    types::{Float, Vector},
    vector_ext::VectorExt,
};

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub base: Vector,
}

impl Color {
    pub fn new(r: Float, g: Float, b: Float) -> Color {
        Self {
            base: Vector::new(r, g, b),
        }
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

    pub fn luminance(&self) -> Float {
        self.base.dot(Vector::new(0.2126, 0.7152, 0.0722))
    }

    pub fn is_finite(&self) -> bool {
        self.r().is_finite() && self.g().is_finite() && self.b().is_finite()
    }

    pub fn to_gamma(&self) -> Color {
        let r = if self.r().is_finite() { self.r() } else { 0.0 };
        let g = if self.g().is_finite() { self.g() } else { 0.0 };
        let b = if self.b().is_finite() { self.b() } else { 0.0 };
        Self::new(linear_to_gamma(r), linear_to_gamma(g), linear_to_gamma(b))
    }

    pub fn to_linear(&self) -> Color {
        Self::new(
            gamma_to_linear(self.r()),
            gamma_to_linear(self.g()),
            gamma_to_linear(self.b()),
        )
    }

    pub fn to_u8(&self) -> [u8; 3] {
        return [to_u8(self.r()), to_u8(self.g()), to_u8(self.b())];
    }

    #[allow(dead_code)]
    pub fn is_black(&self) -> bool {
        to_u8(self.r()) == 0 && to_u8(self.g()) == 0 && to_u8(self.b()) == 0
    }

    pub fn mix(a: Self, b: Self, factor: Float) -> Self {
        Color::from(VectorExt::lerp(a.base, b.base, factor))
    }
}

impl From<Vector> for Color {
    fn from(color: Vector) -> Self {
        Self::new(color.x, color.y, color.z)
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
        Self::from(self.base + b.base)
    }
}

impl Add<Float> for Color {
    type Output = Self;

    fn add(self, b: Float) -> Self {
        self + Self::fill(b)
    }
}

impl Add<Color> for Float {
    type Output = Color;

    fn add(self, b: Color) -> Color {
        b + self
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, b: Self) -> Self {
        Self::from(self.base - b.base)
    }
}

impl Sub<Float> for Color {
    type Output = Self;

    fn sub(self, b: Float) -> Self {
        self - Self::fill(b)
    }
}

impl Sub<Color> for Float {
    type Output = Color;

    fn sub(self, b: Color) -> Color {
        Color::new(self - b.r(), self - b.g(), self - b.b())
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, b: Self) -> Self {
        Self::from(self.base * b.base)
    }
}

impl Mul<Float> for Color {
    type Output = Self;

    fn mul(self, b: Float) -> Self {
        Self::from(self.base * b)
    }
}

impl Mul<Color> for Float {
    type Output = Color;

    fn mul(self, b: Color) -> Color {
        b * self
    }
}

impl Div for Color {
    type Output = Self;

    fn div(self, b: Self) -> Self {
        Self::from(self.base / b.base)
    }
}

impl Div<Float> for Color {
    type Output = Self;

    fn div(self, b: Float) -> Self {
        Self::from(self.base / b)
    }
}

impl Div<Color> for Float {
    type Output = Color;

    fn div(self, b: Color) -> Color {
        Color::new(self / b.r(), self / b.g(), self / b.b())
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
