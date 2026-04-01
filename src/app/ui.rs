use iced::widget::{button, column, container, image, pick_list, row, text, text_input};
use iced::{Element, Fill};

use crate::app::app::{App, Message};

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

pub fn view(app: &App) -> Element<'_, Message> {
    let selected_scene = SCENES.get(app.selected_scene_idx).map(|s| s.to_string());

    let controls = column![
        text(format!(
            "Render size: {}x{}",
            app.render_options.img_width, app.render_options.img_height
        )),
        text("Samples per pixel:"),
        text_input("100", &app.samples_per_pixel).on_input(Message::SamplesChanged),
        text("Scene:"),
        pick_list(
            SCENES.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
            selected_scene,
            Message::SceneSelected,
        ),
        button(if app.is_rendering() { "Cancel" } else { "Render" }).on_press(Message::RenderButtonClicked),
    ]
    .spacing(10)
    .padding(10)
    .width(250);

    let render_view = container(image(&app.render_image).content_fit(iced::ContentFit::ScaleDown))
        .center_x(Fill)
        .center_y(Fill)
        .width(Fill)
        .height(Fill);

    row![controls, render_view].into()
}
