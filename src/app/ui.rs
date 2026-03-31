use crate::util::types::Uint;

pub struct UiState {
    pub render_texture_id: egui::TextureId,
    pub render_width: Uint,
    pub render_height: Uint,
    pub is_rendering: bool,
    pub samples_per_pixel: String,
    pub selected_scene_idx: usize,
}

pub enum UiAction {
    RenderButtonClicked,
}

pub fn build_ui(ui: &mut egui::Ui, ui_state: &mut UiState) -> Vec<UiAction> {
    let mut actions: Vec<UiAction> = vec![];

    egui::Panel::left("controls_panel")
        .resizable(true)
        .default_size(250.0)
        .show_inside(ui, |ui| {
            ui.heading("Controls");
            ui.separator();

            ui.label(format!(
                "Render size: {}x{}",
                ui_state.render_width, ui_state.render_height
            ));

            ui.separator();

            egui::CollapsingHeader::new("Renderer")
                .default_open(true)
                .show(ui, |ui| {
                    ui.label("Samples per pixel:");
                    ui.add(egui::TextEdit::singleline(&mut ui_state.samples_per_pixel))
                });

            egui::CollapsingHeader::new("Camera").default_open(true).show(ui, |ui| {
                ui.label("Camera options will go here.");
            });

            let selected_scene_idx = &mut ui_state.selected_scene_idx;
            let scenes = [
                "Spheres",
                "Marbles",
                "Cornell Room",
                "Cornell Room w/ Smoke",
                "Triangles",
                "Mesh",
                "RTiaW Book 2 Final",
                "PBR",
            ];
            egui::CollapsingHeader::new("Scene").default_open(true).show(ui, |ui| {
                ui.label("Scene options will go here.");

                egui::ComboBox::from_label("Scene #:").show_index(ui, selected_scene_idx, scenes.len(), |i| scenes[i]);
            });

            ui.separator();

            let render_button_text = if ui_state.is_rendering { "Cancel" } else { "Render" };
            if ui.button(render_button_text).clicked() {
                actions.push(UiAction::RenderButtonClicked);
            }
        });

    egui::CentralPanel::default().show_inside(ui, |ui| {
        let available = ui.available_size();
        let img_size = egui::vec2(ui_state.render_width as f32, ui_state.render_height as f32);
        let scale = (available.x / img_size.x).min(available.y / img_size.y).min(1.0);
        let display_size = img_size * scale;

        ui.centered_and_justified(|ui| {
            ui.add(egui::Image::new(egui::load::SizedTexture::new(
                ui_state.render_texture_id,
                display_size,
            )));
        });
    });

    // Request continuous repaint so the progressive render updates are visible
    ui.ctx().request_repaint();

    actions
}
