use crate::{
    rt::{camera::CameraOptions, renderer::render_options::RenderOptions},
    util::types::Uint,
};

#[derive(Copy, Clone)]
pub enum RendererCommand {
    Idle,
    Render {
        render_options: RenderOptions,
        camera_options: CameraOptions,
        scene_idx: Uint,
    },
    CancelRender,
}
