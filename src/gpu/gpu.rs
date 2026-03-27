use std::sync::{Arc, Mutex};

use bytemuck::NoUninit;
use wgpu::util::DeviceExt;
use winit::{dpi::PhysicalSize, window::Window};

use crate::{gpu::gpu_canvas::GpuCanvas, rt::frame_buffer::FrameBuffer, util::types::Uint};

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

    pub fn texture_format(&self) -> wgpu::TextureFormat {
        match self {
            Self::Windowed(gpu) => gpu.surface_state.lock().unwrap().config.format,
            Self::Headless(_) => panic!(),
        }
    }

    pub fn is_ready(&self) -> bool {
        match self {
            Self::Windowed(gpu) => gpu.surface_state.lock().unwrap().is_configured,
            Self::Headless(_) => true,
        }
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

    pub fn create_bind_group(&self, layout: &wgpu::BindGroupLayout, entries: &[wgpu::BindGroupEntry]) -> wgpu::BindGroup {
        self.device().create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Bind group"),
            layout: layout,
            entries: entries,
        })
    }

    pub fn write_texture(&self, frame_buffer: Arc<FrameBuffer>, texture: &GpuCanvas) {
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

    pub fn get_current_texture(&self) -> Option<wgpu::SurfaceTexture> {
        match self {
            Self::Headless(_) => panic!(),
            Self::Windowed(gpu) => match gpu.surface.get_current_texture() {
                wgpu::CurrentSurfaceTexture::Success(tex) | wgpu::CurrentSurfaceTexture::Suboptimal(tex) => Some(tex),
                _ => None,
            },
        }
    }

    pub fn create_command_encoder(&self) -> wgpu::CommandEncoder {
        self.device().create_command_encoder(&Default::default())
    }

    pub fn submit_and_present(&self, encoder: wgpu::CommandEncoder, extra_buffers: Vec<wgpu::CommandBuffer>, frame: wgpu::SurfaceTexture) {
        let mut buffers = extra_buffers;
        buffers.push(encoder.finish());
        self.queue().submit(buffers);
        frame.present();
    }

    pub fn resize(&self, width: Uint, height: Uint) {
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
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle());
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
    pub surface_state: Mutex<SurfaceState>,
}

pub struct SurfaceState {
    pub is_configured: bool,
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

        let is_configured = if size.width > 0 && size.height > 0 {
            surface.configure(&device, &config);
            true
        } else {
            false
        };

        Ok(Self {
            instance,
            adapter,
            device,
            queue,
            surface,
            surface_state: Mutex::new(SurfaceState { is_configured, config }),
        })
    }

    fn new_instance() -> wgpu::Instance {
        wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            flags: wgpu::InstanceFlags::VALIDATION,
            backend_options: Default::default(),
            memory_budget_thresholds: Default::default(),
            display: None,
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

    pub fn resize(&self, width: Uint, height: Uint) {
        if width > 0 && height > 0 {
            let mut state = self.surface_state.lock().unwrap();
            state.config.width = width as u32;
            state.config.height = height as u32;
            self.surface.configure(&self.device, &state.config);
            state.is_configured = true;
        }
    }
}
