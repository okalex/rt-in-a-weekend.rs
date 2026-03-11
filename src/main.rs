mod rt;

use std::sync::Arc;

use clap::Parser;
use nalgebra::{Point3, Vector3};
use winit::event_loop::EventLoop;

use crate::rt::app::app::App;
use crate::rt::camera::Camera;
use crate::rt::color::Color;
use crate::rt::file::load_model_with_mat;
use crate::rt::frame_buffer::FrameBuffer;
use crate::rt::materials::{
    dielectric::Dielectric, diffuse_light::DiffuseLight, lambertian::Lambertian,
    material::Material, metal::Metal,
};
use crate::rt::objects::triangle::Triangle;
use crate::rt::objects::{
    bvh_node::BvhNode,
    constant_medium::ConstantMedium,
    hittable::{Hittable, rotate_y, translate},
    quad::Quad,
    scene::{Box3d, Scene},
    sphere::Sphere,
};
use crate::rt::ppm_writer::PpmWriter;
use crate::rt::random::{rand, rand_range, rand_range_vector};
use crate::rt::renderer::{LineServer, RenderOptionsBuilder, Renderer};
use crate::rt::textures::checkered::Checkered;
use crate::rt::textures::image_map::ImageMap;
use crate::rt::textures::noise::Noise;
use crate::rt::textures::texture::Texture;
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
}

fn run_windowed(
    width: u32,
    height: u32,
    renderer: Arc<Renderer>,
    scene: Arc<dyn Hittable>,
) -> anyhow::Result<()> {
    let event_loop = EventLoop::with_user_event().build()?;
    let mut app = App::new(width, height, Arc::clone(&renderer), scene);
    event_loop.run_app(&mut app)?;

    Ok(())
}

fn main() {
    env_logger::init();

    let args = Args::parse();

    let (raw_camera, raw_scene) = get_camera_and_scene(args.scene);
    let camera = Arc::new(raw_camera);
    let scene = wrap_scene(raw_scene);

    let render_options = Arc::new(
        RenderOptionsBuilder::new()
            .width(args.width)
            .samples_per_pixel(args.samples)
            .max_depth(args.depth)
            .use_multithreading(args.multithreading)
            .background(Color::black())
            .build(args.aspect as f64),
    );

    let viewport = Arc::new(Viewport::new(
        render_options.img_width,
        render_options.img_height,
        &camera,
    ));

    let frame_buffer = Arc::new(FrameBuffer::new(
        render_options.img_width as usize,
        render_options.img_height as usize,
    ));

    let line_server = Arc::new(LineServer::new(render_options.img_height));

    let renderer = Arc::new(Renderer::new(
        Arc::clone(&render_options),
        Arc::clone(&camera),
        Arc::clone(&viewport),
        Arc::clone(&frame_buffer),
        Arc::clone(&line_server),
    ));

    if args.interactive {
        let _ = run_windowed(
            render_options.img_width,
            render_options.img_height,
            Arc::clone(&renderer),
            scene,
        );
    } else {
        let writer = PpmWriter::new(Arc::clone(&frame_buffer), 255);

        let thread_handles = renderer.render(Arc::clone(&scene));
        thread_handles.into_iter().for_each(|h| h.join().unwrap());

        writer.write();
    }
}

fn get_camera_and_scene(scene_idx: u32) -> (Camera, Scene) {
    let (camera_options, raw_scene) = match scene_idx {
        1 => (camera_a(), scene_a()),
        2 => (camera_b(), scene_b()),
        3 => (camera_c(), scene_c()),
        4 => (camera_d(), scene_d()),
        5 => (camera_e(), scene_e()),
        6 => (camera_f(), scene_f()),
        7 => (camera_g(), scene_g()),
        8 => (camera_cornell(), scene_cornell()),
        9 => (camera_cornell(), scene_cornell_smoke()),
        10 => (camera_book2_final(), scene_book2_final(true)),
        11 => (camera_triangle(), scene_triangle()),
        12 => (camera_cube(), scene_cube()),
        _ => panic!(),
    };
    (camera_options, raw_scene)
}

fn rand_arr3() -> [f64; 3] {
    [rand(), rand(), rand()]
}

fn wrap_scene(scene: Scene) -> Arc<dyn Hittable> {
    let bvh: Arc<dyn Hittable> = Arc::new(BvhNode::from_scene(scene));
    Arc::new(Scene::new_obj(Arc::clone(&bvh)))
}

