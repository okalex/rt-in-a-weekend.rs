mod lib;

use std::sync::Arc;

use crate::lib::bvh_node::BvhNode;
use crate::lib::camera::CameraBuilder;
use crate::lib::color::Color;
use crate::lib::constant_medium::ConstantMedium;
use crate::lib::hittable::{Hittable, RotateY, Translate, rotate_y, translate};
use crate::lib::material::{Dielectric, DiffuseLight, Lambertian, Material, Metal};
use crate::lib::quad::Quad;
use crate::lib::random::{rand, rand_range};
use crate::lib::scene::{Box3d, Scene};
use crate::lib::sphere::Sphere;
use crate::lib::texture::{CheckerTexture, ImageTexture, NoiseTexture, Texture};
use crate::lib::vec3::Vec3;
use crate::lib::writer::{PpmWriter, Writer};

fn main() {
    let scene_idx = 10;
    let render_settings = CameraBuilder::new()
        .width(800)
        .samples_per_pixel(4000)
        .max_depth(40);

    let (camera_builder, raw_scene) = match scene_idx {
        1 => (camera_a(render_settings), scene_a()),
        2 => (camera_b(render_settings), scene_b()),
        3 => (camera_c(render_settings), scene_c()),
        4 => (camera_d(render_settings), scene_d()),
        5 => (camera_e(render_settings), scene_e()),
        6 => (camera_f(render_settings), scene_f()),
        7 => (camera_g(render_settings), scene_g()),
        8 => (camera_cornell(render_settings), scene_cornell()),
        9 => (camera_cornell(render_settings), scene_cornell_smoke()),
        10 => (camera_book2_final(render_settings), scene_book2_final(false)),
        _ => panic!(),
    };
    let camera = camera_builder.build();
    let scene = wrap_scene(raw_scene);

    let writer: Arc<dyn Writer> = Arc::new(PpmWriter::new(camera.width(), camera.height(), 255));
    writer.init();
    camera.render(Arc::clone(&scene), Arc::clone(&writer));
    writer.close();
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
        Vec3::new_arr(center),
        radius,
        Arc::clone(&mat),
    ))
}

fn textured_sphere(center: [f64; 3], radius: f64, texture: Arc<dyn Texture>) -> Arc<dyn Hittable> {
    let mat: Arc<dyn Material> = Arc::new(Lambertian::new(texture));
    new_sphere(center, radius, mat)
}

fn quad(q: [f64; 3], u: [f64; 3], v: [f64; 3], mat: Arc<dyn Material>) -> Arc<dyn Hittable> {
    Arc::new(Quad::new(
        Vec3::new_arr(q),
        Vec3::new_arr(u),
        Vec3::new_arr(v),
        mat,
    ))
}

fn box3d(a: [f64; 3], b: [f64; 3], mat: Arc<dyn Material>) -> Arc<dyn Hittable> {
    Arc::new(Box3d::new(
        Vec3::new_arr(a),
        Vec3::new_arr(b),
        Arc::clone(&mat),
    ))
}

fn lambertian(color: [f64; 3]) -> Arc<dyn Material> {
    Arc::new(Lambertian::from_color_values(color))
}

fn diffuse_light(color: [f64; 3]) -> Arc<dyn Material> {
    Arc::new(DiffuseLight::from_color(Color::from_arr(color)))
}

fn camera_a(builder: CameraBuilder) -> CameraBuilder {
    builder
        .vfov(50.0)
        .lookfrom([-1.0, 1.0, 1.0])
        .lookat([0.0, 0.0, -1.0])
        .defocus_angle(0.5)
        .focus_dist(3.4)
}

