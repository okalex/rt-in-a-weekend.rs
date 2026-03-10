use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

use nalgebra::{Point3, Vector3};

use crate::rt::camera::Camera;
use crate::rt::color::Color;
use crate::rt::frame_buffer::FrameBuffer;
use crate::rt::hittable::Hittable;
use crate::rt::interval::Interval;
use crate::rt::random::{rand, rand_in_unit_disk};
use crate::rt::ray::Ray;
use crate::rt::viewport::Viewport;

pub struct Renderer {
    options: Arc<RenderOptions>,
    camera: Arc<Camera>,
    viewport: Arc<Viewport>,
    pub frame_buffer: Arc<FrameBuffer>,
    line_server: Arc<LineServer>,
}

impl Renderer {
    pub fn new(
        options: Arc<RenderOptions>,
        camera: Arc<Camera>,
        viewport: Arc<Viewport>,
        frame_buffer: Arc<FrameBuffer>,
        line_server: Arc<LineServer>,
    ) -> Self {
        Self {
            options,
            camera,
            viewport,
            frame_buffer,
            line_server,
        }
    }

    pub fn render(&self, scene: Arc<dyn Hittable>) -> Vec<JoinHandle<()>> {
        let num_threads = if self.options.use_multithreading {
            std::thread::available_parallelism().unwrap().get()
        } else {
            1
        };

        (0..num_threads)
            .map(|_| {
                let worker = RenderWorker::new(
                    Arc::clone(&self.options),
                    Arc::clone(&self.camera),
                    Arc::clone(&self.viewport),
                    Arc::clone(&self.frame_buffer),
                    Arc::clone(&self.line_server),
                    Arc::clone(&scene),
                );
                std::thread::spawn(move || {
                    worker.render();
                })
            })
            .collect()
    }
}

pub struct RenderWorker {
    options: Arc<RenderOptions>,
    camera: Arc<Camera>,
    viewport: Arc<Viewport>,
    frame_buffer: Arc<FrameBuffer>,
    line_server: Arc<LineServer>,
    scene: Arc<dyn Hittable>,
}

impl RenderWorker {
    pub fn new(
        options: Arc<RenderOptions>,
        camera: Arc<Camera>,
        viewport: Arc<Viewport>,
        frame_buffer: Arc<FrameBuffer>,
        line_server: Arc<LineServer>,
        scene: Arc<dyn Hittable>,
    ) -> Self {
        Self {
            options,
            camera,
            viewport,
            frame_buffer,
            line_server,
            scene,
        }
    }

    pub fn render(&self) {
        loop {
            let maybe_line = self.line_server.next_line();
            match maybe_line {
                None => break,
                Some(line_idx) => {
                    self.render_line(&self.scene, line_idx);

                    let remaining_lines = self.line_server.len();
                    eprint!("\rLines remaining: {}       ", remaining_lines);
                }
            }
        }
    }

    fn render_line(&self, scene: &Arc<dyn Hittable>, line_idx: u32) {
        let data: Vec<[u8; 3]> = (0..self.options.img_width)
            .map(|i| self.sample_pixel(&scene, i, line_idx).to_gamma().to_u8())
            .collect();

        self.frame_buffer.set_line(line_idx as usize, &data);
    }

    fn sample_pixel(&self, scene: &Arc<dyn Hittable>, i: u32, j: u32) -> Color {
        let mut pixel_color = Color::black();
        for _ in 0..self.options.samples_per_pixel {
            let r = self.get_ray(i, j);
            pixel_color = pixel_color + self.ray_color(&r, self.options.max_depth, scene);
        }
        return pixel_color / (self.options.samples_per_pixel as f64);
    }

    fn sample_square(&self) -> Vector3<f64> {
        return Vector3::new(rand() - 0.5, rand() - 0.5, 0.0);
    }

    fn defocus_disk_sample(&self) -> Point3<f64> {
        let p = rand_in_unit_disk();
        return Point3::from(
            self.camera.position.coords
                + (self.viewport.defocus_disk.u * p.x)
                + (self.viewport.defocus_disk.v * p.y),
        );
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = self
            .viewport
            .pixel_loc(i as f64 + offset.x, j as f64 + offset.y);

        let ray_origin = if self.camera.defocus_angle <= 0.0 {
            self.camera.position
        } else {
            self.defocus_disk_sample()
        };
        let ray_dir = pixel_sample - ray_origin;
        let ray_time = rand();

        Ray::new(ray_origin, ray_dir, ray_time)
    }

    fn ray_color(&self, ray: &Ray, depth: u32, scene: &Arc<dyn Hittable>) -> Color {
        if depth <= 0 {
            return Color::black();
        }

        match scene.hit(ray, Interval::new(0.001, f64::INFINITY)) {
            Some(hit_record) => {
                let emitted = hit_record
                    .mat
                    .emitted(hit_record.u, hit_record.v, &hit_record.point);

                let scattered_color = match hit_record.mat.scatter(ray, &hit_record) {
                    Some(scattered) => {
                        scattered.attenuation * self.ray_color(&scattered.ray, depth - 1, scene)
                    }
                    None => Color::black(),
                };

                emitted + scattered_color
            }

            None => self.options.background,
        }
    }
}

pub struct RenderOptions {
    pub img_width: u32,
    pub img_height: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub use_multithreading: bool,
    pub background: Color,
}

pub struct RenderOptionsBuilder {
    img_width: u32,
    samples_per_pixel: u32,
    max_depth: u32,
    use_multithreading: bool,
    background: Color,
}

impl RenderOptionsBuilder {
    pub fn new() -> Self {
        Self {
            img_width: 400,
            samples_per_pixel: 100,
            max_depth: 50,
            use_multithreading: true,
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
