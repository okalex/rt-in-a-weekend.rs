use std::f64::consts::PI;
use std::thread;
use std::sync::{Arc, Mutex};
use crate::lib::{vec3, ray, scene, color, interval, random, viewport, writer, line_counter};

#[derive(Clone, Copy)]
pub struct CameraOptions {
  pub img_width: u32,
  pub img_height: u32,
  pub vfov: f64,
  pub lookfrom: vec3::Vec3,
  pub lookat: vec3::Vec3,
  pub vup: vec3::Vec3,
  pub defocus_angle: f64,
  pub focus_dist: f64,
  pub samples_per_pixel: u32,
  pub max_depth: u32,
}

pub struct Camera {
  pub options: CameraOptions,
  pub center: vec3::Vec3,
  viewport: viewport::Viewport,
  defocus_disk_u: vec3::Vec3,
  defocus_disk_v: vec3::Vec3,
}

impl Camera {

  pub fn render(&self, scene: Arc<scene::Scene>, writer: Arc<dyn writer::Writer>) {
    let num_threads = std::thread::available_parallelism().unwrap().get();
    let segments = segment_lines(self.options.img_height, num_threads as u32);
    let line_counter = Arc::new(Mutex::new(line_counter::LineCounter::new(self.options.img_height)));

    thread::scope(|s| {
      for (start, end) in segments {
        let counter_clone = Arc::clone(&line_counter);
        let scene_clone = Arc::clone(&scene);
        let writer_clone = Arc::clone(&writer);

        s.spawn(move || {
          eprintln!("Rendering lines {} to {}...", start, end - 1);

          for j in start..end {
            for i in 0..self.options.img_width {
              let pixel_color = self.sample_pixel(&scene_clone, i, j).to_gamma();
              writer_clone.write_pixel(i as usize, j as usize, pixel_color.to_u8());
            }

            let mut c = counter_clone.lock().unwrap();
            c.dec();
            c.announce();
          }
        });
      }
    });

    eprintln!("\n\rDone.");
  }

  fn sample_pixel(&self, scene: &Arc<scene::Scene>, i: u32, j: u32) -> color::Color {
    let mut pixel_color = color::new_vec(0.0, 0.0, 0.0);
    for _ in 0..self.options.samples_per_pixel {
      let r = self.get_ray(i, j);
      pixel_color = pixel_color + self.ray_color(&r, self.options.max_depth, scene);
    }
    return pixel_color.scale(1.0 / (self.options.samples_per_pixel as f64));
  }

  fn sample_square(&self) -> vec3::Vec3 {
    return vec3::new(
      random::rand() - 0.5,
      random::rand() - 0.5,
      0.0
    );
  }

  fn defocus_disk_sample(&self) -> vec3::Vec3 {
    let p = vec3::rand_in_unit_disk();
    return self.center + self.defocus_disk_u.scale(p.x()) + self.defocus_disk_v.scale(p.y());
  }

  fn get_ray(&self, i: u32, j: u32) -> ray::Ray {
    let offset = self.sample_square();
    let pixel_sample = self.viewport.pixel00_loc(self)
      + self.viewport.delta_u().scale(i as f64 + offset.x())
      + self.viewport.delta_v().scale(j as f64 + offset.y());

    let ray_origin = if self.options.defocus_angle <= 0.0 { self.center } else { self.defocus_disk_sample() };
    let ray_dir = pixel_sample - ray_origin;

    return ray::new(ray_origin, ray_dir);
  }

  fn ray_color(&self, ray: &ray::Ray, depth: u32, scene: &Arc<scene::Scene>) -> color::Color {
    if depth <= 0 {
      return color::new_vec(0.0, 0.0, 0.0);
    }

    let interval = interval::new(0.001, 1000000.0);
    let hit_record = scene.hit(ray, interval);
    if hit_record.is_hit {
      let scattered = hit_record.mat.scatter(ray, &hit_record);
      return scattered.attenuation * self.ray_color(&scattered.ray, depth - 1, scene);
    }
    return self.background(ray);
  }

  fn background(&self, ray: &ray::Ray) -> color::Color {
    let a = 0.5 * (ray.dir().unit().y() + 1.0);
    let white = color::wrap_vec(vec3::ones());
    let blue = color::new_vec(0.5, 0.7, 1.0);
    return white.scale(1.0 - a) + blue.scale(a);
  }
}

pub fn new(options: CameraOptions) -> Camera {
  let theta = degrees_to_radians(options.vfov);
  let h = (theta / 2.0).tan();
  let viewport_height = 2.0 * h * options.focus_dist;
  let viewport_width = viewport_height * (options.img_width as f64) / (options.img_height as f64);
  let w = (options.lookfrom - options.lookat).unit();
  let u = options.vup.cross(&w).unit();
  let v = w.cross(&u);
  let viewport = viewport::new(options.img_width, options.img_height, viewport_width, viewport_height, u, v, w);

  let defocus_radius = options.focus_dist * degrees_to_radians(options.defocus_angle / 2.0).tan();

  return Camera {
    options: options,
    center: options.lookfrom,
    viewport: viewport,
    defocus_disk_u: u.scale(defocus_radius),
    defocus_disk_v: v.scale(defocus_radius),
  }
}

fn degrees_to_radians(degrees: f64) -> f64 {
  return degrees * PI / 180.0;
}

fn segment_lines(num_lines: u32, num_segments: u32) -> Vec<(u32, u32)> {
  let segment_size = num_lines / num_segments;
  let mut segments = Vec::new();
  for i in 0..num_segments {
    let start = i * segment_size;
    let end = if i < (num_segments - 1) {
      (i + 1) * segment_size
    } else {
      num_lines
    };
    segments.push((start, end));
  }
  return segments;
}