fn scene_a() -> Scene {
    let material_ground: Arc<dyn Material> =
        Arc::new(Lambertian::from_color_values([0.8, 0.8, 0.0]));
    let material_center: Arc<dyn Material> =
        Arc::new(Lambertian::from_color_values([0.1, 0.2, 0.5]));
    let material_left: Arc<dyn Material> = Arc::new(Dielectric::new(1.5));
    let material_bubble: Arc<dyn Material> = Arc::new(Dielectric::new(1.0 / 1.5));
    let material_right: Arc<dyn Material> = Arc::new(Metal::new([0.8, 0.6, 0.2], 0.2));

    let sphere1: Arc<dyn Hittable> = Arc::new(Sphere::stationary(
        Vec3::new(1.0, -100.5, -1.0),
        100.0,
        Arc::clone(&material_ground),
    ));
    let sphere2: Arc<dyn Hittable> = Arc::new(Sphere::stationary(
        Vec3::new(0.0, 0.0, -1.2),
        0.5,
        Arc::clone(&material_center),
    ));
    let sphere3: Arc<dyn Hittable> = Arc::new(Sphere::stationary(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        Arc::clone(&material_left),
    ));
    let sphere4: Arc<dyn Hittable> = Arc::new(Sphere::stationary(
        Vec3::new(-1.0, 0.0, -1.0),
        0.4,
        Arc::clone(&material_bubble),
    ));
    let sphere5: Arc<dyn Hittable> = Arc::new(Sphere::stationary(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        Arc::clone(&material_right),
    ));

    let mut scene = Scene::new();
    scene.add(Arc::clone(&sphere1));
    scene.add(Arc::clone(&sphere2));
    scene.add(Arc::clone(&sphere3));
    scene.add(Arc::clone(&sphere4));
    scene.add(Arc::clone(&sphere5));
    scene
}

fn camera_b(builder: CameraBuilder) -> CameraBuilder {
    builder
        .lookfrom([13.0, 2.0, 3.0])
        .lookat([0.0, 0.0, 0.0])
        .defocus_angle(0.6)
        .focus_dist(10.0)
}

