mod app;
mod examples;
mod gpu;
mod rt;
mod util;

use std::sync::Arc;

use clap::Parser;
use winit::event_loop::EventLoop;

#[allow(unused)]
use crate::{
    app::app::App,
    examples::scenes::get_scene,
    rt::{
        camera::Camera,
        frame_buffer::FrameBuffer,
        renderer::{
            cpu::line_server::LineServer,
            render_options::{RenderOptions, RenderOptionsBuilder},
            renderer::Renderer,
        },
        sampler::Sampler,
        viewport::Viewport,
    },
};
use crate::{
    app::cli::{Args, print_config},
    gpu::gpu::Gpu,
    rt::renderer::{render_options::SamplerType, renderer_command::RendererCommand},
    util::{color::Color, ppm_writer::PpmWriter, types::Float},
};

#[tokio::main]
async fn main() {
    env_logger::init();

    let args = Args::parse();
    print_config(&args);

    if args.interactive {
        let _ = run_windowed(&args);
    } else {
        let _ = run_headless(&args).await;
    }
}

pub fn get_render_options(args: &Args) -> RenderOptions {
    let _color = Color::black(); // Just so the import doesn't warn

    RenderOptionsBuilder::new()
        .width(args.width)
        .samples_per_pixel(args.samples)
        .dispatch_size(args.dispatch_size)
        .max_depth(args.depth)
        .use_gpu(args.gpu)
        .use_multithreading(args.multithreading)
        .use_importance_sampling(args.importance)
        .background(Color::black())
        .sampler_type(if args.sampler == 2 {
            SamplerType::Stratified
        } else {
            SamplerType::Random
        })
        .build(args.aspect as Float)
}

async fn run_headless(args: &Args) -> anyhow::Result<()> {
    let render_options = get_render_options(args);
    let (camera_options, scene) = get_scene(args.scene);
    let camera = Camera::new(&render_options, &camera_options);

    let frame_buffer = Arc::new(FrameBuffer::new(
        render_options.img_width as usize,
        render_options.img_height as usize,
    ));

    let writer = PpmWriter::new(Arc::clone(&frame_buffer), 255);

    let gpu = if args.gpu {
        Some(Arc::new(Gpu::new_headless().await?))
    } else {
        None
    };

    let (_tx, rx) = tokio::sync::watch::channel(RendererCommand::Render);

    let renderer = Arc::new(
        Renderer::new(
            rx,
            Arc::new(render_options),
            Arc::new(scene),
            Arc::new(camera),
            Arc::clone(&frame_buffer),
            gpu,
        )
        .await
        .unwrap(),
    );

    renderer.render().await;
    writer.write();

    Ok(())
}

fn run_windowed(args: &Args) -> anyhow::Result<()> {
    let event_loop = EventLoop::with_user_event().build()?;
    let mut app = App::new(args);
    event_loop.run_app(&mut app)?;

    Ok(())
}
