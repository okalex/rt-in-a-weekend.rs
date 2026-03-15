use std::sync::Arc;

use winit::{event_loop::ActiveEventLoop, keyboard::KeyCode, window::Window};

use crate::rt::{
    frame_buffer::FrameBuffer,
    gpu::{gpu::Gpu, gpu_texture::GpuTexture},
};

pub struct State {
    pub window: Arc<Window>,
    frame_buffer: Arc<FrameBuffer>,
    gpu: Gpu,
    texture: GpuTexture,
    bind_group: wgpu::BindGroup,
    render_pipeline: wgpu::RenderPipeline,
}

impl State {
    pub async fn new(window: Arc<Window>, frame_buffer: Arc<FrameBuffer>) -> anyhow::Result<Self> {
        let gpu = Gpu::init(Arc::clone(&window)).await?;

        let texture = GpuTexture::new(&gpu.device, Arc::clone(&frame_buffer));
        let bind_group_layout = gpu.create_bind_group_layout(&texture.bind_group_layout_entries(0));
        let bind_group = gpu.create_bind_group(&bind_group_layout, &texture.bind_group_entries(0));
        let render_pipeline = gpu.create_render_pipeline(&[&bind_group_layout]);

        Ok(Self {
            window,
            frame_buffer,
            gpu,
            texture,
            bind_group,
            render_pipeline,
        })
    }

    pub fn update(&mut self) {}

    pub fn resize(&mut self, width: u32, height: u32) {
        self.gpu.resize(width, height);
    }

    pub fn handle_key(&mut self, event_loop: &ActiveEventLoop, code: KeyCode, is_pressed: bool) {
        if code == KeyCode::Escape && is_pressed {
            event_loop.exit();
        } else {
            // self.camera_controller.handle_key(code, is_pressed);
        }
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.window.request_redraw();
        if !self.gpu.is_surface_configured {
            return Ok(());
        }

        // Copy data to GPU
        self.gpu.write_texture(Arc::clone(&self.frame_buffer), &self.texture);
        self.gpu.render(&self.render_pipeline, &self.bind_group);

        Ok(())
    }
}