fn scene_b() -> Scene {
    // Textures
    let checker_texture: Arc<dyn Texture> = Arc::new(CheckerTexture::from_color_values(
        0.32,
        [0.2, 0.3, 0.1],
        [0.9, 0.9, 0.9],
    ));
    let perlin_texture: Arc<dyn Texture> = Arc::new(NoiseTexture::new(8.0));

    // Materials
    let mat_ground: Arc<dyn Material> = Arc::new(Lambertian::new(checker_texture.clone()));
    let mat1: Arc<dyn Material> = Arc::new(Dielectric::new(1.5));
    let mat2: Arc<dyn Material> = Arc::new(Lambertian::from_color_values([0.4, 0.2, 0.1]));
    let mat3: Arc<dyn Material> = Arc::new(Metal::new([0.7, 0.6, 0.5], 0.0));

    // Objects
    let sphere_ground: Arc<dyn Hittable> = Arc::new(Sphere::stationary(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        mat_ground.clone(),
    ));
    let sphere1: Arc<dyn Hittable> = Arc::new(Sphere::stationary(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Arc::clone(&mat1),
    ));
    let sphere2: Arc<dyn Hittable> = Arc::new(Sphere::stationary(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Arc::clone(&mat2),
    ));
    let sphere3: Arc<dyn Hittable> = Arc::new(Sphere::stationary(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Arc::clone(&mat3),
    ));

    let mut scene = Scene::new();
    scene.add(Arc::clone(&sphere_ground));
    scene.add(Arc::clone(&sphere1));
    scene.add(Arc::clone(&sphere2));
    scene.add(Arc::clone(&sphere3));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand();
            let center = Vec3::new(a as f64 + 0.9 * rand(), 0.2, b as f64 + 0.9 * rand());

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.4 {
                    // Diffuse
                    let albedo = rand_arr3();
                    let mat_sphere: Arc<dyn Material> =
                        Arc::new(Lambertian::from_color_values(albedo));
                    let center2 = center + Vec3::new(0.0, rand_range(0.0, 0.5), 0.0);
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

fn camera_c(builder: CameraBuilder) -> CameraBuilder {
    builder.lookfrom([13.0, 2.0, 3.0]).lookat([0.0, 0.0, 0.0])
}

fn scene_c() -> Scene {
    let checker_texture: Arc<dyn Texture> = Arc::new(CheckerTexture::from_color_values(
        0.32,
        [0.2, 0.3, 0.1],
        [0.9, 0.9, 0.9],
    ));
    let mat_ground: Arc<dyn Material> = Arc::new(Lambertian::new(checker_texture.clone()));

    let sphere1: Arc<dyn Hittable> = Arc::new(Sphere::stationary(
        Vec3::new(0.0, -10.0, 0.0),
        10.0,
        mat_ground.clone(),
    ));
    let sphere2: Arc<dyn Hittable> = Arc::new(Sphere::stationary(
        Vec3::new(0.0, 10.0, 0.0),
        10.0,
        mat_ground.clone(),
    ));

    let mut scene = Scene::new();
    scene.add(Arc::clone(&sphere1));
    scene.add(Arc::clone(&sphere2));
    scene
}

fn camera_d(builder: CameraBuilder) -> CameraBuilder {
    builder.lookfrom([0.0, 4.0, 16.0]).lookat([0.0, 1.5, 0.0])
}

fn scene_d() -> Scene {
    let earth_texture: Arc<dyn Texture> = Arc::new(ImageTexture::new(
        "/Users/alex/src/okalex/rt-in-a-weekend/img/earthmap.jpg",
    ));
    let earth_surface: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::clone(&earth_texture)));
    let globe: Arc<dyn Hittable> = Arc::new(Sphere::stationary(
        Vec3::new(0.0, 2.0, 0.0),
        2.0,
        earth_surface.clone(),
    ));

    let checker_texture: Arc<dyn Texture> = Arc::new(CheckerTexture::from_color_values(
        0.32,
        [0.2, 0.3, 0.1],
        [0.9, 0.9, 0.9],
    ));
    let mat_ground: Arc<dyn Material> = Arc::new(Lambertian::new(checker_texture.clone()));
    let sphere_ground: Arc<dyn Hittable> = Arc::new(Sphere::stationary(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        mat_ground.clone(),
    ));

    let mut scene = Scene::new();
    scene.add(Arc::clone(&globe));
    scene.add(Arc::clone(&sphere_ground));
    scene
}

fn camera_e(builder: CameraBuilder) -> CameraBuilder {
    builder.lookfrom([13.0, 2.0, 3.0]).lookat([0.0, 0.0, 0.0])
}

fn scene_e() -> Scene {
    let perlin_texture: Arc<dyn Texture> = Arc::new(NoiseTexture::new(4.0));
    let perlin_surface: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::clone(&perlin_texture)));
    let sphere1: Arc<dyn Hittable> = Arc::new(Sphere::stationary(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        perlin_surface.clone(),
    ));
    let sphere2: Arc<dyn Hittable> = Arc::new(Sphere::stationary(
        Vec3::new(0.0, 2.0, 0.0),
        2.0,
        perlin_surface.clone(),
    ));

    let mut scene = Scene::new();
    scene.add(Arc::clone(&sphere1));
    scene.add(Arc::clone(&sphere2));
    scene
}

fn camera_f(builder: CameraBuilder) -> CameraBuilder {
    builder
        .aspect_ratio(1.0)
        .vfov(80.0)
        .lookfrom([0.0, 0.0, 9.0])
        .lookat([0.0, 0.0, 0.0])
}

