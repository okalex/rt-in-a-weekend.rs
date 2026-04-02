mod app;
mod examples;
mod gpu;
mod rt;
mod util;

use std::sync::Arc;

use clap::Parser;

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
    examples::scenes::get_camera_options,
    gpu::gpu::Gpu,
    rt::renderer::{render_options::SamplerType, renderer::RendererState, renderer_command::RendererCommand},
    util::{color::Color, ppm_writer::PpmWriter, types::Float},
};

fn main() {
    env_logger::init();

    let args = Args::parse();
    print_config(&args);

    if args.interactive {
        let _ = run_windowed(&args);
    } else {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let _ = rt.block_on(run_headless(&args));
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
        // .background(Color::black())
        .sampler_type(if args.sampler == 2 {
            SamplerType::Stratified
        } else {
            SamplerType::Random
        })
        .build(args.aspect as Float)
}

async fn run_headless(args: &Args) -> anyhow::Result<()> {
    let render_options = get_render_options(args);
    let camera_options = get_camera_options(args.scene);

    let frame_buffer = Arc::new(FrameBuffer::new(
        render_options.img_width as usize,
        render_options.img_height as usize,
    ));

    let writer = PpmWriter::new(Arc::clone(&frame_buffer), 255);

    let (tx, rx) = tokio::sync::watch::channel(RendererCommand::Idle);

    let renderer_state = Arc::new(RendererState::new());
    let gpu = Arc::new(Gpu::new().await?);
    let mut renderer = Renderer::new(rx, Arc::clone(&frame_buffer), gpu, renderer_state).await;
    let handle = tokio::spawn(async move {
        renderer.run().await;
    });

    // Start rendering
    let _ = tx.send(RendererCommand::Render {
        render_options,
        camera_options,
        scene_idx: args.scene,
    });

    // Drop the tx channel so renderer exits and wait
    drop(tx);
    let _ = handle.await;

    // Write the results
    writer.write();

    Ok(())
}

fn run_windowed(args: &Args) -> iced::Result {
    let render_options = get_render_options(args);
    let args = *args;
    iced::application(move || App::new(args), App::update, App::view)
        .title("Crabapple")
        .subscription(App::subscription)
        .exit_on_close_request(false)
        .window_size(iced::Size::new(
            (render_options.img_width + 250) as f32,
            render_options.img_height as f32,
        ))
        .run()
}
