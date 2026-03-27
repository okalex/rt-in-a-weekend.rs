use std::{sync::Arc, time::Instant};

use wesl::include_wesl;

use crate::{
    gpu::gpu::Gpu,
    rt::{
        camera::Camera,
        frame_buffer::FrameBuffer,
        geometry::scene::Scene,
        renderer::{
            gpu::{gpu_meta::GpuMeta, gpu_types::GpuScene, render_pipeline::RenderPipeline},
            render_options::RenderOptions,
        },
    },
    util::{color::Color, types::Float},
};

pub struct GpuRenderer {
    options: Arc<RenderOptions>,
    scene: Arc<Scene>,
    camera: Arc<Camera>,
    frame_buffer: Arc<FrameBuffer>,
    gpu: Arc<Gpu>,
}

impl GpuRenderer {
    const WORKGROUP_SIZE_X: u32 = 64u32;
    const WORKGROUP_SIZE_Y: u32 = 1u32;

    pub async fn new(
        options: Arc<RenderOptions>,
        scene: Arc<Scene>,
        camera: Arc<Camera>,
        frame_buffer: Arc<FrameBuffer>,
        gpu: Arc<Gpu>,
    ) -> anyhow::Result<Self> {
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
        let render_pipeline = self.setup_pipeline();
        let _ = render_pipeline.warmup().await; // Warm up to try to fix dispatch timeouts (doesn't seem to work)

        let workgroup_dims = [
            self.options.img_width.div_ceil(Self::WORKGROUP_SIZE_X),
            self.options.img_height.div_ceil(Self::WORKGROUP_SIZE_Y),
        ];

        // Render progressively in smaller dispatches
        for samples in 1..=(self.options.samples_per_pixel / self.options.dispatch_size) {
            // Update the frame number for rng seeding
            let gpu_meta = Arc::new(GpuMeta::new(
                Arc::clone(&self.options),
                Arc::clone(&self.camera),
                samples,
            ));
            render_pipeline.init_buf(&gpu_meta, &render_pipeline.meta_buf);

            // Render
            render_pipeline.dispatch(workgroup_dims);

            // Update frame buffer
            if samples % 50 == 1 {
                let result = render_pipeline.get_result().await;
                match result {
                    Ok(pixels) => {
                        let colors = self.collect_results(pixels);
                        self.write_result_to_frame(&colors)
                    }
                    _ => panic!(),
                }
            }
        }

        // Display final results
        let result = render_pipeline.get_result().await;
        match result {
            Ok(pixels) => {
                let colors = self.collect_results(pixels);
                self.write_result_to_frame(&colors)
            }
            _ => panic!(),
        }

        let elapsed = now.elapsed().as_millis();
        eprintln!("Done rendering: {}.{:0>3} s", elapsed / 1000, elapsed % 1000);
    }

    fn setup_pipeline(&self) -> RenderPipeline<[f32; 4]> {
        let compute_shader = match self.create_shader() {
            Ok(shader) => shader,
            Err(e) => {
                eprintln!("Error creating shader: {}", e);
                panic!();
            }
        };

        let gpu_meta = Arc::new(GpuMeta::new(Arc::clone(&self.options), Arc::clone(&self.camera), 0));
        let gpu_scene = Arc::new(GpuScene::from(&self.scene));

        let num_pixels = (self.options.img_height * self.options.img_width) as u32;
        let output_buf_size = (num_pixels * std::mem::size_of::<[f32; 4]>() as u32) as u64;

        RenderPipeline::<[f32; 4]>::new(
            Arc::clone(&self.gpu),
            &compute_shader,
            output_buf_size,
            Arc::clone(&gpu_meta),
            Arc::clone(&gpu_scene),
        )
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
