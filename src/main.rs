mod lib;

use std::sync::Arc;

use crate::lib::bvh_node::BvhNode;
use crate::lib::camera::CameraBuilder;
use crate::lib::color::Color;
use crate::lib::hittable::Hittable;
use crate::lib::material::{DiffuseLight, Lambertian, Material, dielectric, metal};
use crate::lib::quad::Quad;
use crate::lib::random::{rand, rand_range};
use crate::lib::scene::Scene;
use crate::lib::sphere::Sphere;
use crate::lib::texture::{CheckerTexture, ImageTexture, NoiseTexture, Texture};
use crate::lib::vec3::Vec3;
use crate::lib::writer::{PpmWriter, Writer};

fn main() {
    let camera = camera_g().build();
    let scene = make_scene(scene_g());

    let writer: Arc<dyn Writer> = Arc::new(PpmWriter::new(camera.width(), camera.height(), 255));
    writer.init();
    camera.render(Arc::clone(&scene), Arc::clone(&writer));
    writer.close();
}

fn rand_arr3() -> [f64; 3] {
    [rand(), rand(), rand()]
}

fn make_scene(scene: Scene) -> Arc<Scene> {
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

fn diffuse_light(color: [f64; 3]) -> Arc<dyn Material> {
    Arc::new(DiffuseLight::from_color(Color::from_arr(color)))
}

fn camera_a() -> CameraBuilder {
    CameraBuilder::new()
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
    let material_left: Arc<dyn Material> = Arc::new(dielectric(1.5));
    let material_bubble: Arc<dyn Material> = Arc::new(dielectric(1.0 / 1.5));
    let material_right: Arc<dyn Material> = Arc::new(metal([0.8, 0.6, 0.2], 0.2));

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

fn camera_b() -> CameraBuilder {
    CameraBuilder::new()
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
    let mat1: Arc<dyn Material> = Arc::new(dielectric(1.5));
    let mat2: Arc<dyn Material> = Arc::new(Lambertian::from_color_values([0.4, 0.2, 0.1]));
    let mat3: Arc<dyn Material> = Arc::new(metal([0.7, 0.6, 0.5], 0.0));

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
                    let mat_sphere: Arc<dyn Material> = Arc::new(metal(albedo, fuzz));
                    let sphere: Arc<dyn Hittable> =
                        Arc::new(Sphere::stationary(center, 0.2, Arc::clone(&mat_sphere)));
                    scene.add(Arc::clone(&sphere));
                } else if choose_mat < 0.8 {
                    // Glass
                    let mat_sphere: Arc<dyn Material> = Arc::new(dielectric(1.5));
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

fn camera_c() -> CameraBuilder {
    CameraBuilder::new()
        .lookfrom([13.0, 2.0, 3.0])
        .lookat([0.0, 0.0, 0.0])
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

fn camera_d() -> CameraBuilder {
    CameraBuilder::new()
        .lookfrom([0.0, 4.0, 16.0])
        .lookat([0.0, 1.5, 0.0])
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

fn camera_e() -> CameraBuilder {
    CameraBuilder::new()
        .lookfrom([13.0, 2.0, 3.0])
        .lookat([0.0, 0.0, 0.0])
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

fn camera_f() -> CameraBuilder {
    CameraBuilder::new()
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

fn camera_g() -> CameraBuilder {
    CameraBuilder::new()
        .lookfrom([26.0, 3.0, 6.0])
        .lookat([0.0, 2.0, 0.0])
        .background([0.0, 0.0, 0.0])
        .width(400)
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
