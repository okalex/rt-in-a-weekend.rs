use std::sync::Arc;

use crate::rt::frame_buffer::FrameBuffer;

pub struct GpuTexture {
    texture: wgpu::Texture,
    view: wgpu::TextureView,
    unorm_view: wgpu::TextureView,
    sampler: wgpu::Sampler,
}

impl GpuTexture {
    pub fn new(device: &wgpu::Device, buffer: Arc<FrameBuffer>) -> Self {
        let texture_format = wgpu::TextureFormat::Rgba8UnormSrgb;
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Texture"),
            size: wgpu::Extent3d {
                width: buffer.width as u32,
                height: buffer.height as u32,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: texture_format,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[wgpu::TextureFormat::Rgba8Unorm],
        });
        let view = texture.create_view(&Default::default());
        let unorm_view = texture.create_view(&wgpu::TextureViewDescriptor {
            format: Some(wgpu::TextureFormat::Rgba8Unorm),
            ..Default::default()
        });
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        Self { texture, view, unorm_view, sampler }
    }

    pub fn bind_group_layout_entries(&self, binding_idx: u32) -> [wgpu::BindGroupLayoutEntry; 2] {
        [
            wgpu::BindGroupLayoutEntry {
                binding: binding_idx,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: binding_idx + 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
        ]
    }

    pub fn bind_group_entries(&self, binding_idx: u32) -> [wgpu::BindGroupEntry<'_>; 2] {
        [
            wgpu::BindGroupEntry {
                binding: binding_idx,
                resource: wgpu::BindingResource::TextureView(&self.view),
            },
            wgpu::BindGroupEntry {
                binding: binding_idx + 1,
                resource: wgpu::BindingResource::Sampler(&self.sampler),
            },
        ]
    }

    pub fn as_image_copy(&self) -> wgpu::TexelCopyTextureInfo<'_> {
        self.texture.as_image_copy()
    }

    pub fn unorm_view(&self) -> &wgpu::TextureView {
        &self.unorm_view
    }
}
