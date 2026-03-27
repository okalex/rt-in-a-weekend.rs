use crate::rt::frame_buffer::FrameBuffer;

pub fn build_ui(ui: &mut egui::Ui, render_texture_id: egui::TextureId, frame_buffer: &FrameBuffer) {
    egui::Panel::left("controls_panel")
        .resizable(true)
        .default_size(250.0)
        .show_inside(ui, |ui| {
            ui.heading("Controls");
            ui.separator();

            ui.label(format!("Render size: {}x{}", frame_buffer.width, frame_buffer.height));

            ui.separator();

            egui::CollapsingHeader::new("Renderer").default_open(true).show(ui, |ui| {
                ui.label("Renderer options will go here.");
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
        let img_size = egui::vec2(frame_buffer.width as f32, frame_buffer.height as f32);
        let scale = (available.x / img_size.x).min(available.y / img_size.y).min(1.0);
        let display_size = img_size * scale;

        ui.centered_and_justified(|ui| {
            ui.add(egui::Image::new(egui::load::SizedTexture::new(render_texture_id, display_size)));
        });
    });

    // Request continuous repaint so the progressive render updates are visible
    ui.ctx().request_repaint();
}
