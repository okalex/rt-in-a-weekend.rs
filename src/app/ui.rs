use iced::widget::{button, column, container, image, pick_list, row, text, text_input};
use iced::{Element, Fill};

use crate::app::app::Message;
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
    pub is_rendering: bool,
    pub render_width: Uint,
    pub render_height: Uint,
    pub render_image: image::Handle,
    pub samples_per_pixel: String,
    pub scene_idx: Uint,
}

impl UiState {
    pub fn new(render_options: RenderOptions, scene_idx: Uint) -> Self {
        let render_width = render_options.img_width;
        let render_height = render_options.img_height;
        let render_image = image::Handle::from_rgba(
            render_width,
            render_height,
            vec![0u8; (render_width * render_height * 4) as usize],
        );

        Self {
            is_rendering: false,
            render_width,
            render_height,
            render_image,
            samples_per_pixel: render_options.samples_per_pixel.to_string(),
            scene_idx,
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

    pub fn update_scene_idx(&mut self, new_value: Uint) {
        self.scene_idx = new_value;
    }

    pub fn update_render_image(&mut self, data: Vec<u8>) {
        self.render_image = image::Handle::from_rgba(self.render_width, self.render_height, data);
    }
}

pub fn view(state: &UiState) -> Element<'_, Message> {
    let selected_scene = SCENES.get(state.scene_idx as usize).map(|s| s.to_string());

    #[rustfmt::skip]
    let controls = column![
        text(format!("Render size: {}x{}", state.render_width, state.render_height)),

        text("Samples per pixel:"),
        text_input("100", &state.samples_per_pixel).on_input(Message::SamplesChanged),

        text("Scene:"),
        pick_list(
            SCENES.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
            selected_scene,
            |scene_name| {
                let scene_idx = SCENES.iter().position(|&s| s == scene_name).unwrap();
                Message::SceneSelected(scene_idx as Uint)
            },
        ),
        
        button(if state.is_rendering { "Cancel" } else { "Render" }).on_press(Message::RenderButtonClicked),
    ]
    .spacing(10)
    .padding(10)
    .width(250);

    let render_view = container(image(&state.render_image).content_fit(iced::ContentFit::ScaleDown))
        .center_x(Fill)
        .center_y(Fill)
        .width(Fill)
        .height(Fill);

    row![controls, render_view].into()
}
