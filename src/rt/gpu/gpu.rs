use std::sync::Arc;

use bytemuck::NoUninit;
use wgpu::util::DeviceExt;
use winit::{
    dpi::PhysicalSize,
    window::Window,
};

use crate::rt::{
    frame_buffer::FrameBuffer,
    gpu::gpu_texture::GpuTexture,
    types::Uint,
};

pub enum Gpu {
    Windowed(GpuWindowed),
    Headless(GpuHeadless),
}

impl Gpu {
    pub async fn new_windowed(window: Arc<Window>) -> anyhow::Result<Self> {
        let gpu = GpuWindowed::new(window).await?;
        Ok(Self::Windowed(gpu))
    }

    pub async fn new_headless() -> anyhow::Result<Self> {
        let gpu = GpuHeadless::new().await?;
        Ok(Self::Headless(gpu))
    }

    pub fn device(&self) -> &wgpu::Device {
        match self {
            Self::Windowed(gpu) => &gpu.device,
            Self::Headless(gpu) => &gpu.device,
        }
    }

    pub fn queue(&self) -> &wgpu::Queue {
        match self {
            Self::Windowed(gpu) => &gpu.queue,
            Self::Headless(gpu) => &gpu.queue,
        }
    }

    pub fn is_ready(&self) -> bool {
        match self {
            Self::Windowed(gpu) => gpu.is_surface_configured,
            Self::Headless(_) => true,
        }
    }

    pub fn create_shader(&self, desc: wgpu::ShaderModuleDescriptor) -> wgpu::ShaderModule {
        self.device().create_shader_module(desc)
    }

    #[allow(unused)]
    pub fn create_vertex_buffer<T: NoUninit>(&self, buffer: &[T]) -> wgpu::Buffer {
        self.device().create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(buffer),
            usage: wgpu::BufferUsages::VERTEX,
        })
    }

    pub fn create_buffer(&self, size: u64, usages: wgpu::BufferUsages) -> wgpu::Buffer {
        self.device().create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: size,
            usage: usages,
            mapped_at_creation: false,
        })
    }

    pub fn create_bind_group_layout(&self, entries: &[wgpu::BindGroupLayoutEntry]) -> wgpu::BindGroupLayout {
        self.device().create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Bind group layout"),
            entries: entries,
        })
    }

    pub fn create_bind_group(&self, layout: &wgpu::BindGroupLayout, entries: &[wgpu::BindGroupEntry]) -> wgpu::BindGroup {
        self.device().create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Bind group"),
            layout: layout,
            entries: entries,
        })
    }

    pub fn write_texture(&self, frame_buffer: Arc<FrameBuffer>, texture: &GpuTexture) {
        // Lock only long enough to copy the buffer
        let buffer = { frame_buffer.data.lock().unwrap().clone() };
        self.queue().write_texture(
            texture.as_image_copy(),
            &buffer,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * frame_buffer.width as u32),
                rows_per_image: None,
            },
            wgpu::Extent3d {
                width: frame_buffer.width as u32,
                height: frame_buffer.height as u32,
                depth_or_array_layers: 1,
            },
        )
    }

    pub fn create_render_pipeline(
        &self,
        bind_group_layouts: &[&wgpu::BindGroupLayout],
        shader: &wgpu::ShaderModule,
    ) -> wgpu::RenderPipeline {
        match self {
            Self::Headless(_) => panic!(),
            Self::Windowed(gpu) => gpu.create_render_pipeline(bind_group_layouts, shader),
        }
    }

    pub fn create_compute_pipeline(&self, shader: &wgpu::ShaderModule) -> wgpu::ComputePipeline {
        self.device().create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("Compute Pipeline"),
            layout: None,
            module: shader,
            entry_point: None,
            compilation_options: Default::default(),
            cache: Default::default(),
        })
    }

    pub fn render(&self, render_pipeline: &wgpu::RenderPipeline, bind_group: &wgpu::BindGroup) {
        match self {
            Self::Headless(_) => panic!(),
            Self::Windowed(gpu) => gpu.render(render_pipeline, bind_group),
        }
    }

    pub fn resize(&mut self, width: Uint, height: Uint) {
        match self {
            Self::Headless(_) => panic!(),
            Self::Windowed(gpu) => gpu.resize(width, height),
        }
    }
}

