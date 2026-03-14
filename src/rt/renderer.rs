use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

use crate::rt::camera::Camera;
use crate::rt::color::Color;
use crate::rt::frame_buffer::FrameBuffer;
use crate::rt::interval::Interval;
use crate::rt::materials::material::ScatterRecord;
use crate::rt::objects::hittable::{HitRecord, Hittable};
use crate::rt::objects::scene::Scene;
#[allow(unused_imports)]
use crate::rt::pdf::{CosinePdf, HemispherePdf, HittablePdf, MixturePdf, Pdf, SpherePdf};
use crate::rt::ray::Ray;

pub struct Renderer {
    pub frame_buffer: Arc<FrameBuffer>,
    workers: Vec<Arc<RenderWorker>>,
}

impl Renderer {
    pub fn new(
        options: Arc<RenderOptions>,
        camera: Arc<Camera>,
        frame_buffer: Arc<FrameBuffer>,
        line_server: Arc<LineServer>,
    ) -> Self {
        let num_threads = if options.use_multithreading {
            std::thread::available_parallelism().unwrap().get()
        } else {
            1
        };

        let workers = (0..num_threads)
            .map(|_| {
                Arc::new(RenderWorker::new(
                    Arc::clone(&options),
                    Arc::clone(&camera),
                    Arc::clone(&frame_buffer),
                    Arc::clone(&line_server),
                ))
            })
            .collect();

        Self {
            frame_buffer,
            workers,
        }
    }

    pub fn render(&self, scene: Arc<Scene>) -> Vec<JoinHandle<()>> {
        let mut thread_handles: Vec<JoinHandle<()>> = vec![];
        for worker in &self.workers {
            let worker_clone = Arc::clone(worker);
            let scene_clone = Arc::clone(&scene);

            let thread_handle = std::thread::spawn(move || {
                worker_clone.render(scene_clone);
            });

            thread_handles.push(thread_handle);
        }
        thread_handles
    }
}

pub struct RenderWorker {
    options: Arc<RenderOptions>,
    camera: Arc<Camera>,
    frame_buffer: Arc<FrameBuffer>,
    line_server: Arc<LineServer>,
}

impl RenderWorker {
    pub fn new(
        options: Arc<RenderOptions>,
        camera: Arc<Camera>,
        frame_buffer: Arc<FrameBuffer>,
        line_server: Arc<LineServer>,
    ) -> Self {
        Self {
            options,
            camera,
            frame_buffer,
            line_server,
        }
    }

    pub fn render(&self, scene: Arc<Scene>) {
        loop {
            let remaining_lines = self.line_server.len();
            eprint!("\rLines remaining: {}       ", remaining_lines);

            let maybe_line = self.line_server.next_line();
            match maybe_line {
                None => break,
                Some(line_idx) => {
                    self.render_line(&scene, line_idx);
                }
            }
        }

        let remaining_lines = self.line_server.len();
        eprint!("\rLines remaining: {}       ", remaining_lines);
    }

    fn render_line(&self, scene: &Arc<Scene>, line_idx: u32) {
        let data: Vec<[u8; 3]> = (0..self.options.img_width)
            .map(|i| self.sample_pixel(scene, i, line_idx).to_gamma().to_u8())
            .collect();

        self.frame_buffer.set_line(line_idx as usize, &data);
    }

    fn sample_pixel(&self, scene: &Arc<Scene>, i: u32, j: u32) -> Color {
        let mut pixel_color = Color::black();
        self.camera.foreach_ray(i, j, |ray| {
            pixel_color = pixel_color + self.ray_color(scene, &ray, 0);
        });
        return self.camera.sampler.integrate_samples(pixel_color);
    }

