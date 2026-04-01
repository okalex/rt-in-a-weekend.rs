use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

use tokio::sync::watch::Receiver;

use crate::{
    examples::scenes::get_scene,
    gpu::gpu::Gpu,
    rt::{
        camera::{Camera, CameraOptions},
        frame_buffer::FrameBuffer,
        geometry::scene::Scene,
        renderer::{
            cpu::cpu_renderer::CpuRenderer, gpu::gpu_renderer::GpuRenderer, render_options::RenderOptions,
            renderer_command::RendererCommand,
        },
    },
    util::types::Uint,
};

pub struct RendererState {
    pub is_rendering: AtomicBool,
}

impl RendererState {
    pub fn new() -> Self {
        Self {
            is_rendering: AtomicBool::new(false),
        }
    }
}

pub struct Renderer {
    command_channel: Receiver<RendererCommand>,
    frame_buffer: Arc<FrameBuffer>,
    gpu: Option<Arc<Gpu>>,
    state: Arc<RendererState>,
}

impl Renderer {
    pub async fn new(
        command_channel: Receiver<RendererCommand>,
        frame_buffer: Arc<FrameBuffer>,
        gpu: Option<Arc<Gpu>>,
        state: Arc<RendererState>,
    ) -> Self {
        Self {
            command_channel,
            frame_buffer,
            gpu,
            state,
        }
    }

    pub async fn run(&mut self) {
        loop {
            match self.command_channel.changed().await {
                Ok(()) => {
                    let command = *self.command_channel.borrow_and_update();
                    let _ = self.handle_command(command).await;
                }

                Err(_) => break,
            }
        }
    }

    pub async fn handle_command(&self, command: RendererCommand) -> anyhow::Result<()> {
        match command {
            RendererCommand::Idle => Ok(()),

            RendererCommand::Render {
                render_options,
                camera_options,
                scene_idx,
            } => self.handle_render(render_options, camera_options, scene_idx).await,

            RendererCommand::CancelRender => Ok(()),
        }
    }

    pub async fn handle_render(
        &self,
        render_options: RenderOptions,
        camera_options: CameraOptions,
        scene_idx: Uint,
    ) -> anyhow::Result<()> {
        self.state.is_rendering.store(true, Ordering::Relaxed);

        let scene = Arc::new(get_scene(scene_idx));
        let camera = Arc::new(Camera::new(&render_options, &camera_options));
        let render_options = Arc::new(render_options);

        if render_options.use_gpu {
            let renderer = if let Some(gpu) = &self.gpu {
                Self::gpu(
                    self.command_channel.clone(),
                    render_options,
                    scene,
                    camera,
                    Arc::clone(&self.frame_buffer),
                    Arc::clone(gpu),
                )
                .await?
            } else {
                self.state.is_rendering.store(false, Ordering::Relaxed);
                panic!("Can't use GPU renderer without a GPU")
            };

            renderer.render().await
        } else {
            let renderer = Self::cpu(
                self.command_channel.clone(),
                render_options,
                scene,
                camera,
                Arc::clone(&self.frame_buffer),
            )
            .await?;

            renderer.render().await
        }

        self.state.is_rendering.store(false, Ordering::Relaxed);
        Ok(())
    }

    async fn cpu(
        command_channel: Receiver<RendererCommand>,
        options: Arc<RenderOptions>,
        scene: Arc<Scene>,
        camera: Arc<Camera>,
        frame_buffer: Arc<FrameBuffer>,
    ) -> anyhow::Result<CpuRenderer> {
        let renderer = CpuRenderer::new(command_channel, options, scene, camera, frame_buffer);
        Ok(renderer)
    }

    async fn gpu(
        command_channel: Receiver<RendererCommand>,
        options: Arc<RenderOptions>,
        scene: Arc<Scene>,
        camera: Arc<Camera>,
        frame_buffer: Arc<FrameBuffer>,
        gpu: Arc<Gpu>,
    ) -> anyhow::Result<GpuRenderer> {
        let renderer = GpuRenderer::new(command_channel, options, scene, camera, frame_buffer, gpu).await?;
        Ok(renderer)
    }
}
