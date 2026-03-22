use std::sync::Arc;

use encase::ShaderType;
use glam::Vec3;

use crate::rt::{camera::Camera, renderer::render_options::RenderOptions, viewport::Viewport};

#[derive(ShaderType, Debug)]
pub struct GpuMeta {
    pub width: u32,
    pub height: u32,
    pub num_samples: u32,
    pub frame_num: u32,
    pub max_depth: u32,
    pub background: Vec3,
    pub camera: GpuCamera,
    pub viewport: GpuViewport,
}

impl GpuMeta {
    pub fn new(render_options: Arc<RenderOptions>, camera: Arc<Camera>, frame_num: u32) -> Self {
        Self {
            width: render_options.img_width,
            height: render_options.img_height,
            num_samples: render_options.dispatch_size,
            frame_num,
            background: render_options.background.base,
            max_depth: render_options.max_depth,
            camera: GpuCamera::from(Arc::clone(&camera)),
            viewport: GpuViewport::from(&Arc::clone(&camera).viewport),
        }
    }
}

#[derive(ShaderType, Debug)]
pub struct GpuCamera {
    position: Vec3,
    lookat: Vec3,
    defocus_angle: f32,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl From<Arc<Camera>> for GpuCamera {
    fn from(camera: Arc<Camera>) -> Self {
        Self {
            position: camera.options.position,
            lookat: camera.options.target,
            defocus_angle: camera.options.defocus_angle,
            defocus_disk_u: camera.defocus_disk.u,
            defocus_disk_v: camera.defocus_disk.v,
        }
    }
}

#[derive(ShaderType, Debug)]
pub struct GpuViewport {
    delta_u: Vec3,
    delta_v: Vec3,
    pixel00_loc: Vec3,
}

impl From<&Viewport> for GpuViewport {
    fn from(viewport: &Viewport) -> Self {
        Self {
            delta_u: viewport.delta_u,
            delta_v: viewport.delta_v,
            pixel00_loc: viewport.pixel00_loc,
        }
    }
}
