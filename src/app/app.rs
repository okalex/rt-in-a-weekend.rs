use std::sync::{atomic::Ordering, Arc};
use std::time::Duration;

use iced::widget::image;
use iced::{Element, Subscription, Task};
use tokio::sync::watch::Sender;

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
    pub render_options: RenderOptions,
    pub camera_options: CameraOptions,
    command_channel: Sender<RendererCommand>,
    pub renderer_state: Arc<RendererState>,
    pub samples_per_pixel: String,
    pub selected_scene_idx: usize,
    pub render_image: image::Handle,
}

#[derive(Debug, Clone)]
pub enum Message {
    RenderButtonClicked,
    SceneSelected(String),
    SamplesChanged(String),
    Tick,
}

impl App {
    pub fn new(args: Args) -> (Self, Task<Message>) {
        let render_options = get_render_options(&args);
        let scene_idx = args.scene;
        let camera_options = get_camera_options(scene_idx);

        let frame_buffer = Arc::new(FrameBuffer::new(
            render_options.img_width as usize,
            render_options.img_height as usize,
        ));

        let (tx, rx) = tokio::sync::watch::channel(RendererCommand::Idle);
        let renderer_state = Arc::new(RendererState::new());

        // Spawn renderer on background thread with its own tokio runtime
        let fb_clone = Arc::clone(&frame_buffer);
        let state_clone = Arc::clone(&renderer_state);
        let use_gpu = args.gpu;
        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let gpu = if use_gpu {
                    Some(Arc::new(Gpu::new().await.unwrap()))
                } else {
                    None
                };
                let mut renderer = Renderer::new(rx, fb_clone, gpu, state_clone).await;
                renderer.run().await;
            });
        });

        let width = render_options.img_width;
        let height = render_options.img_height;
        let render_image = image::Handle::from_rgba(width, height, vec![0u8; (width * height * 4) as usize]);

        let app = Self {
            frame_buffer,
            render_options,
            camera_options,
            command_channel: tx,
            renderer_state,
            samples_per_pixel: render_options.samples_per_pixel.to_string(),
            selected_scene_idx: (scene_idx - 1) as usize,
            render_image,
        };

        (app, Task::none())
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::RenderButtonClicked => {
                if self.is_rendering() {
                    let _ = self.command_channel.send(RendererCommand::CancelRender);
                } else {
                    let render_options = RenderOptions {
                        samples_per_pixel: self
                            .samples_per_pixel
                            .parse::<Uint>()
                            .unwrap_or(self.render_options.samples_per_pixel),
                        ..self.render_options
                    };
                    let _ = self.command_channel.send(RendererCommand::Render {
                        render_options,
                        camera_options: self.camera_options,
                        scene_idx: (self.selected_scene_idx + 1) as Uint,
                    });
                }
            }
            Message::SceneSelected(scene_name) => {
                if let Some(idx) = crate::app::ui::SCENES.iter().position(|&s| s == scene_name) {
                    self.selected_scene_idx = idx;
                }
            }
            Message::SamplesChanged(value) => {
                self.samples_per_pixel = value;
            }
            Message::Tick => {
                self.refresh_image();
            }
        }
        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        crate::app::ui::view(self)
    }

    pub fn subscription(&self) -> Subscription<Message> {
        iced::time::every(Duration::from_millis(33)).map(|_| Message::Tick)
    }

    fn refresh_image(&mut self) {
        let data = self.frame_buffer.data.lock().unwrap().clone();
        self.render_image =
            image::Handle::from_rgba(self.render_options.img_width, self.render_options.img_height, data);
    }

    pub fn is_rendering(&self) -> bool {
        self.renderer_state.is_rendering.load(Ordering::Relaxed)
    }
}
