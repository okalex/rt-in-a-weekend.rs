use std::sync::Arc;

use crate::rt::{
    camera::Camera,
    frame_buffer::FrameBuffer,
    objects::scene::Scene,
    renderer::{
        cpu_renderer::{CpuRenderer, LineServer, RenderOptions},
        gpu_renderer::GpuRenderer,
    },
};

pub enum Renderer {
    Cpu(CpuRenderer),
    Gpu(GpuRenderer),
}

impl Renderer {
    pub async fn cpu(
        options: Arc<RenderOptions>,
        scene: Arc<Scene>,
        camera: Arc<Camera>,
        frame_buffer: Arc<FrameBuffer>,
        line_server: Arc<LineServer>,
    ) -> anyhow::Result<Renderer> {
        let renderer = CpuRenderer::new(options, scene, camera, frame_buffer, line_server);
        Ok(Self::Cpu(renderer))
    }

    pub async fn gpu(
        options: Arc<RenderOptions>,
        scene: Arc<Scene>,
        camera: Arc<Camera>,
        frame_buffer: Arc<FrameBuffer>,
    ) -> anyhow::Result<Renderer> {
        let renderer = GpuRenderer::new(options, scene, camera, frame_buffer).await?;
        Ok(Self::Gpu(renderer))
    }

    pub async fn render(&self) {
        match self {
            Self::Cpu(cpu) => cpu.render().await,
            Self::Gpu(gpu) => gpu.render().await,
        }
    }
}