fn new_sphere(center: [f64; 3], radius: f64, mat: Arc<dyn Material>) -> Arc<dyn Hittable> {
    Arc::new(Sphere::stationary(
        Point3::from(center),
        radius,
        Arc::clone(&mat),
    ))
}

fn textured_sphere(center: [f64; 3], radius: f64, texture: Arc<dyn Texture>) -> Arc<dyn Hittable> {
    let mat: Arc<dyn Material> = Arc::new(Lambertian::new(texture));
    new_sphere(center, radius, mat)
}

fn quad(q: [f64; 3], u: [f64; 3], v: [f64; 3], mat: Arc<dyn Material>) -> Arc<dyn Hittable> {
    Arc::new(Quad::from_arr(q, u, v, mat))
}

fn triangle(a: [f64; 3], b: [f64; 3], c: [f64; 3], mat: Arc<dyn Material>) -> Arc<dyn Hittable> {
    Arc::new(Triangle::new(a, b, c, mat))
}

fn box3d(a: [f64; 3], b: [f64; 3], mat: Arc<dyn Material>) -> Arc<dyn Hittable> {
    Arc::new(Box3d::new(
        Vector3::from(a),
        Vector3::from(b),
        Arc::clone(&mat),
    ))
}

fn lambertian(color: [f64; 3]) -> Arc<dyn Material> {
    Arc::new(Lambertian::from_color_values(color))
}

fn diffuse_light(color: [f64; 3]) -> Arc<dyn Material> {
    Arc::new(DiffuseLight::from_color(Color::from_arr(color)))
}

fn camera_a() -> Camera {
    Camera::new()
        .vfov(50.0)
        .position([-1.0, 1.0, 1.0])
        .target([0.0, 0.0, -1.0])
        .defocus_angle(0.5)
        .focus_dist(3.4)
}

fn scene_a() -> Scene {
    let material_ground = lambertian([0.8, 0.8, 0.0]);
    let material_center = lambertian([0.1, 0.2, 0.5]);
    let material_left: Arc<dyn Material> = Arc::new(Dielectric::new(1.5));
    let material_bubble: Arc<dyn Material> = Arc::new(Dielectric::new(1.0 / 1.5));
    let material_right: Arc<dyn Material> = Arc::new(Metal::new([0.8, 0.6, 0.2], 0.2));

    let sphere1 = new_sphere([1.0, -100.5, -1.0], 100.0, material_ground);
    let sphere2 = new_sphere([0.0, 0.0, -1.2], 0.5, material_center);
    let sphere3 = new_sphere([-1.0, 0.0, -1.0], 0.5, material_left);
    let sphere4 = new_sphere([-1.0, 0.0, -1.0], 0.4, material_bubble);
    let sphere5 = new_sphere([1.0, 0.0, -1.0], 0.5, material_right);

    let mut scene = Scene::new();
    scene.add(Arc::clone(&sphere1));
    scene.add(Arc::clone(&sphere2));
    scene.add(Arc::clone(&sphere3));
    scene.add(Arc::clone(&sphere4));
    scene.add(Arc::clone(&sphere5));
    scene
}

fn camera_triangle() -> Camera {
    Camera::new()
        .vfov(50.0)
        .position([0.0, 1.0, 2.0])
        .target([0.0, 0.5, 0.0])
        .defocus_angle(0.5)
        .focus_dist(3.4)
}

fn scene_triangle() -> Scene {
    let material_ground = lambertian([0.8, 0.8, 0.0]);
    let material_center = lambertian([0.1, 0.2, 0.5]);

    let sphere1 = new_sphere([1.0, -100.5, -1.0], 100.0, material_ground);
    let tri1 = triangle(
        [0.0, 1.0, -1.0],
        [-1.0, 0.0, -1.0],
        [1.0, 0.5, -1.0],
        material_center,
    );

    let mut scene = Scene::new();
    scene.add(Arc::clone(&sphere1));
    scene.add(Arc::clone(&tri1));
    scene
}

fn camera_cube() -> Camera {
    Camera::new()
        .vfov(50.0)
        .position([0.0, 4.0, 6.0])
        .target([0.0, 1.0, 0.0])
        .defocus_angle(0.5)
        .focus_dist(3.4)
}