    fn ray_color(&self, scene: &Arc<Scene>, ray: &Ray, depth: u32) -> Color {
        if depth >= self.options.max_depth {
            return Color::black();
        }

        match scene.hit(ray, Interval::new(0.001, f64::INFINITY)) {
            Some(hit_record) => {
                let emitted = hit_record.mat.emitted(ray, &hit_record);

                let scattered_color = match hit_record.mat.scatter(ray, &hit_record) {
                    Some(scatter_record) => {
                        self.scatter_color(scene, ray, depth, &hit_record, &scatter_record)
                    }
                    None => Color::black(),
                };

                emitted + scattered_color
            }

            None => self.options.background,
        }
    }

    fn scatter_color(
        &self,
        scene: &Arc<Scene>,
        ray: &Ray,
        depth: u32,
        hit_record: &HitRecord,
        scatter_record: &ScatterRecord,
    ) -> Color {
        match &scatter_record.skip_pdf_ray {
            Some(skip_pdf_ray) => {
                scatter_record.attenuation * self.ray_color(scene, &skip_pdf_ray, depth + 1)
            }

            None => {
                let pdf = self.get_pdf(scene, hit_record, scatter_record);
                let scattered_ray = Ray::new(hit_record.point, pdf.generate(), ray.time);

                let scattering_pdf = hit_record
                    .mat
                    .scattering_pdf(ray, hit_record, &scattered_ray);

                let mut pdf_value = pdf.value(&scattered_ray.dir);
                if pdf_value <= 0.0 {
                    pdf_value = scattering_pdf;
                }

                let sample_color = self.ray_color(scene, &scattered_ray, depth + 1);
                scatter_record.attenuation * scattering_pdf * sample_color / pdf_value
            }
        }
    }

    fn get_pdf(
        &self,
        scene: &Arc<Scene>,
        hit_record: &HitRecord,
        scatter_record: &ScatterRecord,
    ) -> Arc<dyn Pdf> {
        if self.options.use_importance_sampling {
            if scene.lights.len() > 0 {
                let light_pdf: Arc<dyn Pdf> = Arc::new(HittablePdf::new(
                    Arc::clone(&scene.lights) as Arc<dyn Hittable>,
                    hit_record.point,
                ));
                Arc::new(MixturePdf::new(light_pdf, Arc::clone(&scatter_record.pdf)))
            } else {
                Arc::clone(&scatter_record.pdf)
            }
        } else {
            // Arc::new(HemispherePdf::new(hit_record.normal))
            Arc::new(CosinePdf::new(&hit_record.normal))
            // Arc::new(SpherePdf::new())
        }
    }
}

pub struct RenderOptions {
    pub img_width: u32,
    pub img_height: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub use_multithreading: bool,
    pub use_importance_sampling: bool,
    pub background: Color,
}

pub struct RenderOptionsBuilder {
    img_width: u32,
    samples_per_pixel: u32,
    max_depth: u32,
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

    pub fn build(&self, aspect_ratio: f64) -> RenderOptions {
        RenderOptions {
            img_width: self.img_width,
            img_height: (self.img_width as f64 / aspect_ratio) as u32,
            samples_per_pixel: self.samples_per_pixel,
            max_depth: self.max_depth,
            use_multithreading: self.use_multithreading,
            use_importance_sampling: self.use_importance_sampling,
            background: self.background,
        }
    }

    pub fn width(mut self, new_width: u32) -> Self {
        self.img_width = new_width;
        self
    }

    pub fn samples_per_pixel(mut self, new_samples_per_pixel: u32) -> Self {
        self.samples_per_pixel = new_samples_per_pixel;
        self
    }

    pub fn max_depth(mut self, new_max_depth: u32) -> Self {
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

pub struct LineServer {
    lines: Arc<Mutex<Vec<u32>>>,
}

impl LineServer {
    pub fn new(num_lines: u32) -> Self {
        let lines: Arc<Mutex<Vec<u32>>> = Arc::new(Mutex::new((0..num_lines).rev().collect()));
        Self { lines }
    }

    pub fn next_line(&self) -> Option<u32> {
        self.lines.lock().unwrap().pop()
    }

    pub fn len(&self) -> u32 {
        self.lines.lock().unwrap().len() as u32
    }
}
