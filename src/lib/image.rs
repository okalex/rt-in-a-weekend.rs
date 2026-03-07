use image::ImageReader;
use std::path::Path;

use crate::lib::color::Color;

pub struct Image {
    pub width: u32,
    pub height: u32,
    pixels: Vec<Vec<Color>>,
}

impl Image {
    pub fn load(filename: &str) -> Self {
        let path = Path::new(filename);
        let reader = ImageReader::open(path).expect("Could not load image"); // Note: this is not safe - ignoring errors for now
        let decoded = reader.decode().expect("Could not decode image"); // Note: this is not safe - ignoring errors for now
        let img = decoded.to_rgb32f();
        let width = img.width();
        let height = img.height();

        let mut pixels: Vec<Vec<Color>> =
            vec![vec![Color::black(); width as usize]; height as usize];
        for j in 0..height {
            for i in 0..height {
                let pixel = img.get_pixel(i, j); // Note: this is not safe - ignoring errors for now
                let color = Color::new(pixel[0] as f64, pixel[1] as f64, pixel[2] as f64);
                pixels[j as usize][i as usize] = color;
            }
        }

        Self {
            width,
            height,
            pixels,
        }
    }

    pub fn pixel_data(&self, x: u32, y: u32) -> Color {
        self.pixels[y as usize][x as usize]
    }
}
