use std::sync::Arc;

use crate::lib::color::Color;
use crate::lib::image::Image;
use crate::lib::interval::Interval;
use crate::lib::perlin::Perlin;
use crate::lib::vec3::Vec3;

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, point: &Vec3) -> Color;
}

pub struct SolidColor {
    albedo: Color,
}

impl SolidColor {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }

    pub fn from_arr(color: [f64; 3]) -> Self {
        Self::new(Color::from_arr(color))
    }
}

impl Texture for SolidColor {
    fn value(&self, u: f64, v: f64, point: &Vec3) -> Color {
        self.albedo
    }
}

pub struct CheckerTexture {
    inv_scale: f64,
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(scale: f64, even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> Self {
        let inv_scale = 1.0 / scale;
        Self {
            inv_scale,
            even,
            odd,
        }
    }

    pub fn from_colors(scale: f64, even_color: Color, odd_color: Color) -> Self {
        let even: Arc<dyn Texture> = Arc::new(SolidColor::new(even_color));
        let odd: Arc<dyn Texture> = Arc::new(SolidColor::new(odd_color));
        Self::new(scale, even, odd)
    }

    pub fn from_color_values(scale: f64, even_color: [f64; 3], odd_color: [f64; 3]) -> Self {
        Self::from_colors(
            scale,
            Color::from_arr(even_color),
            Color::from_arr(odd_color),
        )
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, point: &Vec3) -> Color {
        let x_int = (self.inv_scale * point.x()).floor() as i64;
        let y_int = (self.inv_scale * point.y()).floor() as i64;
        let z_int = (self.inv_scale * point.z()).floor() as i64;
        let is_even = (x_int + y_int + z_int) % 2 == 0;
        if is_even {
            self.even.value(u, v, point)
        } else {
            self.odd.value(u, v, point)
        }
    }
}

impl Clone for CheckerTexture {
    fn clone(&self) -> Self {
        Self {
            inv_scale: self.inv_scale,
            even: Arc::clone(&self.even),
            odd: Arc::clone(&self.odd),
        }
    }
}

pub struct ImageTexture {
    image: Image,
}

impl ImageTexture {
    pub fn new(filename: &str) -> Self {
        let image = Image::load(filename);
        Self { image }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, point: &Vec3) -> Color {
        if self.image.height <= 0 {
            return Color::new(0.0, 1.0, 1.0);
        }

        let interval = Interval::new(0.0, 1.0);
        let u_clamped = interval.clamp(u);
        let v_clamped = 1.0 - interval.clamp(v);

        let i = (u_clamped * (self.image.width as f64)) as u32;
        let j = (v_clamped * (self.image.height as f64)) as u32;

        self.image.pixel_data(i, j)
    }
}

pub struct NoiseTexture {
    scale: f64,
    noise: Perlin,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        Self {
            scale,
            noise: Perlin::new(),
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, u: f64, v: f64, point: &Vec3) -> Color {
        let scaled = point.scale(self.scale);
        // let noise = 0.5 * (1.0 + self.noise.noise(&point.scale(self.scale))); // Perlin noise
        let noise = self.noise.turb(&scaled, 7); // Turbulent noise
        // let noise = 0.5 * (1.0 + (self.scale * point.z() + 10.0 * self.noise.turb(point, 7)).sin());
        Color::wrap_vec(Vec3::ones().scale(noise))
    }
}
