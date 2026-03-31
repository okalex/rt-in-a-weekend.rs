use std::sync::{atomic::Ordering, Arc};

use tokio::sync::watch::Sender;
use winit::{event_loop::ActiveEventLoop, keyboard::KeyCode, window::Window};

use crate::{
    app::{
        egui_integration::EguiIntegration,
        ui::{self, UiAction, UiState},
    },
    gpu::{gpu::Gpu, gpu_canvas::GpuCanvas},
    rt::{
        camera::CameraOptions,
        frame_buffer::FrameBuffer,
        renderer::{
            render_options::RenderOptions,
            renderer::{Renderer, RendererState},
            renderer_command::RendererCommand,
        },
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
    render_options: RenderOptions,
    camera_options: CameraOptions,
    scene_idx: Uint,
    command_channel: Sender<RendererCommand>,
    renderer_state: Arc<RendererState>,
    ui_state: UiState,
}

impl State {
    pub async fn new(
        window: Arc<Window>,
        render_options: RenderOptions,
        camera_options: CameraOptions,
        scene_idx: Uint,
    ) -> anyhow::Result<Self> {
        let gpu = Arc::new(Gpu::new_windowed(Arc::clone(&window)).await?);
        let mut egui = EguiIntegration::new(&gpu, &window);

        // Set up renderer display
        let frame_buffer = Arc::new(FrameBuffer::new(
            render_options.img_width as usize,
            render_options.img_height as usize,
        ));
        let canvas = GpuCanvas::new(gpu.device(), Arc::clone(&frame_buffer));
        let render_texture_id = egui.register_texture(gpu.device(), canvas.view(), wgpu::FilterMode::Linear);

        // Set up renderer
        let (tx, rx) = tokio::sync::watch::channel(RendererCommand::Idle);
        let renderer_state = Arc::new(RendererState::new());
        let mut renderer = Renderer::new(
            rx,
            Arc::clone(&frame_buffer),
            Some(Arc::clone(&gpu)),
            Arc::clone(&renderer_state),
        )
        .await;
        let _ = tokio::spawn(async move {
            renderer.run().await;
        });

        // Initialize UI state
        let ui_state = UiState {
            render_texture_id,
            render_width: render_options.img_width,
            render_height: render_options.img_height,
            is_rendering: renderer_state.is_rendering.load(Ordering::Relaxed),
            samples_per_pixel: render_options.samples_per_pixel.to_string(),
            selected_scene_idx: (scene_idx - 1) as usize,
        };

        Ok(Self {
            window,
            egui,
            render_texture_id,
            frame_buffer,
            gpu,
            canvas,
            render_options: render_options,
            camera_options: camera_options,
            scene_idx,
            command_channel: tx,
            renderer_state,
            ui_state,
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

        // Update UI state with renderer state
        self.update_ui_state();

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

        // Display UI
        let ui_state = &mut self.ui_state;
        let mut ui_actions: Vec<UiAction> = vec![];
        self.egui.render_ui(&self.gpu, &self.window, frame, |ui| {
            ui_actions = ui::build_ui(ui, ui_state);
        });

        // Handle user input
        for action in ui_actions {
            self.handle_ui_action(action);
        }

        Ok(())
    }

    fn handle_ui_action(&self, action: UiAction) {
        match action {
            UiAction::RenderButtonClicked => {
                if self.renderer_state.is_rendering.load(Ordering::Relaxed) {
                    self.cancel_render();
                } else {
                    self.start_render();
                }
            }
        };
    }

    fn start_render(&self) {
        let _ = self.command_channel.send(RendererCommand::Render {
            render_options: self.build_render_options(),
            camera_options: self.camera_options,
            scene_idx: (self.ui_state.selected_scene_idx + 1) as Uint,
        });
    }

    fn update_ui_state(&mut self) {
        self.ui_state.is_rendering = self.renderer_state.is_rendering.load(Ordering::Relaxed);
    }

    fn build_render_options(&self) -> RenderOptions {
        RenderOptions {
            img_width: self.render_options.img_width,
            img_height: self.render_options.img_height,
            samples_per_pixel: self.ui_state.samples_per_pixel.parse::<Uint>().unwrap(),
            dispatch_size: self.render_options.dispatch_size,
            max_depth: self.render_options.max_depth,
            use_gpu: self.render_options.use_gpu,
            use_multithreading: self.render_options.use_multithreading,
            use_importance_sampling: self.render_options.use_importance_sampling,
            background: self.render_options.background,
            sampler_type: self.render_options.sampler_type,
        }
    }

    fn cancel_render(&self) {
        let _ = self.command_channel.send(RendererCommand::CancelRender);
    }
}