fn scene_f() -> Scene {
    let left_red: Arc<dyn Material> = Arc::new(Lambertian::from_color_values([1.0, 0.2, 0.2]));
    let back_green: Arc<dyn Material> = Arc::new(Lambertian::from_color_values([0.2, 1.0, 0.2]));
    let right_blue: Arc<dyn Material> = Arc::new(Lambertian::from_color_values([0.2, 0.2, 1.0]));
    let upper_orange: Arc<dyn Material> = Arc::new(Lambertian::from_color_values([1.0, 0.5, 0.0]));
    let lower_teal: Arc<dyn Material> = Arc::new(Lambertian::from_color_values([0.2, 0.8, 0.8]));

    let left: Arc<dyn Hittable> = Arc::new(Quad::new(
        Vec3::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        Arc::clone(&left_red),
    ));
    let back: Arc<dyn Hittable> = Arc::new(Quad::new(
        Vec3::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        Arc::clone(&back_green),
    ));
    let right: Arc<dyn Hittable> = Arc::new(Quad::new(
        Vec3::new(3.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        Arc::clone(&right_blue),
    ));
    let upper: Arc<dyn Hittable> = Arc::new(Quad::new(
        Vec3::new(-2.0, 3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        Arc::clone(&upper_orange),
    ));
    let lower: Arc<dyn Hittable> = Arc::new(Quad::new(
        Vec3::new(-2.0, -3.0, 5.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -4.0),
        Arc::clone(&lower_teal),
    ));

    let mut scene = Scene::new();
    scene.add(Arc::clone(&left));
    scene.add(Arc::clone(&back));
    scene.add(Arc::clone(&right));
    scene.add(Arc::clone(&upper));
    scene.add(Arc::clone(&lower));
    scene
}

fn camera_g(builder: CameraBuilder) -> CameraBuilder {
    builder
        .lookfrom([26.0, 3.0, 6.0])
        .lookat([0.0, 2.0, 0.0])
        .background([0.0, 0.0, 0.0])
}

fn scene_g() -> Scene {
    let pertext: Arc<dyn Texture> = Arc::new(NoiseTexture::new(4.0));
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

fn camera_cornell(builder: CameraBuilder) -> CameraBuilder {
    builder
        .aspect_ratio(1.0)
        .background([0.0, 0.0, 0.0])
        .vfov(40.0)
        .lookfrom([278.0, 278.0, -800.0])
        .lookat([278.0, 278.0, 0.0])
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
        Color::new(0.0, 0.0, 0.0),
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

fn camera_book2_final(builder: CameraBuilder) -> CameraBuilder {
    builder
        .aspect_ratio(1.0)
        .background([0.01, 0.01, 0.01])
        .vfov(40.0)
        .lookfrom([478.0, 278.0, -600.0])
        .lookat([278.0, 278.0, 0.0])
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
    let center1 = Vec3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
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
    // if with_haze {
    //     let boundary = new_sphere([0.0, 0.0, 0.0], 5000.0, Arc::clone(&glass));
    //     let medium: Arc<dyn Hittable> = Arc::new(ConstantMedium::from_color(
    //         boundary,
    //         0.001,
    //         Color::new(1.0, 1.0, 1.0),
    //     ));
    //     scene.add(Arc::clone(&medium));
    // }

    // Globe
    // let earth_texture: Arc<dyn Texture> = Arc::new(ImageTexture::new(
    //     "/Users/alex/src/okalex/rt-in-a-weekend/img/earthmap.jpg",
    // ));
    // let earth_surface: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::clone(&earth_texture)));
    // let globe: Arc<dyn Hittable> = new_sphere([400.0, 200.0, 400.0],100.0, Arc::clone(&earth_surface.clone()));
    // scene.add(Arc::clone(&globe));

    // Noisy ball
    let pertext: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::new(NoiseTexture::new(0.2))));
    let sphere = new_sphere([220.0, 280.0, 300.0], 80.0, Arc::clone(&pertext));
    scene.add(Arc::clone(&sphere));

    // Bubbly box
    let white = lambertian([0.73, 0.73, 0.73]);
    let mut boxes2 = Scene::new();
    for j in 0..1000 {
        let sphere: Arc<dyn Hittable> = Arc::new(Sphere::stationary(
            Vec3::rand_range(0.0, 165.0),
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
