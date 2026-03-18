use std::sync::Arc;

use wesl::include_wesl;

use crate::rt::{
    camera::Camera,
    color::Color,
    frame_buffer::FrameBuffer,
    gpu::{gpu::Gpu, gpu_compute::GpuCompute},
    objects::scene::Scene,
    renderer::{
        gpu::gpu_types::{GpuMaterials, GpuMeta, GpuObjects},
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
        let gpu_compute = self.setup_pipeline();

        let workgroup_dims = [
            self.options.img_width.div_ceil(Self::WORKGROUP_SIZE),
            self.options.img_height.div_ceil(Self::WORKGROUP_SIZE),
        ];
        let result = gpu_compute.dispatch(workgroup_dims).await;

        match result {
            Ok(pixels) => self.write_result_to_frame(pixels),
            _ => panic!(),
        }
    }

    fn setup_pipeline(&self) -> GpuCompute<[f32; 4]> {
        let compute_shader = match self.create_shader() {
            Ok(shader) => shader,
            Err(e) => {
                eprint!("Error creating shader: {}", e);
                panic!();
            }
        };

        let gpu_meta = Arc::new(GpuMeta::new(
            Arc::clone(&self.options),
            Arc::clone(&self.camera),
        ));
        let gpu_objects = Arc::new(GpuObjects::new(Arc::clone(&self.scene)));
        let gpu_materials = Arc::new(GpuMaterials::from(&self.scene.materials));

        let num_pixels = (self.options.img_height * self.options.img_width) as u32;
        let output_buf_size = (num_pixels * std::mem::size_of::<[f32; 4]>() as u32) as u64;

        let gpu_compute = GpuCompute::<[f32; 4]>::new(
            Arc::clone(&self.gpu),
            &compute_shader,
            output_buf_size,
            Arc::clone(&gpu_meta),
            Arc::clone(&gpu_objects),
            Arc::clone(&gpu_materials),
        );

        gpu_compute.init_bufs(gpu_meta, gpu_objects, gpu_materials);

        gpu_compute
    }

    fn create_shader(&self) -> anyhow::Result<wgpu::ShaderModule> {
        // let mut composer = Composer::default();

        // composer.add_composable_module(ComposableModuleDescriptor {
        //     source: include_str!("../../../shaders/types.wgsl"),
        //     file_path: "../../../shaders/types.wgsl",
        //     ..Default::default()
        // })?;
        // let module = composer.make_naga_module(NagaModuleDescriptor {
        //     source: include_str!("../../../shaders/compute.wgsl"),
        //     file_path: "../../../shaders/compute.wgsl",
        //     ..Default::default()
        // })?;
        // let shader = self
        //     .gpu
        //     .device()
        //     .create_shader_module(wgpu::ShaderModuleDescriptor {
        //         label: Some("compute.wgsl"),
        //         source: wgpu::ShaderSource::Naga(std::borrow::Cow::Owned(module)),
        //     });

        let shader = self
            .gpu
            .device()
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("compute.wgsl"),
                source: wgpu::ShaderSource::Wgsl(include_wesl!("render_shader").into()),
            });
        // .create_shader_module(include_wesl!("compute_shader"));

        Ok(shader)
    }

    fn write_result_to_frame(&self, pixels: Vec<[f32; 4]>) {
        let colors: Vec<[u8; 3]> = pixels
            .iter()
            .map(|values| {
                let color = Color::new(values[0] as Float, values[1] as Float, values[2] as Float);
                color.to_gamma().to_u8()
            })
            .collect();

        eprintln!("Received data: {} pixels", colors.len());
        self.frame_buffer.set_frame(&colors);
    }
}