fn scene_cube() -> Scene {
    let mut scene = Scene::new();

    let checkers: Arc<dyn Texture> = Arc::new(Checkered::from_color_values(
        0.32,
        [0.2, 0.3, 0.1],
        [0.9, 0.9, 0.9],
    ));
    let mat_ground: Arc<dyn Material> = Arc::new(Lambertian::new(checkers.clone()));
    let sphere1 = new_sphere([1.0, -100.5, -1.0], 100.0, mat_ground);
    scene.add(Arc::clone(&sphere1));

    let metal: Arc<dyn Material> = Arc::new(Metal::new([0.6, 0.6, 0.7], 0.3));
    let objs = match load_model_with_mat("teapot.obj", metal) {
        Ok(os) => os,
        _ => panic!(),
    };
    for obj in objs {
        let arc: Arc<dyn Hittable> = translate(rotate_y(Arc::new(obj), -15.0), [0.0, 0.0, 0.0]);
        scene.add(Arc::clone(&arc));
    }

    let difflight = diffuse_light([8.0, 8.0, 8.0]);
    let quad_light = quad(
        [8.0, 1.0, 3.0],
        [0.0, 0.0, -4.0],
        [0.0, 2.0, 0.0],
        Arc::clone(&difflight),
    );
    let sphere_light = new_sphere([0.0, 6.5, -2.0], 0.5, Arc::clone(&difflight));
    scene.add(quad_light);
    scene.add(sphere_light);

    scene
}

fn camera_b() -> Camera {
    Camera::new()
        .position([13.0, 2.0, 3.0])
        .target([0.0, 0.0, 0.0])
        .defocus_angle(0.6)
        .focus_dist(10.0)
}

fn scene_b() -> Scene {
    // Textures
    let checker_texture: Arc<dyn Texture> = Arc::new(Checkered::from_color_values(
        0.32,
        [0.2, 0.3, 0.1],
        [0.9, 0.9, 0.9],
    ));
    let perlin_texture: Arc<dyn Texture> = Arc::new(Noise::new(8.0));

    // Materials
    let mat_ground: Arc<dyn Material> = Arc::new(Lambertian::new(checker_texture.clone()));
    let mat1: Arc<dyn Material> = Arc::new(Dielectric::new(1.5));
    let mat2: Arc<dyn Material> = Arc::new(Lambertian::from_color_values([0.4, 0.2, 0.1]));
    let mat3: Arc<dyn Material> = Arc::new(Metal::new([0.7, 0.6, 0.5], 0.0));

    // Objects
    let sphere_ground = new_sphere([0.0, -1000.0, 0.0], 1000.0, mat_ground.clone());
    let sphere1 = new_sphere([0.0, 1.0, 0.0], 1.0, Arc::clone(&mat1));
    let sphere2 = new_sphere([-4.0, 1.0, 0.0], 1.0, Arc::clone(&mat2));
    let sphere3 = new_sphere([4.0, 1.0, 0.0], 1.0, Arc::clone(&mat3));

    let mut scene = Scene::new();
    scene.add(Arc::clone(&sphere_ground));
    scene.add(Arc::clone(&sphere1));
    scene.add(Arc::clone(&sphere2));
    scene.add(Arc::clone(&sphere3));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand();
            let center = Point3::new(a as f64 + 0.9 * rand(), 0.2, b as f64 + 0.9 * rand());

            if (center - Point3::new(4.0, 0.2, 0.0)).magnitude() > 0.9 {
                if choose_mat < 0.4 {
                    // Diffuse
                    let albedo = rand_arr3();
                    let mat_sphere: Arc<dyn Material> =
                        Arc::new(Lambertian::from_color_values(albedo));
                    let center2 = center + Vector3::new(0.0, rand_range(0.0, 0.5), 0.0);
                    let sphere: Arc<dyn Hittable> = Arc::new(Sphere::moving(
                        center,
                        center2,
                        0.2,
                        Arc::clone(&mat_sphere),
                    ));
                    scene.add(Arc::clone(&sphere));
                } else if choose_mat < 0.6 {
                    // Metal
                    let albedo = rand_arr3();
                    let fuzz = rand_range(0.0, 0.5);
                    let mat_sphere: Arc<dyn Material> = Arc::new(Metal::new(albedo, fuzz));
                    let sphere: Arc<dyn Hittable> =
                        Arc::new(Sphere::stationary(center, 0.2, Arc::clone(&mat_sphere)));
                    scene.add(Arc::clone(&sphere));
                } else if choose_mat < 0.8 {
                    // Glass
                    let mat_sphere: Arc<dyn Material> = Arc::new(Dielectric::new(1.5));
                    let sphere: Arc<dyn Hittable> =
                        Arc::new(Sphere::stationary(center, 0.2, Arc::clone(&mat_sphere)));
                    scene.add(Arc::clone(&sphere));
                } else {
                    // Marble
                    let mat_sphere: Arc<dyn Material> =
                        Arc::new(Lambertian::new(Arc::clone(&perlin_texture)));
                    let sphere: Arc<dyn Hittable> =
                        Arc::new(Sphere::stationary(center, 0.2, Arc::clone(&mat_sphere)));
                    scene.add(Arc::clone(&sphere));
                }
            }
        }
    }

    scene
}

