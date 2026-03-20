use std::sync::Arc;
use std::time::Instant;

use crate::rt::camera::Camera;
use crate::rt::color::Color;
use crate::rt::frame_buffer::FrameBuffer;
use crate::rt::interval::Interval;
use crate::rt::materials::material::{Material, ScatterRecord};
use crate::rt::geometry::hit_record::HitRecord;
use crate::rt::geometry::scene::Scene;
use crate::rt::pdf::{CosinePdf, HittablePdf, MixturePdf, Pdf};
use crate::rt::ray::Ray;
use crate::rt::renderer::cpu::line_server::LineServer;
use crate::rt::renderer::render_options::RenderOptions;
use crate::rt::types::{INFINITY, Uint};

pub struct CpuRenderer {
    workers: Vec<Arc<CpuRenderWorker>>,
}

impl CpuRenderer {
    pub fn new(
        options: Arc<RenderOptions>,
        scene: Arc<Scene>,
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
                Arc::new(CpuRenderWorker::new(
                    Arc::clone(&options),
                    Arc::clone(&scene),
                    Arc::clone(&camera),
                    Arc::clone(&frame_buffer),
                    Arc::clone(&line_server),
                ))
            })
            .collect();

        Self { workers }
    }

    pub async fn render(&self) {
        let now = Instant::now();

        let mut thread_handles = vec![];
        for worker in &self.workers {
            let worker_clone = Arc::clone(worker);

            let thread_handle = tokio::task::spawn_blocking(move || {
                worker_clone.render();
            });

            thread_handles.push(thread_handle);
        }

        futures::future::join_all(thread_handles).await;

        eprintln!("Done rendering: {}ms", now.elapsed().as_millis());
    }
}

pub struct CpuRenderWorker {
    options: Arc<RenderOptions>,
    scene: Arc<Scene>,
    camera: Arc<Camera>,
    frame_buffer: Arc<FrameBuffer>,
    line_server: Arc<LineServer>,
}

impl CpuRenderWorker {
    pub fn new(
        options: Arc<RenderOptions>,
        scene: Arc<Scene>,
        camera: Arc<Camera>,
        frame_buffer: Arc<FrameBuffer>,
        line_server: Arc<LineServer>,
    ) -> Self {
        Self {
            options,
            scene,
            camera,
            frame_buffer,
            line_server,
        }
    }

    pub fn render(&self) {
        loop {
            let remaining_lines = self.line_server.len();
            eprint!("\rLines remaining: {}       ", remaining_lines);

            let maybe_line = self.line_server.next_line();
            match maybe_line {
                None => break,
                Some(line_idx) => {
                    self.render_line(line_idx);
                }
            }
        }

        let remaining_lines = self.line_server.len();
        eprint!("\rLines remaining: {}       ", remaining_lines);
    }

    fn render_line(&self, line_idx: Uint) {
        let data: Vec<[u8; 3]> = (0..self.options.img_width)
            .map(|i| self.sample_pixel(i, line_idx).to_gamma().to_u8())
            .collect();

        self.frame_buffer.set_line(line_idx as usize, &data);
    }

    fn sample_pixel(&self, i: Uint, j: Uint) -> Color {
        let mut pixel_color = Color::black();
        self.camera.foreach_ray(i, j, |ray| {
            pixel_color = pixel_color + self.ray_color(&ray, 0);
        });
        return self.camera.sampler.integrate_samples(pixel_color);
    }

    fn ray_color(&self, ray: &Ray, depth: Uint) -> Color {
        if depth >= self.options.max_depth {
            return Color::black();
        }

        match self.scene.hit(ray, Interval::new(0.001, INFINITY)) {
            Some(hit_record) => {
                let mat = self.scene.get_material(hit_record.mat_idx);

                let emitted = mat.emitted(ray, &hit_record);

                let scattered_color = match mat.scatter(ray, &hit_record) {
                    Some(scatter_record) => {
                        self.scatter_color(ray, depth, &hit_record, &scatter_record, mat)
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
        ray: &Ray,
        depth: Uint,
        hit_record: &HitRecord,
        scatter_record: &ScatterRecord,
        mat: &Material,
    ) -> Color {
        match &scatter_record.skip_pdf_ray {
            Some(skip_pdf_ray) => {
                scatter_record.attenuation * self.ray_color(&skip_pdf_ray, depth + 1)
            }

            None => {
                let pdf = self.get_pdf(hit_record, scatter_record);
                let scattered_dir = pdf.generate();
                let scattered_ray = Ray::new(hit_record.point, scattered_dir, ray.time);
                let scattering_pdf = mat.scattering_pdf(ray, hit_record, &scattered_ray);
                let pdf_value = pdf.value(&scattered_dir);
                let sample_color = self.ray_color(&scattered_ray, depth + 1);

                scatter_record.attenuation * scattering_pdf * sample_color / pdf_value
            }
        }
    }

    fn get_pdf(&self, hit_record: &HitRecord, scatter_record: &ScatterRecord) -> Arc<Pdf> {
        if self.options.use_importance_sampling {
            if self.scene.lights.len() > 0 {
                let light_pdf = Arc::new(Pdf::Hittable(HittablePdf::new(
                    Arc::clone(&self.scene.lights),
                    hit_record.point,
                )));
                Arc::new(Pdf::Mixture(MixturePdf::new(
                    light_pdf,
                    Arc::clone(&scatter_record.pdf),
                )))
            } else {
                Arc::clone(&scatter_record.pdf)
            }
        } else {
            // Arc::new(Pdf::Hemisphere(HemispherePdf::new(hit_record.normal)))
            Arc::new(Pdf::Cosine(CosinePdf::new(&hit_record.normal)))
            // Arc::new(SpherePdf::new())
        }
    }
}
