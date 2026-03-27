use std::sync::Arc;

use winit::window::Window;

use crate::gpu::gpu::Gpu;

pub struct EguiIntegration {
    pub state: egui_winit::State,
    renderer: egui_wgpu::Renderer,
}

impl EguiIntegration {
    pub fn new(gpu: &Gpu, window: &Window) -> Self {
        let egui_context = egui::Context::default();
        let state = egui_winit::State::new(
            egui_context,
            egui::ViewportId::ROOT,
            window,
            Some(window.scale_factor() as f32),
            None,
            None,
        );
        let renderer = egui_wgpu::Renderer::new(
            gpu.device(),
            gpu.texture_format(),
            egui_wgpu::RendererOptions::default(),
        );

        Self { state, renderer }
    }

    pub fn register_texture(
        &mut self,
        device: &wgpu::Device,
        view: &wgpu::TextureView,
        filter: wgpu::FilterMode,
    ) -> egui::TextureId {
        self.renderer.register_native_texture(device, view, filter)
    }

    pub fn update_texture(
        &mut self,
        device: &wgpu::Device,
        view: &wgpu::TextureView,
        filter: wgpu::FilterMode,
        texture_id: egui::TextureId,
    ) {
        self.renderer
            .update_egui_texture_from_wgpu_texture(device, view, filter, texture_id);
    }

    pub fn render_ui(
        &mut self,
        gpu: &Gpu,
        window: &Arc<Window>,
        frame: wgpu::SurfaceTexture,
        build_ui: impl FnMut(&mut egui::Ui),
    ) {
        let raw_input = self.state.take_egui_input(window);
        let egui_ctx = self.state.egui_ctx().clone();

        let full_output = egui_ctx.run_ui(raw_input, build_ui);
        self.state.handle_platform_output(window, full_output.platform_output);

        let pixels_per_point = egui_ctx.pixels_per_point();
        let paint_jobs = egui_ctx.tessellate(full_output.shapes, pixels_per_point);

        let screen_descriptor = egui_wgpu::ScreenDescriptor {
            size_in_pixels: [window.inner_size().width, window.inner_size().height],
            pixels_per_point,
        };

        for (id, image_delta) in &full_output.textures_delta.set {
            self.renderer
                .update_texture(gpu.device(), gpu.queue(), *id, image_delta);
        }

        let view = frame.texture.create_view(&Default::default());
        let mut encoder = gpu.create_command_encoder();

        let extra_buffers =
            self.renderer
                .update_buffers(gpu.device(), gpu.queue(), &mut encoder, &paint_jobs, &screen_descriptor);

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
            self.renderer
                .render(&mut rpass.forget_lifetime(), &paint_jobs, &screen_descriptor);
        }

        gpu.submit_and_present(encoder, extra_buffers, frame);

        for id in &full_output.textures_delta.free {
            self.renderer.free_texture(id);
        }
    }
}
