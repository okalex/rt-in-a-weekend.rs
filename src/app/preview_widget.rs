use std::{
    cell::Cell,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
};

use iced::{
    Rectangle,
    mouse::Cursor,
    wgpu,
    widget::shader::{Pipeline, Primitive, Program, Viewport},
};

use crate::{app::app::Message, rt::frame_buffer::FrameBuffer};

pub struct PreviewPipeline {
    render_pipeline: wgpu::RenderPipeline,
    bind_group_layout: wgpu::BindGroupLayout,
    bind_group: Option<wgpu::BindGroup>,
    texture: Option<wgpu::Texture>,
    sampler: wgpu::Sampler,
    texture_width: u32,
    texture_height: u32,
}

impl Pipeline for PreviewPipeline {
    fn new(device: &wgpu::Device, _queue: &wgpu::Queue, format: wgpu::TextureFormat) -> Self
    where
        Self: Sized,
    {
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        let shader = device.create_shader_module(wgpu::include_wgsl!("../shaders/display_shader.wgsl"));

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_layout],
            ..Default::default()
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        Self {
            render_pipeline,
            bind_group_layout,
            bind_group: None,
            texture: None,
            sampler,
            texture_width: 0,
            texture_height: 0,
        }
    }
}

impl PreviewPipeline {
    fn setup(&mut self, device: &wgpu::Device, width: u32, height: u32) {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        let texture_view = texture.create_view(&Default::default());

        self.bind_group = Some(device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &self.bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&self.sampler),
                },
            ],
        }));

        self.texture = Some(texture);
        self.texture_width = width;
        self.texture_height = height;
    }
}

#[derive(Debug)]
pub struct PreviewImage {
    width: u32,
    height: u32,
    data: Option<Arc<Vec<u8>>>,
}

impl Primitive for PreviewImage {
    type Pipeline = PreviewPipeline;

    fn prepare(
        &self,
        pipeline: &mut Self::Pipeline,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        _bounds: &Rectangle,
        _viewport: &Viewport,
    ) {
        if (self.width != pipeline.texture_width && self.width > 0)
            || (self.height != pipeline.texture_height && self.height > 0)
        {
            pipeline.setup(device, self.width, self.height);
        }

        if let (Some(data), Some(texture)) = (self.data.as_ref(), pipeline.texture.as_ref()) {
            queue.write_texture(
                texture.as_image_copy(),
                data,
                wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(self.width * 4),
                    rows_per_image: None,
                },
                wgpu::Extent3d {
                    width: self.width,
                    height: self.height,
                    depth_or_array_layers: 1,
                },
            );
        }
    }

    fn draw(&self, pipeline: &Self::Pipeline, render_pass: &mut wgpu::RenderPass<'_>) -> bool {
        let Some(bind_group) = &pipeline.bind_group else {
            return true;
        };

        render_pass.set_pipeline(&pipeline.render_pipeline);
        render_pass.set_bind_group(0, bind_group, &[]);
        render_pass.draw(0..3, 0..1);
        true
    }
}

#[derive(Default)]
pub struct PreviewWidgetState {
    last_render_idx: Cell<u64>,
}
pub struct PreviewWidget {
    frame: Arc<FrameBuffer>,
    render_idx: Arc<AtomicU64>,
}

impl PreviewWidget {
    pub fn new(frame: Arc<FrameBuffer>, render_idx: Arc<AtomicU64>) -> Self {
        Self { frame, render_idx }
    }
}

impl Program<Message> for PreviewWidget {
    type State = PreviewWidgetState;
    type Primitive = PreviewImage;

    fn draw(&self, state: &Self::State, _cursor: Cursor, _bounds: Rectangle) -> Self::Primitive {
        let curr_render_idx = self.render_idx.load(Ordering::Relaxed);
        if curr_render_idx == state.last_render_idx.get() {
            return PreviewImage {
                data: None,
                width: self.frame.width as u32,
                height: self.frame.height as u32,
            };
        }
        state.last_render_idx.set(curr_render_idx);

        let data = self.frame.data.lock().unwrap().clone();
        PreviewImage {
            data: Some(Arc::new(data)),
            width: self.frame.width as u32,
            height: self.frame.height as u32,
        }
    }
}
