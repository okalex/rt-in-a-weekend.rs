use std::sync::{
    Arc,
    atomic::{AtomicBool, AtomicU64, Ordering},
};

use tokio::sync::watch;

use crate::{
    examples::scenes::get_scene,
    gpu::gpu::Gpu,
    rt::{
        camera::{Camera, CameraOptions},
        frame_buffer::FrameBuffer,
        renderer::{
            cpu::cpu_renderer::CpuRenderer, gpu::gpu_renderer::GpuRenderer, render_options::RenderOptions,
            renderer_command::RendererCommand,
        },
    },
    util::types::Uint,
};

pub struct RendererState {
    pub is_rendering: AtomicBool,
    pub render_idx: Arc<AtomicU64>,
}

impl RendererState {
    pub fn new() -> Self {
        Self {
            is_rendering: AtomicBool::new(false),
            render_idx: Arc::new(AtomicU64::new(0)),
        }
    }
}

pub struct Renderer {
    renderer_commands: watch::Receiver<RendererCommand>,
    frame_buffer: Arc<FrameBuffer>,
    gpu: Arc<Gpu>,
    state: Arc<RendererState>,
}

impl Renderer {
    pub async fn new(
        renderer_commands: watch::Receiver<RendererCommand>,
        frame_buffer: Arc<FrameBuffer>,
        gpu: Arc<Gpu>,
        state: Arc<RendererState>,
    ) -> Self {
        Self {
            renderer_commands,
            frame_buffer,
            gpu,
            state,
        }
    }

    pub async fn run(&mut self) {
        loop {
            match self.renderer_commands.changed().await {
                Ok(()) => {
                    let command = *self.renderer_commands.borrow_and_update();
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
            let renderer = GpuRenderer::new(
                self.renderer_commands.clone(),
                render_options,
                scene,
                camera,
                Arc::clone(&self.frame_buffer),
                Arc::clone(&self.gpu),
                Arc::clone(&self.state),
            )
            .await?;
            renderer.render().await
        } else {
            let renderer = CpuRenderer::new(
                self.renderer_commands.clone(),
                render_options,
                scene,
                camera,
                Arc::clone(&self.frame_buffer),
                Arc::clone(&self.state),
            );
            renderer.render().await
        }

        self.state.is_rendering.store(false, Ordering::Relaxed);
        Ok(())
    }
}
