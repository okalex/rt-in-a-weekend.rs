use std::sync::Arc;

use naga_oil::compose::{ComposableModuleDescriptor, Composer, NagaModuleDescriptor};

use crate::rt::{
    camera::Camera,
    color::Color,
    frame_buffer::FrameBuffer,
    gpu::{gpu::Gpu, gpu_compute::GpuCompute},
    objects::scene::Scene,
    renderer::{gpu::gpu_types::GpuMeta, render_options::RenderOptions},
    types::Float,
};

pub struct GpuRenderer {
    options: Arc<RenderOptions>,
    scene: Arc<Scene>,
    camera: Arc<Camera>,
    frame_buffer: Arc<FrameBuffer>,
    gpu_compute: GpuCompute<[u32; 2], [f32; 4]>,
}

impl GpuRenderer {
    pub async fn new(
        options: Arc<RenderOptions>,
        scene: Arc<Scene>,
        camera: Arc<Camera>,
        frame_buffer: Arc<FrameBuffer>,
    ) -> anyhow::Result<Self> {
        let gpu = Arc::new(Gpu::new_headless().await?);
        let compute_shader = Self::create_shader(Arc::clone(&gpu))?;

        let num_pixels = (options.img_height * options.img_width) as u32;
        let output_buf_size = (num_pixels * std::mem::size_of::<[f32; 4]>() as u32) as u64;
        let gpu_compute = GpuCompute::new(gpu, &compute_shader, output_buf_size);

        Ok(Self {
            options,
            scene,
            camera,
            frame_buffer,
            gpu_compute,
        })
    }

    fn create_shader(gpu: Arc<Gpu>) -> anyhow::Result<wgpu::ShaderModule> {
        let mut composer = Composer::default();

        composer.add_composable_module(ComposableModuleDescriptor {
            source: include_str!("../../../shaders/types.wgsl"),
            file_path: "../../../shaders/types.wgsl",
            ..Default::default()
        })?;
        let module = composer.make_naga_module(NagaModuleDescriptor {
            source: include_str!("../../../shaders/compute.wgsl"),
            file_path: "../../../shaders/compute.wgsl",
            ..Default::default()
        })?;
        let shader = gpu
            .device()
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("compute.wgsl"),
                source: wgpu::ShaderSource::Naga(std::borrow::Cow::Owned(module)),
            });

        Ok(shader)
    }

    pub async fn render(&self) {
        let width = self.options.img_width as usize;
        let height = self.options.img_height as usize;

        let gpu_meta = GpuMeta::new(Arc::clone(&self.options), Arc::clone(&self.camera));
        self.gpu_compute.set_meta(gpu_meta);

        let workgroup_size = 16u32;
        let workgroup_dims = [
            (width as u32).div_ceil(workgroup_size),
            (height as u32).div_ceil(workgroup_size),
        ];
        let result = self.gpu_compute.dispatch(workgroup_dims).await;

        match result {
            Ok(pixels) => {
                let colors: Vec<[u8; 3]> = pixels
                    .iter()
                    .map(|values| {
                        let color =
                            Color::new(values[0] as Float, values[1] as Float, values[2] as Float);
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
