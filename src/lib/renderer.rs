use std::sync::{Arc, Mutex};
use std::thread;

use crate::lib::camera::Camera;
use crate::lib::color::Color;
use crate::lib::hittable::{HitRecord, Hittable};
use crate::lib::interval::Interval;
use crate::lib::materials::lambertian::Lambertian;
use crate::lib::materials::material::Material;
use crate::lib::random::rand;
use crate::lib::ray::Ray;
use crate::lib::vec3::Vec3;
use crate::lib::viewport::Viewport;
use crate::lib::writer::Writer;

pub struct Renderer {
    options: RenderOptions,
    camera: Arc<Camera>,
    viewport: Arc<Viewport>,
    writer: Arc<dyn Writer>,
}

impl Renderer {
    pub fn new(
        options: RenderOptions,
        camera: Arc<Camera>,
        viewport: Arc<Viewport>,
        writer: Arc<dyn Writer>,
    ) -> Self {
        Self {
            options,
            camera,
            viewport,
            writer,
        }
    }

    pub fn render(&self, scene: Arc<dyn Hittable>) {
        let num_threads = if self.options.use_multithreading {
            std::thread::available_parallelism().unwrap().get()
        } else {
            1
        };

        let lines: Arc<Mutex<Vec<u32>>> =
            Arc::new(Mutex::new((0..self.options.img_height).collect()));

        self.writer.init();

        thread::scope(|s| {
            for _ in 0..num_threads {
                let scene_clone: Arc<dyn Hittable> = Arc::clone(&scene);
                let lines_clone = Arc::clone(&lines);

                s.spawn(move || {
                    loop {
                        let maybe_line = lines_clone.lock().unwrap().pop();
                        match maybe_line {
                            None => break,
                            Some(line_idx) => {
                                self.render_line(&scene_clone, line_idx);
                                let remaining_lines = lines_clone.lock().unwrap().len();
                                eprint!("\rLines remaining: {}       ", remaining_lines);
                            }
                        }
                    }
                });
            }
        });

        self.writer.close();
        eprintln!("\n\rDone.");
    }

    fn render_line(&self, scene: &Arc<dyn Hittable>, line_idx: u32) {
        let data = (0..self.options.img_width)
            .map(|i| self.sample_pixel(&scene, i, line_idx).to_gamma().to_u8())
            .collect();

        self.writer.write_line(line_idx as usize, &data);
    }

    fn sample_pixel(&self, scene: &Arc<dyn Hittable>, i: u32, j: u32) -> Color {
        let mut pixel_color = Color::black();
        for _ in 0..self.options.samples_per_pixel {
            let r = self.get_ray(i, j);
            pixel_color = pixel_color + self.ray_color(&r, self.options.max_depth, scene);
        }
        return pixel_color / (self.options.samples_per_pixel as f64);
    }

    fn sample_square(&self) -> Vec3 {
        return Vec3::new(rand() - 0.5, rand() - 0.5, 0.0);
    }

    fn defocus_disk_sample(&self) -> Vec3 {
        let p = Vec3::rand_in_unit_disk();
        return self.camera.position
            + (self.camera.defocus_disk.u * p.x())
            + (self.camera.defocus_disk.v * p.y());
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = self.viewport.pixel00_loc(&self.camera)
            + self.viewport.delta_u * (i as f64 + offset.x())
            + self.viewport.delta_v * (j as f64 + offset.y());

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

        let mat: Arc<dyn Material> = Arc::new(Lambertian::from_color(Color::black()));
        let mut rec = HitRecord::empty(Arc::clone(&mat));

        if scene.hit(ray, Interval::new(0.001, f64::INFINITY), &mut rec) {
            let emitted = rec.mat.emitted(rec.u, rec.v, &rec.point);
            let scattered_color = match rec.mat.scatter(ray, &rec) {
                Some(scattered) => {
                    scattered.attenuation * self.ray_color(&scattered.ray, depth - 1, scene)
                }
                None => Color::black(),
            };
            emitted + scattered_color
        } else {
            self.options.background
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

    pub fn build(&self, camera: &Camera) -> RenderOptions {
        RenderOptions {
            img_width: self.img_width,
            img_height: (self.img_width as f64 / camera.aspect_ratio) as u32,
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

    pub fn background(mut self, new_background: Color) -> Self {
        self.background = new_background;
        self
    }
}
