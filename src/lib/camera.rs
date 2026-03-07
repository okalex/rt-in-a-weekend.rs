use crate::lib::color::Color;
use crate::lib::hittable::Hittable;
use crate::lib::interval::Interval;
use crate::lib::random::rand;
use crate::lib::ray::Ray;
use crate::lib::util::degrees_to_radians;
use crate::lib::vec3::Vec3;
use crate::lib::viewport::Viewport;
use crate::lib::writer::Writer;
use core::f64;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct Camera {
    options: CameraOptions,
    pub center: Vec3,
    viewport: Viewport,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn new(options: CameraOptions) -> Camera {
        let theta = degrees_to_radians(options.vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * options.focus_dist;
        let viewport_width = viewport_height * (options.width as f64) / (options.height as f64);
        let w = (options.lookfrom - options.lookat).unit();
        let u = options.vup.cross(&w).unit();
        let v = w.cross(&u);
        let viewport = Viewport::new(
            options.width,
            options.height,
            viewport_width,
            viewport_height,
            u,
            v,
            w,
        );

        let defocus_radius =
            options.focus_dist * degrees_to_radians(options.defocus_angle / 2.0).tan();

        Camera {
            options: options,
            center: options.lookfrom,
            viewport: viewport,
            defocus_disk_u: u * defocus_radius,
            defocus_disk_v: v * defocus_radius,
        }
    }

    pub fn render(&self, scene: Arc<dyn Hittable>, writer: Arc<dyn Writer>) {
        let num_threads = if self.options.use_multithreading {
            std::thread::available_parallelism().unwrap().get()
        } else {
            1
        };

        let lines: Arc<Mutex<Vec<u32>>> = Arc::new(Mutex::new((0..self.options.height).collect()));

        thread::scope(|s| {
            for _ in 0..num_threads {
                let scene_clone: Arc<dyn Hittable> = Arc::clone(&scene);
                let writer_clone = Arc::clone(&writer);
                let lines_clone = Arc::clone(&lines);

                s.spawn(move || {
                    loop {
                        let maybe_line = lines_clone.lock().unwrap().pop();
                        match maybe_line {
                            None => break,
                            Some(line) => {
                                self.render_line(line, &scene_clone, &writer_clone);
                                let remaining_lines = lines_clone.lock().unwrap().len();
                                eprint!("\rLines remaining: {}       ", remaining_lines);
                            }
                        }
                    }
                });
            }
        });

        eprintln!("\n\rDone.");
    }

    pub fn width(&self) -> u32 {
        self.options.width
    }

    pub fn height(&self) -> u32 {
        self.options.height
    }

    pub fn focus_dist(&self) -> f64 {
        self.options.focus_dist
    }

    fn render_line(&self, line: u32, scene: &Arc<dyn Hittable>, writer: &Arc<dyn Writer>) {
        let mut data: Vec<[u8; 3]> = vec![];
        for i in 0..self.options.width {
            let pixel_color = self.sample_pixel(Arc::clone(&scene), i, line).to_gamma();
            data.push(pixel_color.to_u8());
        }

        writer.write_line(line as usize, &data);
    }

    fn sample_pixel(&self, scene: Arc<dyn Hittable>, i: u32, j: u32) -> Color {
        let mut pixel_color = Color::black();
        for _ in 0..self.options.samples_per_pixel {
            let r = self.get_ray(i, j);
            pixel_color =
                pixel_color + self.ray_color(&r, self.options.max_depth, Arc::clone(&scene));
        }
        return pixel_color / (self.options.samples_per_pixel as f64);
    }

    fn sample_square(&self) -> Vec3 {
        return Vec3::new(rand() - 0.5, rand() - 0.5, 0.0);
    }

    fn defocus_disk_sample(&self) -> Vec3 {
        let p = Vec3::rand_in_unit_disk();
        return self.center + (self.defocus_disk_u * p.x()) + (self.defocus_disk_v * p.y());
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = self.viewport.pixel00_loc(self)
            + self.viewport.delta_u() * (i as f64 + offset.x())
            + self.viewport.delta_v() * (j as f64 + offset.y());

        let ray_origin = if self.options.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_dir = pixel_sample - ray_origin;
        let ray_time = rand();

        Ray::new(ray_origin, ray_dir, ray_time)
    }

    fn ray_color(&self, ray: &Ray, depth: u32, scene: Arc<dyn Hittable>) -> Color {
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

#[derive(Clone, Copy)]
struct CameraOptions {
    width: u32,
    height: u32,
    background: Color,
    vfov: f64,
    lookfrom: Vec3,
    lookat: Vec3,
    vup: Vec3,
    defocus_angle: f64,
    focus_dist: f64,
    samples_per_pixel: u32,
    max_depth: u32,
    use_multithreading: bool,
}

pub struct CameraBuilder {
    width: u32,
    aspect_ratio: f64,
    background: Color,
    vfov: f64,
    lookfrom: Vec3,
    lookat: Vec3,
    vup: Vec3,
    defocus_angle: f64,
    focus_dist: f64,
    samples_per_pixel: u32,
    max_depth: u32,
    use_multithreading: bool,
}

impl CameraBuilder {
    pub fn new() -> Self {
        CameraBuilder {
            width: 400,
            aspect_ratio: 16.0 / 9.0,
            background: Color::new(0.7, 0.8, 1.0),
            vfov: 20.0,
            lookfrom: Vec3::new(0.0, 1.0, 0.0),
            lookat: Vec3::new(0.0, 0.0, 0.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_dist: 1.0,
            samples_per_pixel: 100,
            max_depth: 50,
            use_multithreading: true,
        }
    }

    pub fn build(&self) -> Camera {
        let options = self.make_options();
        Camera::new(options)
    }

    fn make_options(&self) -> CameraOptions {
        let height = (self.width as f64 / self.aspect_ratio) as u32;
        CameraOptions {
            width: self.width,
            height: height,
            background: self.background,
            vfov: self.vfov,
            lookfrom: self.lookfrom,
            lookat: self.lookat,
            vup: self.vup,
            defocus_angle: self.defocus_angle,
            focus_dist: self.focus_dist,
            samples_per_pixel: self.samples_per_pixel,
            max_depth: self.max_depth,
            use_multithreading: self.use_multithreading,
        }
    }

    pub fn width(mut self, new_width: u32) -> Self {
        self.width = new_width;
        self
    }

    pub fn aspect_ratio(mut self, new_aspect_ratio: f64) -> Self {
        self.aspect_ratio = new_aspect_ratio;
        self
    }

    pub fn background(mut self, new_background: [f64; 3]) -> Self {
        self.background = Color::from_arr(new_background);
        self
    }

    pub fn vfov(mut self, new_vfov: f64) -> Self {
        self.vfov = new_vfov;
        self
    }

    pub fn lookfrom(mut self, new_lookfrom: [f64; 3]) -> Self {
        self.lookfrom = Vec3::new_arr(new_lookfrom);
        self
    }

    pub fn lookat(mut self, new_lookat: [f64; 3]) -> Self {
        self.lookat = Vec3::new_arr(new_lookat);
        self
    }

    pub fn vup(mut self, new_vup: [f64; 3]) -> Self {
        self.vup = Vec3::new_arr(new_vup);
        self
    }

    pub fn defocus_angle(mut self, new_defocus_angle: f64) -> Self {
        self.defocus_angle = new_defocus_angle;
        self
    }

    pub fn focus_dist(mut self, new_focus_dist: f64) -> Self {
        self.focus_dist = new_focus_dist;
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
}
