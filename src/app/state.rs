use std::sync::Arc;

use winit::{event_loop::ActiveEventLoop, keyboard::KeyCode, window::Window};

use crate::{
    app::ui,
    gpu::{gpu::Gpu, gpu_texture::GpuTexture},
    rt::frame_buffer::FrameBuffer,
    util::types::Uint,
};

pub struct State {
    pub window: Arc<Window>,
    pub egui_state: egui_winit::State,
    egui_renderer: egui_wgpu::Renderer,
    render_texture_id: egui::TextureId,
    frame_buffer: Arc<FrameBuffer>,
    gpu: Gpu,
    texture: GpuTexture,
}

impl State {
    pub async fn new(window: Arc<Window>, frame_buffer: Arc<FrameBuffer>) -> anyhow::Result<Self> {
        let gpu = Gpu::new_windowed(Arc::clone(&window)).await?;

        let egui_context = egui::Context::default();
        let egui_state = egui_winit::State::new(
            egui_context,
            egui::ViewportId::ROOT,
            &window,
            Some(window.scale_factor() as f32),
            None,
            None,
        );
        let mut egui_renderer = egui_wgpu::Renderer::new(
            gpu.device(),
            *gpu.texture_format(),
            egui_wgpu::RendererOptions::default(),
        );

        let texture = GpuTexture::new(gpu.device(), Arc::clone(&frame_buffer));

        let render_texture_id = egui_renderer.register_native_texture(
            gpu.device(),
            texture.unorm_view(),
            wgpu::FilterMode::Linear,
        );

        Ok(Self {
            window,
            egui_state,
            egui_renderer,
            render_texture_id,
            frame_buffer,
            gpu,
            texture,
        })
    }

    pub fn update(&mut self) {}

    pub fn resize(&mut self, width: Uint, height: Uint) {
        self.gpu.resize(width, height);
    }

    pub fn handle_key(&mut self, _event_loop: &ActiveEventLoop, code: KeyCode, is_pressed: bool) {
        if code == KeyCode::Escape && is_pressed {
            std::process::exit(0);
        }
    }

    pub fn render(&mut self) -> anyhow::Result<()> {
        self.window.request_redraw();
        if !self.gpu.is_ready() {
            return Ok(());
        }

        // Copy ray tracer output to GPU texture
        self.gpu.write_texture(Arc::clone(&self.frame_buffer), &self.texture);

        // Update the egui texture binding to reflect new content
        self.egui_renderer.update_egui_texture_from_wgpu_texture(
            self.gpu.device(),
            self.texture.unorm_view(),
            wgpu::FilterMode::Linear,
            self.render_texture_id,
        );

        // Begin egui frame
        let raw_input = self.egui_state.take_egui_input(&self.window);
        let egui_ctx = self.egui_state.egui_ctx().clone();
        let full_output = egui_ctx.run_ui(raw_input, |ui| {
            ui::build_ui(ui, self.render_texture_id, &self.frame_buffer);
        });

        // Handle platform output (cursor, clipboard, etc.)
        self.egui_state.handle_platform_output(&self.window, full_output.platform_output);

        // Tessellate
        let pixels_per_point = egui_ctx.pixels_per_point();
        let paint_jobs = egui_ctx.tessellate(full_output.shapes, pixels_per_point);

        let screen_descriptor = egui_wgpu::ScreenDescriptor {
            size_in_pixels: [
                self.window.inner_size().width,
                self.window.inner_size().height,
            ],
            pixels_per_point,
        };

        // Acquire surface texture and create encoder
        let frame = self.gpu.get_current_texture();
        let view = frame.texture.create_view(&Default::default());
        let mut encoder = self.gpu.create_command_encoder();

        // Update egui textures and buffers
        for (id, image_delta) in &full_output.textures_delta.set {
            self.egui_renderer.update_texture(self.gpu.device(), self.gpu.queue(), *id, image_delta);
        }
        let extra_buffers = self.egui_renderer.update_buffers(
            self.gpu.device(),
            self.gpu.queue(),
            &mut encoder,
            &paint_jobs,
            &screen_descriptor,
        );

        // Render egui
        {
            let rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("egui render pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
            self.egui_renderer.render(&mut rpass.forget_lifetime(), &paint_jobs, &screen_descriptor);
        }

        // Submit and present
        self.gpu.submit_and_present(encoder, extra_buffers, frame);

        // Free textures
        for id in &full_output.textures_delta.free {
            self.egui_renderer.free_texture(id);
        }

        Ok(())
    }
}
