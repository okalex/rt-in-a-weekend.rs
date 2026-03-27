use std::sync::Arc;

use crate::rt::frame_buffer::FrameBuffer;

pub struct GpuCanvas {
    texture: wgpu::Texture,
    view: wgpu::TextureView,
}

impl GpuCanvas {
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

        let view = texture.create_view(&wgpu::TextureViewDescriptor {
            format: Some(wgpu::TextureFormat::Rgba8Unorm),
            ..Default::default()
        });

        Self { texture, view }
    }

    pub fn as_image_copy(&self) -> wgpu::TexelCopyTextureInfo<'_> {
        self.texture.as_image_copy()
    }

    pub fn view(&self) -> &wgpu::TextureView {
        &self.view
    }
}
