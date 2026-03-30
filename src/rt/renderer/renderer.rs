use std::sync::Arc;

use tokio::sync::watch::Receiver;

use crate::{
    gpu::gpu::Gpu,
    rt::{
        camera::Camera,
        frame_buffer::FrameBuffer,
        geometry::scene::Scene,
        renderer::{
            cpu::cpu_renderer::CpuRenderer, gpu::gpu_renderer::GpuRenderer, render_options::RenderOptions,
            renderer_command::RendererCommand,
        },
    },
};

pub enum Renderer {
    Cpu(CpuRenderer),
    Gpu(GpuRenderer),
}

impl Renderer {
    pub async fn cpu(
        command_channel: Receiver<RendererCommand>,
        options: Arc<RenderOptions>,
        scene: Arc<Scene>,
        camera: Arc<Camera>,
        frame_buffer: Arc<FrameBuffer>,
    ) -> anyhow::Result<Renderer> {
        let renderer = CpuRenderer::new(command_channel, options, scene, camera, frame_buffer);
        Ok(Self::Cpu(renderer))
    }

    pub async fn gpu(
        command_channel: Receiver<RendererCommand>,
        options: Arc<RenderOptions>,
        scene: Arc<Scene>,
        camera: Arc<Camera>,
        frame_buffer: Arc<FrameBuffer>,
        gpu: Arc<Gpu>,
    ) -> anyhow::Result<Renderer> {
        let renderer = GpuRenderer::new(command_channel, options, scene, camera, frame_buffer, gpu).await?;
        Ok(Self::Gpu(renderer))
    }

    pub async fn new(
        command_channel: Receiver<RendererCommand>,
        render_options: Arc<RenderOptions>,
        scene: Arc<Scene>,
        camera: Arc<Camera>,
        frame_buffer: Arc<FrameBuffer>,
        gpu: Option<Arc<Gpu>>,
    ) -> anyhow::Result<Renderer> {
        match gpu {
            Some(gpu) => {
                Renderer::gpu(
                    command_channel,
                    Arc::clone(&render_options),
                    Arc::clone(&scene),
                    Arc::clone(&camera),
                    Arc::clone(&frame_buffer),
                    gpu,
                )
                .await
            }

            None => {
                Renderer::cpu(
                    command_channel,
                    Arc::clone(&render_options),
                    Arc::clone(&scene),
                    Arc::clone(&camera),
                    Arc::clone(&frame_buffer),
                )
                .await
            }
        }
    }

    pub async fn render(&self) {
        match self {
            Self::Cpu(cpu) => cpu.render().await,
            Self::Gpu(gpu) => gpu.render().await,
        }
    }
}
