use crate::rt::{color::Color, types::{Float, Uint}};

pub struct RenderOptions {
    pub img_width: Uint,
    pub img_height: Uint,
    pub samples_per_pixel: Uint,
    pub max_depth: Uint,
    pub use_multithreading: bool,
    pub use_importance_sampling: bool,
    pub background: Color,
}

pub struct RenderOptionsBuilder {
    img_width: Uint,
    samples_per_pixel: Uint,
    max_depth: Uint,
    use_multithreading: bool,
    use_importance_sampling: bool,
    background: Color,
}

impl RenderOptionsBuilder {
    pub fn new() -> Self {
        Self {
            img_width: 400,
            samples_per_pixel: 100,
            max_depth: 50,
            use_multithreading: true,
            use_importance_sampling: true,
            background: Color::new(0.7, 0.8, 1.0),
        }
    }

    pub fn build(&self, aspect_ratio: Float) -> RenderOptions {
        RenderOptions {
            img_width: self.img_width,
            img_height: (self.img_width as Float / aspect_ratio) as Uint,
            samples_per_pixel: self.samples_per_pixel,
            max_depth: self.max_depth,
            use_multithreading: self.use_multithreading,
            use_importance_sampling: self.use_importance_sampling,
            background: self.background,
        }
    }

    pub fn width(mut self, new_width: Uint) -> Self {
        self.img_width = new_width;
        self
    }

    pub fn samples_per_pixel(mut self, new_samples_per_pixel: Uint) -> Self {
        self.samples_per_pixel = new_samples_per_pixel;
        self
    }

    pub fn max_depth(mut self, new_max_depth: Uint) -> Self {
        self.max_depth = new_max_depth;
        self
    }

    pub fn use_multithreading(mut self, new_use_multithreading: bool) -> Self {
        self.use_multithreading = new_use_multithreading;
        self
    }

    pub fn use_importance_sampling(mut self, new_use_importance_sampling: bool) -> Self {
        self.use_importance_sampling = new_use_importance_sampling;
        self
    }

    #[allow(dead_code)]
    pub fn background(mut self, new_background: Color) -> Self {
        self.background = new_background;
        self
    }
}
