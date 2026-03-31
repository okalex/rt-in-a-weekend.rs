pub struct UiState {
    pub render_texture_id: egui::TextureId,
    pub render_width: usize,
    pub render_height: usize,
    pub is_rendering: bool,
}

pub enum UiAction {
    RenderButtonClicked,
}

pub fn build_ui(ui: &mut egui::Ui, ui_state: &UiState) -> Vec<UiAction> {
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
                    ui.label("Renderer options will go here.");

                    let render_button_text = if ui_state.is_rendering { "Cancel" } else { "Render" };
                    if ui.button(render_button_text).clicked() {
                        actions.push(UiAction::RenderButtonClicked);
                    }
                });

            egui::CollapsingHeader::new("Camera").default_open(true).show(ui, |ui| {
                ui.label("Camera options will go here.");
            });

            egui::CollapsingHeader::new("Scene").default_open(true).show(ui, |ui| {
                ui.label("Scene options will go here.");
            });
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
