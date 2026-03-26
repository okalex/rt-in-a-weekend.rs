use std::{sync::Arc, time::Instant};

use crate::{
    rt::{
        camera::Camera,
        frame_buffer::FrameBuffer,
        geometry::{hit_record::HitRecord, scene::Scene},
        materials::material::{Material, ScatterRecord},
        pdf::{Pdf, TransformedPrimitive},
        ray::Ray,
        renderer::{cpu::line_server::LineServer, render_options::RenderOptions},
    },
    util::{
        color::Color,
        interval::Interval,
        types::{Uint, INFINITY},
    },
};

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

        let elapsed = now.elapsed().as_millis();
        eprintln!("Done rendering: {}.{} s", elapsed / 1000, elapsed % 1000);
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

        let (instance_id, hit_record) = match self.scene.hit(ray, Interval::new(0.001, INFINITY)) {
            Some((instance_id, hit_record)) => (instance_id, hit_record),
            None => return self.options.background,
        };

        let material = match self.scene.get_material_for(&instance_id) {
            Some(mat) => mat,
            None => return Color::black(),
        };

        let emitted = material.emitted(ray, &hit_record);

        let scattered_color = if let Some(scatter_record) = material.scatter(ray, &hit_record) {
            self.scatter_color(ray, depth, &hit_record, &scatter_record, material)
        } else {
            Color::black()
        };

        emitted + scattered_color
    }

    fn scatter_color(&self, ray: &Ray, depth: Uint, hit_record: &HitRecord, scatter_record: &ScatterRecord, mat: &Material) -> Color {
        if let Some(skip_pdf_ray) = &scatter_record.skip_pdf_ray {
            return scatter_record.attenuation * self.ray_color(&skip_pdf_ray, depth + 1);
        }

        let material_pdf = scatter_record.pdf.as_ref().unwrap(); // TODO: guard against None
        let pdf = self.get_pdf(hit_record, material_pdf);
        let scattered_dir = pdf.generate();

        let scattered_ray = Ray::new(hit_record.point, scattered_dir, ray.time); // Should time = ray.time + hit_record.t?
        let sample_color = self.ray_color(&scattered_ray, depth + 1);

        // Abort before pdf calculation if ray isn't contributing
        if sample_color.is_black() {
            return Color::black();
        }

        let pdf_value = pdf.value(&scattered_dir);
        if pdf_value < 1e-8 {
            return Color::black();
        }

        let cos_theta = hit_record.normal.dot(scattered_dir.normalize()).max(0.0);
        let brdf = mat.brdf(ray, hit_record, &scattered_dir);

        brdf * sample_color * cos_theta / pdf_value
    }

    fn get_pdf(&self, hit_record: &HitRecord, material_pdf: &Arc<Pdf>) -> Arc<Pdf> {
        let use_importance_sampling = self.options.use_importance_sampling && self.scene.lights.len() > 0;

        if !use_importance_sampling {
            return Arc::clone(&material_pdf);
        }

        let primitives: Vec<_> = self
            .scene
            .lights
            .iter()
            .flat_map(|instance_id| {
                let instance = self.scene.get_instance(instance_id)?;
                let primitive = self.scene.get_primitive_for(instance_id)?.clone();
                Some(TransformedPrimitive {
                    primitive,
                    transform: instance.transform,
                    inv_transform: instance.inv_transform,
                })
            })
            .collect();

        let light_pdf = Arc::new(Pdf::multi(hit_record.point, primitives));
        Arc::new(Pdf::mixture(light_pdf, Arc::clone(&material_pdf)))
    }
}
