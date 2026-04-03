use std::sync::Arc;
use std::sync::atomic::AtomicU64;

use iced::widget::{button, checkbox, column, container, pick_list, row, shader, text, text_input};
use iced::{Element, Fill};

use crate::app::app::Message;
use crate::app::preview_widget::PreviewWidget;
use crate::rt::frame_buffer::FrameBuffer;
use crate::rt::renderer::render_options::RenderOptions;
use crate::util::types::Uint;

pub const SCENES: &[&str] = &[
    "Spheres",
    "Marbles",
    "Cornell Room",
    "Cornell Room w/ Smoke",
    "Triangles",
    "Mesh",
    "RTiaW Book 2 Final",
    "PBR",
];

pub struct UiState {
    is_rendering: bool,
    render_width: Uint,
    render_height: Uint,
    samples_per_pixel: String,
    max_depth: String,
    pub scene_idx: Uint,
    pub use_gpu: bool,
    pub use_importance_sampling: bool,
    frame_buffer: Arc<FrameBuffer>,
    render_idx: Arc<AtomicU64>,
}

impl UiState {
    pub fn new(
        render_options: RenderOptions,
        scene_idx: Uint,
        frame_buffer: Arc<FrameBuffer>,
        render_idx: Arc<AtomicU64>,
    ) -> Self {
        let render_width = render_options.img_width;
        let render_height = render_options.img_height;

        Self {
            is_rendering: false,
            render_width,
            render_height,
            samples_per_pixel: render_options.samples_per_pixel.to_string(),
            max_depth: render_options.max_depth.to_string(),
            scene_idx,
            use_gpu: render_options.use_gpu,
            use_importance_sampling: render_options.use_importance_sampling,
            frame_buffer,
            render_idx,
        }
    }

    pub fn update_is_rendering(&mut self, new_value: bool) {
        self.is_rendering = new_value;
    }

    pub fn update_samples_per_pixel(&mut self, new_value: String) {
        if let Ok(_) = new_value.parse::<Uint>() {
            self.samples_per_pixel = new_value;
        }
    }

    pub fn get_samples_per_pixel(&self) -> Uint {
        self.samples_per_pixel.parse::<Uint>().unwrap()
    }

    pub fn update_max_depth(&mut self, new_value: String) {
        if let Ok(_) = new_value.parse::<Uint>() {
            self.max_depth = new_value;
        }
    }

    pub fn get_max_depth(&self) -> Uint {
        self.max_depth.parse::<Uint>().unwrap()
    }

    pub fn update_scene_idx(&mut self, new_value: Uint) {
        self.scene_idx = new_value;
    }

    pub fn update_use_gpu(&mut self, new_value: bool) {
        self.use_gpu = new_value;
    }

    pub fn update_use_importance_sampling(&mut self, new_value: bool) {
        self.use_importance_sampling = new_value;
    }
}

pub fn view(state: &UiState) -> Element<'_, Message> {
    let selected_scene = SCENES.get(state.scene_idx as usize).map(|s| s.to_string());

    #[rustfmt::skip]
    let controls = column![
        text(format!("Render size: {}x{}", state.render_width, state.render_height)),

        text("Samples per pixel:"),
        text_input("", &state.samples_per_pixel).on_input(Message::SamplesChanged),

        text("Max depth:"),
        text_input("", &state.max_depth).on_input(Message::MaxDepthChanged),

        text("Scene:"),
        pick_list(
            SCENES.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
            selected_scene,
            |scene_name| {
                let scene_idx = SCENES.iter().position(|&s| s == scene_name).unwrap();
                Message::SceneSelected(scene_idx as Uint)
            },
        ),

        text("Use GPU?"),
        checkbox(state.use_gpu).on_toggle(Message::UseGpuChanged),

        text("Importance sampling:"),
        checkbox(state.use_importance_sampling).on_toggle(Message::UseImportanceSamplingChanged),
        
        button(if state.is_rendering { "Cancel" } else { "Render" }).on_press(Message::RenderButtonClicked),
    ]
    .spacing(10)
    .padding(10)
    .width(250);

    let render_view = container(
        shader(PreviewWidget::new(
            Arc::clone(&state.frame_buffer),
            Arc::clone(&state.render_idx),
        ))
        .width(Fill)
        .height(Fill),
    )
    .center_x(Fill)
    .center_y(Fill)
    .width(Fill)
    .height(Fill);

    row![controls, render_view].into()
}