fn camera_c() -> Camera {
    Camera::new()
        .position([13.0, 2.0, 3.0])
        .target([0.0, 0.0, 0.0])
}

fn scene_c() -> Scene {
    let checker_texture: Arc<dyn Texture> = Arc::new(Checkered::from_color_values(
        0.32,
        [0.2, 0.3, 0.1],
        [0.9, 0.9, 0.9],
    ));
    let mat_ground: Arc<dyn Material> = Arc::new(Lambertian::new(checker_texture.clone()));

    let sphere1 = new_sphere([0.0, -10.0, 0.0], 10.0, mat_ground.clone());
    let sphere2 = new_sphere([0.0, 10.0, 0.0], 10.0, mat_ground.clone());

    let mut scene = Scene::new();
    scene.add(Arc::clone(&sphere1));
    scene.add(Arc::clone(&sphere2));
    scene
}

fn camera_d() -> Camera {
    Camera::new()
        .position([0.0, 4.0, 16.0])
        .target([0.0, 1.5, 0.0])
}

fn scene_d() -> Scene {
    let earth_texture: Arc<dyn Texture> = Arc::new(ImageMap::new(
        "/Users/alex/src/okalex/rt-in-a-weekend/img/earthmap.jpg",
    ));
    let earth_surface: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::clone(&earth_texture)));
    let globe = new_sphere([0.0, 2.0, 0.0], 2.0, earth_surface.clone());

    let checker_texture: Arc<dyn Texture> = Arc::new(Checkered::from_color_values(
        0.32,
        [0.2, 0.3, 0.1],
        [0.9, 0.9, 0.9],
    ));
    let mat_ground: Arc<dyn Material> = Arc::new(Lambertian::new(checker_texture.clone()));
    let sphere_ground = new_sphere([0.0, -1000.0, 0.0], 1000.0, mat_ground.clone());

    let mut scene = Scene::new();
    scene.add(Arc::clone(&globe));
    scene.add(Arc::clone(&sphere_ground));
    scene
}

fn camera_e() -> Camera {
    Camera::new()
        .position([13.0, 2.0, 3.0])
        .target([0.0, 0.0, 0.0])
}

fn scene_e() -> Scene {
    let perlin_texture: Arc<dyn Texture> = Arc::new(Noise::new(4.0));
    let perlin_surface: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::clone(&perlin_texture)));
    let sphere1 = new_sphere([0.0, -1000.0, 0.0], 1000.0, perlin_surface.clone());
    let sphere2 = new_sphere([0.0, 2.0, 0.0], 2.0, perlin_surface.clone());

    let mut scene = Scene::new();
    scene.add(Arc::clone(&sphere1));
    scene.add(Arc::clone(&sphere2));
    scene
}

fn camera_f() -> Camera {
    Camera::new()
        .vfov(80.0)
        .position([0.0, 0.0, 9.0])
        .target([0.0, 0.0, 0.0])
}

