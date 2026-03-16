use std::sync::Arc;

use crate::rt::{
    camera::Camera,
    color::Color,
    frame_buffer::FrameBuffer,
    gpu::{gpu::Gpu, gpu_compute::GpuCompute},
    objects::scene::Scene,
    renderer::cpu_renderer::RenderOptions,
};

pub struct GpuRenderer {
    options: Arc<RenderOptions>,
    scene: Arc<Scene>,
    camera: Arc<Camera>,
    frame_buffer: Arc<FrameBuffer>,
    gpu_compute: GpuCompute<[u32; 2], [f32; 3]>,
}

impl GpuRenderer {
    pub async fn new(
        options: Arc<RenderOptions>,
        scene: Arc<Scene>,
        camera: Arc<Camera>,
        frame_buffer: Arc<FrameBuffer>,
    ) -> anyhow::Result<Self> {
        let gpu = Arc::new(Gpu::new_headless().await?);
        let compute_shader = gpu.create_shader(wgpu::include_wgsl!("compute.wgsl"));

        let num_pixels = options.img_height * options.img_width;
        let output_buf_size = (num_pixels * std::mem::size_of::<[f32; 3]>() as u32) as u64;
        let gpu_compute = GpuCompute::new(gpu, &compute_shader, output_buf_size);

        gpu_compute.set_dims([options.img_width, options.img_height]);

        Ok(Self {
            options,
            scene,
            camera,
            frame_buffer,
            gpu_compute,
        })
    }

    pub async fn render(&self) {
        let width = self.options.img_width as usize;
        let height = self.options.img_height as usize;

        let workgroup_size = 16u32;
        let workgroup_dims = [
            (width as u32).div_ceil(workgroup_size),
            (height as u32).div_ceil(workgroup_size),
            1,
        ];
        let result = self.gpu_compute.dispatch(workgroup_dims).await;

        match result {
            Ok(pixels) => {
                let colors: Vec<[u8; 3]> = pixels
                    .iter()
                    .map(|values| {
                        let color =
                            Color::new(values[0] as f64, values[1] as f64, values[2] as f64);
                        color.to_gamma().to_u8()
                    })
                    .collect();

                eprintln!("Received data: {} pixels", colors.len());
                self.frame_buffer.set_frame(&colors);
            }

            _ => panic!(),
        }
    }
}
