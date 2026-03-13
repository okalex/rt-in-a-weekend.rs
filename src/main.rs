mod rt;

use std::sync::Arc;

use clap::Parser;
use winit::event_loop::EventLoop;

use crate::rt::app::app::App;
use crate::rt::camera::Camera;
use crate::rt::color::Color;
use crate::rt::frame_buffer::FrameBuffer;
use crate::rt::materials::material::{EmptyMaterial, Material};
use crate::rt::objects::{bvh_node::BvhNode, hittable::Hittable, scene::Scene};
use crate::rt::ppm_writer::PpmWriter;
use crate::rt::renderer::{LineServer, RenderOptionsBuilder, Renderer};
use crate::rt::sampler::Sampler;
use crate::rt::test_scenes::{Shapes, get_camera_and_scene};
use crate::rt::viewport::Viewport;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    interactive: bool,

    #[arg(short, long, default_value_t = 2)]
    scene: u32,

    #[arg(short, long, default_value_t = 400)]
    width: u32,

    #[arg(short, long, default_value_t = 16.0/9.0)]
    aspect: f32,

    #[arg(long, default_value_t = 100)]
    samples: u32,

    #[arg(short, long, default_value_t = 10)]
    depth: u32,

    #[arg(short, long, default_value_t = true)]
    multithreading: bool,

    #[arg(long, default_value_t = 1)]
    sampler: u32,
}

fn main() {
    env_logger::init();

    let args = Args::parse();

    let (camera_options, raw_scene) = get_camera_and_scene(args.scene);

    let render_options = Arc::new(
        RenderOptionsBuilder::new()
            .width(args.width)
            .samples_per_pixel(args.samples)
            .max_depth(args.depth)
            .use_multithreading(args.multithreading)
            .background(Color::black())
            .build(args.aspect as f64),
    );

    let viewport = Viewport::new(
        render_options.img_width,
        render_options.img_height,
        &camera_options,
    );

    let sampler = match args.sampler {
        2 => Sampler::stratified(render_options.samples_per_pixel),
        _ => Sampler::random(render_options.samples_per_pixel),
    };

    let camera = Arc::new(Camera::new(camera_options, viewport, sampler));
    let scene = wrap_scene(raw_scene);

    let frame_buffer = Arc::new(FrameBuffer::new(
        render_options.img_width as usize,
        render_options.img_height as usize,
    ));

    let line_server = Arc::new(LineServer::new(render_options.img_height));

    let renderer = Arc::new(Renderer::new(
        Arc::clone(&render_options),
        Arc::clone(&camera),
        Arc::clone(&frame_buffer),
        Arc::clone(&line_server),
    ));

    // Set up lights - TODO: this is only temporary for testing purposes
    let empty_material: Arc<dyn Material> = Arc::new(EmptyMaterial::new());
    let mut lights = Scene::new();
    lights.add(Shapes::quad(
        [343.0, 554.0, 332.0],
        [-130.0, 0.0, 0.0],
        [0.0, 0.0, -105.0],
        Arc::clone(&empty_material),
    ));
    lights.add(Shapes::sphere([190.0, 90.0, 190.0], 90.0, Arc::clone(&empty_material)));

    if args.interactive {
        let _ = run_windowed(
            render_options.img_width,
            render_options.img_height,
            Arc::clone(&renderer),
            scene,
            Arc::new(lights),
        );
    } else {
        let writer = PpmWriter::new(Arc::clone(&frame_buffer), 255);

        let thread_handles = renderer.render(Arc::clone(&scene), Arc::new(lights));
        thread_handles.into_iter().for_each(|h| h.join().unwrap());

        writer.write();
    }
}

fn run_windowed(
    width: u32,
    height: u32,
    renderer: Arc<Renderer>,
    scene: Arc<dyn Hittable>,
    lights: Arc<dyn Hittable>,
) -> anyhow::Result<()> {
    let event_loop = EventLoop::with_user_event().build()?;
    let mut app = App::new(width, height, Arc::clone(&renderer), scene, lights);
    event_loop.run_app(&mut app)?;

    Ok(())
}

fn wrap_scene(scene: Scene) -> Arc<dyn Hittable> {
    let bvh: Arc<dyn Hittable> = Arc::new(BvhNode::from_scene(scene));
    Arc::new(Scene::new_obj(Arc::clone(&bvh)))
}
