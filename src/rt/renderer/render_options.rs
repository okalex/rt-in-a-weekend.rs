use crate::util::{
    color::Color,
    types::{Float, Uint},
};

#[derive(Clone, Copy)]
pub enum SamplerType {
    Random,
    Stratified,
}

pub struct RenderOptions {
    pub img_width: Uint,
    pub img_height: Uint,
    pub samples_per_pixel: Uint,
    pub dispatch_size: Uint,
    pub max_depth: Uint,
    pub use_gpu: bool,
    pub use_multithreading: bool,
    pub use_importance_sampling: bool,
    pub background: Color,
    pub sampler_type: SamplerType,
}

pub struct RenderOptionsBuilder {
    img_width: Uint,
    samples_per_pixel: Uint,
    dispatch_size: Uint,
    max_depth: Uint,
    use_gpu: bool,
    use_multithreading: bool,
    use_importance_sampling: bool,
    background: Color,
    sampler_type: SamplerType,
}

impl RenderOptionsBuilder {
    pub fn new() -> Self {
        Self {
            img_width: 400,
            samples_per_pixel: 100,
            dispatch_size: 20,
            max_depth: 50,
            use_gpu: true,
            use_multithreading: true,
            use_importance_sampling: true,
            background: Color::new(0.7, 0.8, 1.0),
            sampler_type: SamplerType::Random,
        }
    }

    pub fn build(&self, aspect_ratio: Float) -> RenderOptions {
        RenderOptions {
            img_width: self.img_width,
            img_height: (self.img_width as Float / aspect_ratio) as Uint,
            samples_per_pixel: self.samples_per_pixel,
            dispatch_size: self.dispatch_size,
            max_depth: self.max_depth,
            use_gpu: self.use_gpu,
            use_multithreading: self.use_multithreading,
            use_importance_sampling: self.use_importance_sampling,
            background: self.background,
            sampler_type: self.sampler_type,
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

    pub fn dispatch_size(mut self, new_dispatch_size: Uint) -> Self {
        self.dispatch_size = new_dispatch_size;
        self
    }

    pub fn max_depth(mut self, new_max_depth: Uint) -> Self {
        self.max_depth = new_max_depth;
        self
    }

    pub fn use_gpu(mut self, new_use_gpu: bool) -> Self {
        self.use_gpu = new_use_gpu;
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

    pub fn sampler_type(mut self, new_sampler_type: SamplerType) -> Self {
        self.sampler_type = new_sampler_type;
        self
    }
}
