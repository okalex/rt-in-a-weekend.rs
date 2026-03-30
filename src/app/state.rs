use std::sync::Arc;

use tokio::sync::watch::Sender;
use winit::{event_loop::ActiveEventLoop, keyboard::KeyCode, window::Window};

use crate::{
    app::{egui_integration::EguiIntegration, ui},
    gpu::{gpu::Gpu, gpu_canvas::GpuCanvas},
    rt::{
        camera::{Camera, CameraOptions},
        frame_buffer::FrameBuffer,
        geometry::scene::Scene,
        renderer::{render_options::RenderOptions, renderer::Renderer, renderer_command::RendererCommand},
    },
    util::types::Uint,
};

#[allow(unused)]
pub struct State {
    pub window: Arc<Window>,
    pub egui: EguiIntegration,
    render_texture_id: egui::TextureId,
    frame_buffer: Arc<FrameBuffer>,
    gpu: Arc<Gpu>,
    canvas: GpuCanvas,
    render_options: Arc<RenderOptions>,
    camera_options: Arc<CameraOptions>,
    scene: Arc<Scene>,
    renderer: Arc<Renderer>,
    command_channel: Sender<RendererCommand>,
}

impl State {
    pub async fn new(
        window: Arc<Window>,
        render_options: RenderOptions,
        camera_options: CameraOptions,
        scene: Scene,
    ) -> anyhow::Result<Self> {
        let gpu = Arc::new(Gpu::new_windowed(Arc::clone(&window)).await?);

        let mut egui = EguiIntegration::new(&gpu, &window);

        let frame_buffer = Arc::new(FrameBuffer::new(
            render_options.img_width as usize,
            render_options.img_height as usize,
        ));
        let canvas = GpuCanvas::new(gpu.device(), Arc::clone(&frame_buffer));

        let render_texture_id = egui.register_texture(gpu.device(), canvas.view(), wgpu::FilterMode::Linear);

        let (tx, rx) = tokio::sync::watch::channel(RendererCommand::Render);

        let render_options = Arc::new(render_options);
        let scene = Arc::new(scene);
        let camera = Arc::new(Camera::new(&render_options, &camera_options));
        let use_gpu = render_options.use_gpu;
        let renderer = Arc::new(
            Renderer::new(
                rx,
                Arc::clone(&render_options),
                Arc::clone(&scene),
                Arc::clone(&camera),
                Arc::clone(&frame_buffer),
                if use_gpu { Some(Arc::clone(&gpu)) } else { None },
            )
            .await?,
        );

        let renderer_clone = Arc::clone(&renderer);
        let _ = tokio::spawn(async move {
            renderer_clone.render().await;
        });

        Ok(Self {
            window,
            egui,
            render_texture_id,
            frame_buffer,
            gpu,
            canvas,
            render_options,
            camera_options: Arc::new(camera_options),
            scene,
            renderer: Arc::clone(&renderer),
            command_channel: tx,
        })
    }

    pub fn update(&mut self) {}

    pub fn resize(&mut self, width: Uint, height: Uint) {
        self.gpu.resize(width, height);
    }

    pub fn handle_key(&mut self, _event_loop: &ActiveEventLoop, code: KeyCode, is_pressed: bool) {
        if code == KeyCode::Escape && is_pressed {
            let _ = self.command_channel.send(RendererCommand::CancelRender);
            // std::process::exit(0);
        }
    }

    pub fn render(&mut self) -> anyhow::Result<()> {
        self.window.request_redraw();

        if !self.gpu.is_ready() {
            return Ok(());
        }

        // Copy ray tracer output to GPU texture
        self.gpu.write_texture(Arc::clone(&self.frame_buffer), &self.canvas);

        self.egui.update_texture(
            self.gpu.device(),
            self.canvas.view(),
            wgpu::FilterMode::Linear,
            self.render_texture_id,
        );

        let frame = match self.gpu.get_current_texture() {
            Some(frame) => frame,
            None => return Ok(()),
        };

        let render_texture_id = self.render_texture_id;
        let frame_buffer = &self.frame_buffer;
        self.egui.render_ui(&self.gpu, &self.window, frame, |ui| {
            ui::build_ui(ui, render_texture_id, frame_buffer);
        });

        Ok(())
    }
}
