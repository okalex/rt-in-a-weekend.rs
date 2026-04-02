use std::sync::{atomic::Ordering, Arc};
use std::time::Duration;

use iced::{Element, Subscription, Task};
use tokio::sync::watch::Sender;

use crate::app::ui::UiState;
use crate::{
    app::cli::Args,
    examples::scenes::get_camera_options,
    get_render_options,
    gpu::gpu::Gpu,
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

pub struct App {
    pub frame_buffer: Arc<FrameBuffer>,
    pub render_options: RenderOptions, // TODO: Move all this into UI state
    pub camera_options: CameraOptions, // TODO: Move all this into UI state
    command_channel: Sender<RendererCommand>,
    pub renderer_state: Arc<RendererState>,
    ui_state: UiState,
}

#[derive(Debug, Clone)]
pub enum Message {
    RenderButtonClicked,
    SceneSelected(Uint),
    SamplesChanged(String),
    MaxDepthChanged(String),
    Tick,
    CloseRequested,
}

impl App {
    pub fn new(args: Args) -> (Self, Task<Message>) {
        let render_options = get_render_options(&args);
        let scene_idx = args.scene;
        let camera_options = get_camera_options(scene_idx);

        // Set up frame buffer and UI image
        // TODO: generate this at render time to accommodate render size changes
        let frame_buffer = Arc::new(FrameBuffer::new(
            render_options.img_width as usize,
            render_options.img_height as usize,
        ));

        // Set up renderer command channel
        let (tx, rx) = tokio::sync::watch::channel(RendererCommand::Idle);
        let renderer_state = Arc::new(RendererState::new());

        // Spawn renderer
        let fb_clone = Arc::clone(&frame_buffer);
        let state_clone = Arc::clone(&renderer_state);
        let use_gpu = args.gpu;
        let _ = tokio::spawn(async move {
            let gpu = if use_gpu {
                Some(Arc::new(Gpu::new().await.unwrap()))
            } else {
                None
            };
            let mut renderer = Renderer::new(rx, fb_clone, gpu, state_clone).await;
            renderer.run().await
        });

        let ui_state = UiState::new(render_options, (scene_idx as Uint) - 1);

        let app = Self {
            frame_buffer,
            render_options,
            camera_options,
            command_channel: tx,
            renderer_state,
            ui_state,
        };

        (app, Task::none())
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::RenderButtonClicked => {
                if self.is_rendering() {
                    let _ = self.command_channel.send(RendererCommand::CancelRender);
                } else {
                    let scene_idx = self.get_scene_idx();
                    let _ = self.command_channel.send(RendererCommand::Render {
                        render_options: self.build_render_options(),
                        camera_options: self.camera_options,
                        scene_idx,
                    });
                }
            }

            Message::SceneSelected(scene_idx) => {
                if scene_idx != self.ui_state.scene_idx {
                    self.camera_options = get_camera_options(scene_idx + 1);
                    self.ui_state.update_scene_idx(scene_idx);
                }
            }

            Message::SamplesChanged(value) => {
                self.ui_state.update_samples_per_pixel(value);
            }

            Message::MaxDepthChanged(value) => {
                self.ui_state.update_max_depth(value);
            }

            Message::Tick => {
                self.ui_state.update_is_rendering(self.is_rendering());
                self.refresh_image();
            }

            Message::CloseRequested => {
                log::info!("Exiting app...");
                if self.is_rendering() {
                    log::info!("Stopping renderer.");
                    let _ = self.command_channel.send(RendererCommand::CancelRender);
                }
                return iced::exit();
            }
        }
        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        crate::app::ui::view(&self.ui_state)
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([
            iced::time::every(Duration::from_millis(33)).map(|_| Message::Tick),
            iced::window::close_requests().map(|_| Message::CloseRequested),
        ])
    }

    fn refresh_image(&mut self) {
        let data = self.frame_buffer.data.lock().unwrap().clone();
        self.ui_state.update_render_image(data);
    }

    fn is_rendering(&self) -> bool {
        self.renderer_state.is_rendering.load(Ordering::Relaxed)
    }

    fn build_render_options(&self) -> RenderOptions {
        RenderOptions {
            samples_per_pixel: self.ui_state.get_samples_per_pixel(),
            max_depth: self.ui_state.get_max_depth(),
            ..self.render_options
        }
    }

    fn get_scene_idx(&self) -> Uint {
        self.ui_state.scene_idx + 1
    }
}