fn scene_f() -> Scene {
    let left_red: Arc<dyn Material> = Arc::new(Lambertian::from_color_values([1.0, 0.2, 0.2]));
    let back_green: Arc<dyn Material> = Arc::new(Lambertian::from_color_values([0.2, 1.0, 0.2]));
    let right_blue: Arc<dyn Material> = Arc::new(Lambertian::from_color_values([0.2, 0.2, 1.0]));
    let upper_orange: Arc<dyn Material> = Arc::new(Lambertian::from_color_values([1.0, 0.5, 0.0]));
    let lower_teal: Arc<dyn Material> = Arc::new(Lambertian::from_color_values([0.2, 0.8, 0.8]));

    let left = quad(
        [-3.0, -2.0, 5.0],
        [0.0, 0.0, -4.0],
        [0.0, 4.0, 0.0],
        Arc::clone(&left_red),
    );
    let back = quad(
        [-2.0, -2.0, 0.0],
        [4.0, 0.0, 0.0],
        [0.0, 4.0, 0.0],
        Arc::clone(&back_green),
    );
    let right = quad(
        [3.0, -2.0, 1.0],
        [0.0, 0.0, 4.0],
        [0.0, 4.0, 0.0],
        Arc::clone(&right_blue),
    );
    let upper = quad(
        [-2.0, 3.0, 1.0],
        [4.0, 0.0, 0.0],
        [0.0, 0.0, 4.0],
        Arc::clone(&upper_orange),
    );
    let lower = quad(
        [-2.0, -3.0, 5.0],
        [4.0, 0.0, 0.0],
        [0.0, 0.0, -4.0],
        Arc::clone(&lower_teal),
    );

    let mut scene = Scene::new();
    scene.add(Arc::clone(&left));
    scene.add(Arc::clone(&back));
    scene.add(Arc::clone(&right));
    scene.add(Arc::clone(&upper));
    scene.add(Arc::clone(&lower));
    scene
}

fn camera_g() -> Camera {
    Camera::new()
        .position([26.0, 3.0, 6.0])
        .target([0.0, 2.0, 0.0])
    // .background([0.0, 0.0, 0.0])
}

fn scene_g() -> Scene {
    let pertext: Arc<dyn Texture> = Arc::new(Noise::new(4.0));
    let ground = textured_sphere([0.0, -1000.0, 0.0], 1000.0, Arc::clone(&pertext));
    let sphere = textured_sphere([0.0, 2.0, 0.0], 2.0, Arc::clone(&pertext));

    let difflight = diffuse_light([8.0, 8.0, 8.0]);
    let quad_light = quad(
        [3.0, 1.0, -2.0],
        [2.0, 0.0, 0.0],
        [0.0, 2.0, 0.0],
        Arc::clone(&difflight),
    );
    let sphere_light = new_sphere([0.0, 6.5, 0.0], 0.5, Arc::clone(&difflight));

    let mut scene = Scene::new();
    scene.add(Arc::clone(&ground));
    scene.add(Arc::clone(&sphere));
    scene.add(Arc::clone(&quad_light));
    scene.add(Arc::clone(&sphere_light));
    scene
}

fn camera_cornell() -> Camera {
    Camera::new()
        .vfov(40.0)
        .position([278.0, 278.0, -800.0])
        .target([278.0, 278.0, 0.0])
    // .background([0.0, 0.0, 0.0])
}

fn scene_cornell() -> Scene {
    let red = lambertian([0.65, 0.05, 0.05]);
    let white = lambertian([0.73, 0.73, 0.73]);
    let green = lambertian([0.12, 0.45, 0.15]);
    let light = diffuse_light([15.0, 15.0, 15.0]);

    let light = quad(
        [343.0, 554.0, 332.0],
        [-130.0, 0.0, 0.0],
        [0.0, 0.0, -105.0],
        Arc::clone(&light),
    );
    let left = quad(
        [555.0, 0.0, 0.0],
        [0.0, 555.0, 0.0],
        [0.0, 0.0, 555.0],
        Arc::clone(&green),
    );
    let right = quad(
        [0.0, 0.0, 0.0],
        [0.0, 555.0, 0.0],
        [0.0, 0.0, 555.0],
        Arc::clone(&red),
    );
    let floor = quad(
        [0.0, 0.0, 0.0],
        [555.0, 0.0, 0.0],
        [0.0, 0.0, 555.0],
        Arc::clone(&white),
    );
    let ceiling = quad(
        [555.0, 555.0, 555.0],
        [-555.0, 0.0, 0.0],
        [0.0, 0.0, -555.0],
        Arc::clone(&white),
    );
    let back = quad(
        [0.0, 0.0, 555.0],
        [555.0, 0.0, 0.0],
        [0.0, 555.0, 0.0],
        Arc::clone(&white),
    );

    let mut box_right = box3d([0.0, 0.0, 0.0], [165.0, 165.0, 165.0], Arc::clone(&white));
    box_right = rotate_y(box_right, -18.0);
    box_right = translate(box_right, [130.0, 0.0, 65.0]);

    let mut box_left = box3d([0.0, 0.0, 0.0], [165.0, 330.0, 165.0], Arc::clone(&white));
    box_left = rotate_y(box_left, 15.0);
    box_left = translate(box_left, [265.0, 0.0, 295.0]);

    let mut scene = Scene::new();
    scene.add(Arc::clone(&left));
    scene.add(Arc::clone(&right));
    scene.add(Arc::clone(&floor));
    scene.add(Arc::clone(&ceiling));
    scene.add(Arc::clone(&back));
    scene.add(Arc::clone(&light));
    scene.add(Arc::clone(&box_right));
    scene.add(Arc::clone(&box_left));
    scene
}

