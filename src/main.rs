mod rt;

use std::sync::Arc;

use clap::Parser;
use winit::event_loop::EventLoop;

#[allow(unused)]
use crate::rt::color::Color;
// use crate::rt::test_scenes::get_camera_and_scene;
use crate::rt::test_scenes2;
use crate::rt::{
    app::app::App,
    camera::Camera,
    frame_buffer::FrameBuffer,
    ppm_writer::PpmWriter,
    renderer::{
        cpu::line_server::LineServer,
        render_options::{
            RenderOptions,
            RenderOptionsBuilder,
        },
        renderer::Renderer,
    },
    sampler::Sampler,
    types::{
        Float,
        Uint,
    },
    viewport::Viewport,
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    interactive: bool,

    #[arg(short, long, default_value_t = 2)]
    scene: Uint,

    #[arg(short, long, default_value_t = 400)]
    width: Uint,

    #[arg(short, long, default_value_t = 16.0/9.0)]
    aspect: Float,

    #[arg(long, default_value_t = 100)]
    samples: Uint,

    #[arg(short, long, default_value_t = 10)]
    depth: Uint,

    #[arg(short, long, default_value_t = false)]
    multithreading: bool,

    #[arg(long, default_value_t = false)]
    importance: bool,

    #[arg(long, default_value_t = false)]
    gpu: bool,

    #[arg(long, default_value_t = 20)]
    dispatch_size: u32,

    #[arg(long, default_value_t = 1)]
    sampler: Uint,
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let args = Arc::new(Args::parse());

    let render_options = Arc::new(
        RenderOptionsBuilder::new()
            .width(args.width)
            .samples_per_pixel(args.samples)
            .dispatch_size(args.dispatch_size)
            .max_depth(args.depth)
            .use_multithreading(args.multithreading)
            .use_importance_sampling(args.importance)
            .background(Color::black())
            .build(args.aspect as Float),
    );

    print_config(Arc::clone(&args), Arc::clone(&render_options));

    let frame_buffer = Arc::new(FrameBuffer::new(
        render_options.img_width as usize,
        render_options.img_height as usize,
    ));
    let renderer = get_renderer(Arc::clone(&args), Arc::clone(&render_options), Arc::clone(&frame_buffer)).await;

    if args.interactive {
        let _ = run_windowed(render_options.img_width, render_options.img_height, renderer, frame_buffer);
    } else {
        let _ = run_headless(renderer, frame_buffer).await;
    }
}

fn print_config(args: Arc<Args>, render_options: Arc<RenderOptions>) {
    eprintln!("Render settings:");
    eprintln!("  interactive         = {}", args.interactive);
    eprintln!("  scene index         = {}", args.scene);
    eprintln!("  image width         = {}", args.width);
    eprintln!("  image height        = {}", render_options.img_height);
    eprintln!("  aspect ratio        = {}", args.aspect);
    eprintln!("  max depth           = {}", args.depth);
    eprintln!("  multithreading      = {}", args.multithreading);
    eprintln!("  importance sampling = {}", args.importance);
    eprintln!("  GPU rendering       = {}", args.gpu);
    eprintln!(
        "  sampler             = {}",
        if args.sampler == 2 { "stratified" } else { "random" }
    );
}

async fn get_renderer(args: Arc<Args>, render_options: Arc<RenderOptions>, frame_buffer: Arc<FrameBuffer>) -> Arc<Renderer> {
    let (camera_options, scene) = test_scenes2::scene1(); //get_camera_and_scene(args.scene);

    let viewport = Viewport::new(render_options.img_width, render_options.img_height, &camera_options);

    let sampler = match args.sampler {
        2 => Sampler::stratified(render_options.samples_per_pixel),
        _ => Sampler::random(render_options.samples_per_pixel),
    };

    let camera = Arc::new(Camera::new(camera_options, viewport, sampler));

    let result = if args.gpu {
        Renderer::gpu(
            Arc::clone(&render_options),
            Arc::new(scene),
            Arc::clone(&camera),
            Arc::clone(&frame_buffer),
        )
        .await
    } else {
        let line_server = Arc::new(LineServer::new(render_options.img_height));

        Renderer::cpu(
            Arc::clone(&render_options),
            Arc::new(scene),
            Arc::clone(&camera),
            Arc::clone(&frame_buffer),
            Arc::clone(&line_server),
        )
        .await
    };

    Arc::new(result.unwrap())
}

async fn run_headless(renderer: Arc<Renderer>, frame_buffer: Arc<FrameBuffer>) -> anyhow::Result<()> {
    let writer = PpmWriter::new(frame_buffer, 255);
    renderer.render().await;
    writer.write();

    Ok(())
}

fn run_windowed(width: Uint, height: Uint, renderer: Arc<Renderer>, frame_buffer: Arc<FrameBuffer>) -> anyhow::Result<()> {
    let event_loop = EventLoop::with_user_event().build()?;
    let mut app = App::new(width, height, renderer, frame_buffer);
    event_loop.run_app(&mut app)?;

    Ok(())
}