#[allow(unused)]
pub struct GpuHeadless {
    pub instance: wgpu::Instance,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

impl GpuHeadless {
    pub async fn new() -> anyhow::Result<Self> {
        let instance = wgpu::Instance::new(&Default::default());
        let adapter = instance.request_adapter(&Default::default()).await?;
        let (device, queue) = adapter.request_device(&Default::default()).await?;

        Ok(Self {
            instance,
            adapter,
            device,
            queue,
        })
    }
}

#[allow(unused)]
pub struct GpuWindowed {
    pub instance: wgpu::Instance,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface: wgpu::Surface<'static>,
    pub is_surface_configured: bool,
    pub config: wgpu::SurfaceConfiguration,
}

impl GpuWindowed {
    pub async fn new(window: Arc<Window>) -> anyhow::Result<Self> {
        let size = window.inner_size();

        let instance = Self::new_instance();
        let surface = instance.create_surface(window.clone()).unwrap();
        let adapter = Self::get_adapter(&instance, &surface).await?;
        let (device, queue) = Self::get_device_queue(&adapter).await?;
        let config = Self::get_surface_config(&size, &surface, &adapter);

        Ok(Self {
            instance,
            adapter,
            device,
            queue,
            surface,
            is_surface_configured: false,
            config,
        })
    }

    fn new_instance() -> wgpu::Instance {
        wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            flags: wgpu::InstanceFlags::VALIDATION,
            ..Default::default()
        })
    }

    async fn get_adapter(instance: &wgpu::Instance, surface: &wgpu::Surface<'static>) -> anyhow::Result<wgpu::Adapter> {
        Ok(instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(), // Set to HighPerformance
                compatible_surface: Some(surface),
                force_fallback_adapter: false, // Force it to use the GPU
            })
            .await?)
    }

    async fn get_device_queue(adapter: &wgpu::Adapter) -> anyhow::Result<(wgpu::Device, wgpu::Queue)> {
        Ok(adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                experimental_features: wgpu::ExperimentalFeatures::disabled(),
                required_limits: wgpu::Limits::default(),
                memory_hints: Default::default(),
                trace: wgpu::Trace::Off,
            })
            .await?)
    }

    fn get_surface_config(size: &PhysicalSize<u32>, surface: &wgpu::Surface, adapter: &wgpu::Adapter) -> wgpu::SurfaceConfiguration {
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        }
    }

    pub fn create_render_pipeline(
        &self,
        bind_group_layouts: &[&wgpu::BindGroupLayout],
        shader: &wgpu::ShaderModule,
    ) -> wgpu::RenderPipeline {
        let pipeline_layout = self.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Pipeline layout"),
            bind_group_layouts: bind_group_layouts,
            immediate_size: 0,
        });

        self.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: self.config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        })
    }

    pub fn resize(&mut self, width: Uint, height: Uint) {
        if width > 0 && height > 0 {
            self.config.width = width as u32;
            self.config.height = height as u32;
            self.surface.configure(&self.device, &self.config);
            self.is_surface_configured = true;
        }
    }

    pub fn render(&self, render_pipeline: &wgpu::RenderPipeline, bind_group: &wgpu::BindGroup) {
        let frame = self.surface.get_current_texture().unwrap();
        let view = frame.texture.create_view(&Default::default());
        let mut encoder = self.device.create_command_encoder(&Default::default());
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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
            rpass.set_pipeline(render_pipeline);
            rpass.set_bind_group(0, bind_group, &[]);
            rpass.draw(0..3, 0..1); // TODO: this should be dependent upon the gpu config, bind group, etc.
        }
        self.queue.submit([encoder.finish()]);
        frame.present();
    }
}