fn scene_cornell_smoke() -> Scene {
    let red = lambertian([0.65, 0.05, 0.05]);
    let white = lambertian([0.73, 0.73, 0.73]);
    let green = lambertian([0.12, 0.45, 0.15]);
    let light = diffuse_light([7.0, 7.0, 7.0]);

    let light = quad(
        [113.0, 554.0, 127.0],
        [330.0, 0.0, 0.0],
        [0.0, 0.0, 305.0],
        Arc::clone(&light),
    );
    let left = quad(
        [555.0, 0.0, 0.0],
        [0.0, 555.0, 0.0],
        [0.0, 0.0, 555.0],
        Arc::clone(&green),
    );
    let right = quad(
        [0.0, 0.0, 0.0],
        [0.0, 555.0, 0.0],
        [0.0, 0.0, 555.0],
        Arc::clone(&red),
    );
    let floor = quad(
        [0.0, 0.0, 0.0],
        [555.0, 0.0, 0.0],
        [0.0, 0.0, 555.0],
        Arc::clone(&white),
    );
    let ceiling = quad(
        [555.0, 555.0, 555.0],
        [-555.0, 0.0, 0.0],
        [0.0, 0.0, -555.0],
        Arc::clone(&white),
    );
    let back = quad(
        [0.0, 0.0, 555.0],
        [555.0, 0.0, 0.0],
        [0.0, 555.0, 0.0],
        Arc::clone(&white),
    );

    let mut box_right_boundary = box3d([0.0, 0.0, 0.0], [165.0, 165.0, 165.0], Arc::clone(&white));
    box_right_boundary = rotate_y(box_right_boundary, -18.0);
    box_right_boundary = translate(box_right_boundary, [130.0, 0.0, 65.0]);
    let box_right: Arc<dyn Hittable> = Arc::new(ConstantMedium::from_color(
        box_right_boundary,
        0.01,
        Color::new(1.0, 1.0, 1.0),
    ));

    let mut box_left_boundary = box3d([0.0, 0.0, 0.0], [165.0, 330.0, 165.0], Arc::clone(&white));
    box_left_boundary = rotate_y(box_left_boundary, 15.0);
    box_left_boundary = translate(box_left_boundary, [265.0, 0.0, 295.0]);
    let box_left: Arc<dyn Hittable> = Arc::new(ConstantMedium::from_color(
        box_left_boundary,
        0.01,
        Color::black(),
    ));

    let mut scene = Scene::new();
    scene.add(Arc::clone(&left));
    scene.add(Arc::clone(&right));
    scene.add(Arc::clone(&floor));
    scene.add(Arc::clone(&ceiling));
    scene.add(Arc::clone(&back));
    scene.add(Arc::clone(&light));
    scene.add(Arc::clone(&box_right));
    scene.add(Arc::clone(&box_left));
    scene
}

fn camera_book2_final() -> Camera {
    Camera::new()
        .vfov(40.0)
        .position([478.0, 278.0, -600.0])
        .target([278.0, 278.0, 0.0])
    // .background([0.01, 0.01, 0.01])
}

