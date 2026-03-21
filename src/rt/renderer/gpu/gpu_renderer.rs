use std::{
    sync::Arc,
    time::Instant,
};

use wesl::include_wesl;

use crate::rt::{
    camera::Camera,
    color::Color,
    frame_buffer::FrameBuffer,
    geometry::scene::Scene,
    gpu::{
        gpu::Gpu,
        gpu_compute::GpuCompute,
    },
    renderer::{
        gpu::gpu_types::{
            GpuBvh,
            GpuInstances,
            GpuMaterials,
            GpuMeta,
            GpuPrimitives,
        },
        render_options::RenderOptions,
    },
    types::Float,
};

pub struct GpuRenderer {
    options: Arc<RenderOptions>,
    scene: Arc<Scene>,
    camera: Arc<Camera>,
    frame_buffer: Arc<FrameBuffer>,
    gpu: Arc<Gpu>,
}

impl GpuRenderer {
    const WORKGROUP_SIZE: u32 = 16u32;

    pub async fn new(
        options: Arc<RenderOptions>,
        scene: Arc<Scene>,
        camera: Arc<Camera>,
        frame_buffer: Arc<FrameBuffer>,
    ) -> anyhow::Result<Self> {
        let gpu = Arc::new(Gpu::new_headless().await?);

        Ok(Self {
            options,
            scene,
            camera,
            frame_buffer,
            gpu,
        })
    }

    pub async fn render(&self) {
        let now = Instant::now();
        let gpu_compute = self.setup_pipeline();
        let _ = gpu_compute.warmup().await; // Warm up to try to fix dispatch timeouts (doesn't seem to work)

        let workgroup_dims = [
            self.options.img_width.div_ceil(Self::WORKGROUP_SIZE),
            self.options.img_height.div_ceil(Self::WORKGROUP_SIZE),
        ];

        let num_pixels = (self.options.img_width * self.options.img_height) as usize;
        let mut accum = vec![Color::black(); num_pixels];

        // Render progressively in smaller dispatches
        for samples in 1..=(self.options.samples_per_pixel / self.options.dispatch_size) {
            let gpu_meta = Arc::new(GpuMeta::new(Arc::clone(&self.options), Arc::clone(&self.camera), samples));
            gpu_compute.init_buf(&gpu_meta, &gpu_compute.meta_buf);

            let result = gpu_compute.dispatch(workgroup_dims).await;

            match result {
                Ok(pixels) => {
                    let colors = self.collect_results(pixels);
                    for idx in 0..num_pixels {
                        accum[idx] = (accum[idx] * (samples as f32 - 1.0) + colors[idx]) / (samples as f32);
                    }
                    self.write_result_to_frame(&accum)
                }
                _ => panic!(),
            }
        }

        let elapsed = now.elapsed().as_millis();
        eprintln!("Done rendering: {}.{} s", elapsed / 1000, elapsed % 1000);
    }

    fn setup_pipeline(&self) -> GpuCompute<[f32; 4]> {
        let compute_shader = match self.create_shader() {
            Ok(shader) => shader,
            Err(e) => {
                eprintln!("Error creating shader: {}", e);
                panic!();
            }
        };

        let gpu_meta = Arc::new(GpuMeta::new(Arc::clone(&self.options), Arc::clone(&self.camera), 0));
        let gpu_primitives = Arc::new(GpuPrimitives::new(Arc::clone(&self.scene)));
        let gpu_instances = Arc::new(GpuInstances::new(Arc::clone(&self.scene)));
        let gpu_materials = Arc::new(GpuMaterials::from(&self.scene.materials));
        let gpu_bvh = Arc::new(GpuBvh::from(&self.scene.bvh));

        let num_pixels = (self.options.img_height * self.options.img_width) as u32;
        let output_buf_size = (num_pixels * std::mem::size_of::<[f32; 4]>() as u32) as u64;

        let gpu_compute = GpuCompute::<[f32; 4]>::new(
            Arc::clone(&self.gpu),
            &compute_shader,
            output_buf_size,
            Arc::clone(&gpu_meta),
            Arc::clone(&gpu_primitives),
            Arc::clone(&gpu_instances),
            Arc::clone(&gpu_materials),
            Arc::clone(&gpu_bvh),
        );

        gpu_compute.init_bufs(gpu_meta, gpu_primitives, gpu_instances, gpu_materials, gpu_bvh);

        gpu_compute
    }

    fn create_shader(&self) -> anyhow::Result<wgpu::ShaderModule> {
        let shader = self.gpu.device().create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("compute.wgsl"),
            source: wgpu::ShaderSource::Wgsl(include_wesl!("render_shader").into()),
        });

        Ok(shader)
    }

    fn collect_results(&self, pixels: Vec<[f32; 4]>) -> Vec<Color> {
        pixels
            .iter()
            .map(|values| Color::new(values[0] as Float, values[1] as Float, values[2] as Float))
            .collect()
    }

    fn write_result_to_frame(&self, pixels: &Vec<Color>) {
        let colors: Vec<[u8; 3]> = pixels.iter().map(|color| color.to_gamma().to_u8()).collect();

        self.frame_buffer.set_frame(&colors);
    }
}
