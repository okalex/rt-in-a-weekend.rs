use nalgebra::Vector3;

use crate::rt::{color::Color, random::rand};

pub enum Sampler {
    Random(RandomSampler),
    Stratified(StratifiedSampler),
}

impl Sampler {
    pub fn random(samples_per_pixel: u32) -> Self {
        Self::Random(RandomSampler::new(samples_per_pixel))
    }

    pub fn stratified(samples_per_pixel: u32) -> Self {
        Self::Stratified(StratifiedSampler::new(samples_per_pixel))
    }

    pub fn foreach_sample(&self, f: impl FnMut(Vector3<f64>)) {
        match self {
            Sampler::Random(s) => s.foreach_sample(f),
            Sampler::Stratified(s) => s.foreach_sample(f),
        }
    }

    pub fn integrate_samples(&self, accumulated_color: Color) -> Color {
        match self {
            Sampler::Random(s) => s.integrate_samples(accumulated_color),
            Sampler::Stratified(s) => s.integrate_samples(accumulated_color),
        }
    }
}

pub struct RandomSampler {
    samples_per_pixel: u32,
}

impl RandomSampler {
    fn new(samples_per_pixel: u32) -> Self {
        Self { samples_per_pixel }
    }

    fn foreach_sample(&self, mut f: impl FnMut(Vector3<f64>)) {
        for _ in 0..self.samples_per_pixel {
            let offset = self.sample_square();
            f(offset)
        }
    }

    #[allow(unused)]
    fn sample_square(&self) -> Vector3<f64> {
        Vector3::new(rand() - 0.5, rand() - 0.5, 0.0)
    }

    fn integrate_samples(&self, accumulated_color: Color) -> Color {
        accumulated_color / (self.samples_per_pixel as f64)
    }
}

pub struct StratifiedSampler {
    sqrt_spp: u32,
    recip_sqrt_spp: f64,
    pixel_samples_scale: f64,
}

impl StratifiedSampler {
    fn new(samples_per_pixel: u32) -> Self {
        let sqrt_spp = (samples_per_pixel as f64).sqrt();
        let recip_sqrt_spp = 1.0 / sqrt_spp;
        let pixel_samples_scale = 1.0 / (sqrt_spp * sqrt_spp);
        Self {
            sqrt_spp: sqrt_spp as u32,
            recip_sqrt_spp,
            pixel_samples_scale,
        }
    }

    fn foreach_sample(&self, mut f: impl FnMut(Vector3<f64>)) {
        for s_j in 0..self.sqrt_spp as u32 {
            for s_i in 0..self.sqrt_spp as u32 {
                let offset = self.sample_square_stratified(s_i, s_j, self.recip_sqrt_spp);
                f(offset)
            }
        }
    }

    fn integrate_samples(&self, accumulated_color: Color) -> Color {
        accumulated_color * self.pixel_samples_scale
    }

    fn sample_square_stratified(&self, s_i: u32, s_j: u32, recip_sqrt_spp: f64) -> Vector3<f64> {
        let px = ((s_i as f64 + rand()) * recip_sqrt_spp) - 0.5;
        let py = ((s_j as f64 + rand()) * recip_sqrt_spp) - 0.5;
        Vector3::new(px, py, 0.0)
    }
}