fn scene_book2_final(with_haze: bool) -> Scene {
    let mut scene = Scene::new();

    // Floor boxes
    let ground = lambertian([0.48, 0.83, 0.53]);
    let mut boxes = Scene::new();
    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let fi = i as f64;
            let fj = j as f64;

            let w = 100.0;

            let x0 = -1000.0 + fi * w;
            let y0 = 0.0;
            let z0 = -1000.0 + fj * w;

            let x1 = x0 + w;
            let y1 = rand_range(1.0, 101.0);
            let z1 = z0 + w;

            boxes.add(box3d([x0, y0, z0], [x1, y1, z1], Arc::clone(&ground)));
        }
    }
    let boxes_bvh: Arc<dyn Hittable> = Arc::new(BvhNode::from_scene(boxes));
    scene.add(Arc::clone(&boxes_bvh));

    // Light
    let diffuse = diffuse_light([7.0, 7.0, 7.0]);
    let light = quad(
        [123.0, 554.0, 147.0],
        [300.0, 0.0, 0.0],
        [0.0, 0.0, 265.0],
        Arc::clone(&diffuse),
    );
    scene.add(Arc::clone(&light));

    // Moving sphere
    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vector3::new(30.0, 0.0, 0.0);
    let sphere_mat = lambertian([0.7, 0.3, 0.1]);
    let sphere: Arc<dyn Hittable> = Arc::new(Sphere::moving(
        center1,
        center2,
        50.0,
        Arc::clone(&sphere_mat),
    ));
    scene.add(Arc::clone(&sphere));

    // Glass sphere
    let glass = dielectric(1.5);
    let sphere = new_sphere([260.0, 150.0, 45.0], 50.0, Arc::clone(&glass));
    scene.add(Arc::clone(&sphere));

    // Metal sphere
    let metal = make_metal([0.8, 0.8, 0.9], 1.0);
    let sphere = new_sphere([0.0, 150.0, 145.0], 50.0, Arc::clone(&metal));
    scene.add(Arc::clone(&sphere));

    // Blue glass
    let boundary = new_sphere([360.0, 150.0, 145.0], 70.0, Arc::clone(&glass));
    scene.add(Arc::clone(&boundary));
    let medium: Arc<dyn Hittable> = Arc::new(ConstantMedium::from_color(
        boundary,
        0.2,
        Color::new(0.2, 0.4, 0.9),
    ));
    scene.add(Arc::clone(&medium));

    // Haze
    if with_haze {
        let boundary = new_sphere([0.0, 0.0, 0.0], 5000.0, Arc::clone(&glass));
        let medium: Arc<dyn Hittable> =
            Arc::new(ConstantMedium::from_color(boundary, 0.0001, Color::white()));
        scene.add(Arc::clone(&medium));
    }

    // Globe
    let earth_texture: Arc<dyn Texture> = Arc::new(ImageMap::new(
        "/Users/alex/src/okalex/rt-in-a-weekend/img/earthmap.jpg",
    ));
    let earth_surface: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::clone(&earth_texture)));
    let globe: Arc<dyn Hittable> = new_sphere(
        [400.0, 200.0, 400.0],
        100.0,
        Arc::clone(&earth_surface.clone()),
    );
    scene.add(Arc::clone(&globe));

    // Noisy ball
    let pertext: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::new(Noise::new(0.2))));
    let sphere = new_sphere([220.0, 280.0, 300.0], 80.0, Arc::clone(&pertext));
    scene.add(Arc::clone(&sphere));

    // Bubbly box
    let white = lambertian([0.73, 0.73, 0.73]);
    let mut boxes2 = Scene::new();
    for _ in 0..1000 {
        let sphere: Arc<dyn Hittable> = Arc::new(Sphere::stationary(
            Point3::from(rand_range_vector(0.0, 165.0)),
            10.0,
            Arc::clone(&white),
        ));
        boxes2.add(Arc::clone(&sphere));
    }
    let mut boxes2_hittable: Arc<dyn Hittable> = Arc::new(BvhNode::from_scene(boxes2));
    boxes2_hittable = rotate_y(boxes2_hittable, 15.0);
    boxes2_hittable = translate(boxes2_hittable, [-100.0, 270.0, 395.0]);
    scene.add(Arc::clone(&boxes2_hittable));

    scene
}

fn dielectric(refraction_idx: f64) -> Arc<dyn Material> {
    Arc::new(Dielectric::new(refraction_idx))
}

fn make_metal(color: [f64; 3], fuzz: f64) -> Arc<dyn Material> {
    Arc::new(Metal::new(color, fuzz))
}
