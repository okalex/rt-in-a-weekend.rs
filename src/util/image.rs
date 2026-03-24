use std::path::Path;

use image::{ImageReader, metadata::CicpColorPrimaries as ColorSpace};

use crate::util::{
    color::Color,
    types::{Float, Uint},
};

pub struct Image {
    pub width: Uint,
    pub height: Uint,
    pixels: Vec<Vec<Color>>,
}

impl Image {
    pub fn load(filename: &str) -> Self {
        eprintln!("Loading image: {}", filename);

        let path = Path::new(filename);
        let reader = ImageReader::open(path).expect("Could not load image"); // Note: this is not safe - ignoring errors for now
        let decoded = reader.decode().expect("Could not decode image"); // Note: this is not safe - ignoring errors for now
        let img = decoded.to_rgb32f();
        let primary_color_space = img.color_space().primaries;
        let width = img.width();
        let height = img.height();

        let mut pixels: Vec<Vec<Color>> = vec![vec![Color::black(); width as usize]; height as usize];
        for j in 0..height {
            for i in 0..width {
                let pixel = img.get_pixel(i, j); // Note: this is not safe - ignoring errors for now
                let color = Color::new(pixel[0] as Float, pixel[1] as Float, pixel[2] as Float);
                pixels[j as usize][i as usize] = match primary_color_space {
                    ColorSpace::SRgb => color.to_linear(), // Only converting srgb currently
                    _ => color,
                };
            }
        }

        Self {
            width: width as Uint,
            height: height as Uint,
            pixels,
        }
    }

    pub fn pixel_data(&self, x: Uint, y: Uint) -> Color {
        self.pixels[y as usize][x as usize]
    }
}
