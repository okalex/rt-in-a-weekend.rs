mod lib;

use std::sync::Arc;

use crate::lib::bvh_node::BvhNode;
use crate::lib::camera::{Camera, CameraOptions};
use crate::lib::color::Color;
use crate::lib::hittable::Hittable;
use crate::lib::material::{Material, Lambertian, dielectric, metal};
use crate::lib::random::{rand, rand_range};
use crate::lib::scene::Scene;
use crate::lib::sphere::Sphere;
use crate::lib::texture::{CheckerTexture, ImageTexture, NoiseTexture, Texture};
use crate::lib::vec3::Vec3;
use crate::lib::writer::{PpmWriter, Writer};

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let img_width = 600;
    let img_height = (img_width as f32/ aspect_ratio) as u32;

    let camera = camera_b(img_width, img_height);
    let writer: Arc<dyn Writer> = Arc::new(PpmWriter::new(img_width, img_height, 255));

    let bvh: Arc<dyn Hittable> = Arc::new(BvhNode::from_scene(scene_b()));
    let scene = Arc::new(Scene::new_obj(Arc::clone(&bvh)));

    writer.init();
    camera.render(Arc::clone(&scene), Arc::clone(&writer));
    writer.close();
}

fn rand_arr3() -> [f64; 3] {
    [rand(), rand(), rand()]
}

fn camera_a(img_width: u32, img_height: u32) -> Camera {
    let camera_options = CameraOptions {
        img_width: img_width,
        img_height: img_height,
        vfov: 50.0,
        lookfrom: Vec3::new(-1.0, 1.0, 1.0),
        lookat: Vec3::new(0.0, 0.0, -1.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
        defocus_angle: 0.5,
        focus_dist: 3.4,
        samples_per_pixel: 100,
        max_depth: 50,
        use_multithreading: true,
    };
    Camera::new(camera_options)
}

fn scene_a() -> Scene {
    let material_ground: Arc<dyn Material> = Arc::new(Lambertian::from_color_values([0.8, 0.8, 0.0]));
    let material_center: Arc<dyn Material> = Arc::new(Lambertian::from_color_values([0.1, 0.2, 0.5]));
    let material_left: Arc<dyn Material> = Arc::new(dielectric(1.5));
    let material_bubble: Arc<dyn Material> = Arc::new(dielectric(1.0/1.5));
    let material_right: Arc<dyn Material> = Arc::new(metal([0.8, 0.6, 0.2], 0.2));

    let sphere1: Arc<dyn Hittable> = Arc::new(Sphere::stationary(Vec3::new(1.0, -100.5, -1.0), 100.0, Arc::clone(&material_ground)));
    let sphere2: Arc<dyn Hittable> = Arc::new(Sphere::stationary(Vec3::new(0.0, 0.0, -1.2), 0.5, Arc::clone(&material_center)));
    let sphere3: Arc<dyn Hittable> = Arc::new(Sphere::stationary(Vec3::new(-1.0, 0.0, -1.0), 0.5, Arc::clone(&material_left)));
    let sphere4: Arc<dyn Hittable> = Arc::new(Sphere::stationary(Vec3::new(-1.0, 0.0, -1.0), 0.4, Arc::clone(&material_bubble)));
    let sphere5: Arc<dyn Hittable> = Arc::new(Sphere::stationary(Vec3::new(1.0, 0.0, -1.0), 0.5, Arc::clone(&material_right)));

    let mut scene = Scene::new();
    scene.add(Arc::clone(&sphere1));
    scene.add(Arc::clone(&sphere2));
    scene.add(Arc::clone(&sphere3));
    scene.add(Arc::clone(&sphere4));
    scene.add(Arc::clone(&sphere5));
    scene
}

fn camera_b(img_width: u32, img_height: u32) -> Camera {
    let camera_options = CameraOptions {
        img_width: img_width,
        img_height: img_height,
        vfov: 20.0,
        lookfrom: Vec3::new(13.0, 2.0, 3.0),
        lookat: Vec3::new(0.0, 0.0, 0.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
        defocus_angle: 0.6,
        focus_dist: 10.0,
        samples_per_pixel: 50,
        max_depth: 50,
        use_multithreading: true,
    };
    Camera::new(camera_options)
}

fn scene_b() -> Scene {
    // Textures
    let checker_texture: Arc<dyn Texture> = Arc::new(
        CheckerTexture::from_color_values(0.32, [0.2, 0.3, 0.1], [0.9, 0.9, 0.9])
    );
    let perlin_texture: Arc<dyn Texture> = Arc::new(NoiseTexture::new(8.0));

    // Materials
    let mat_ground: Arc<dyn Material> = Arc::new(Lambertian::new(checker_texture.clone()));
    let mat1: Arc<dyn Material> = Arc::new(dielectric(1.5));
    let mat2: Arc<dyn Material> = Arc::new(Lambertian::from_color_values([0.4, 0.2, 0.1]));
    let mat3: Arc<dyn Material> = Arc::new(metal([0.7, 0.6, 0.5], 0.0));

    // Objects
    let sphere_ground: Arc<dyn Hittable> = Arc::new(Sphere::stationary(Vec3::new(0.0, -1000.0, 0.0), 1000.0, mat_ground.clone()));
    let sphere1: Arc<dyn Hittable> = Arc::new(Sphere::stationary(Vec3::new(0.0, 1.0, 0.0), 1.0, Arc::clone(&mat1)));
    let sphere2: Arc<dyn Hittable> = Arc::new(Sphere::stationary(Vec3::new(-4.0, 1.0, 0.0), 1.0, Arc::clone(&mat2)));
    let sphere3: Arc<dyn Hittable> = Arc::new(Sphere::stationary(Vec3::new(4.0, 1.0, 0.0), 1.0, Arc::clone(&mat3)));

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
                    let mat_sphere: Arc<dyn Material> = Arc::new(Lambertian::from_color_values(albedo));
                    let center2 = center + Vec3::new(0.0, rand_range(0.0, 0.5), 0.0);
                    let sphere: Arc<dyn Hittable> = Arc::new(Sphere::moving(center, center2, 0.2, Arc::clone(&mat_sphere)));
                    scene.add(Arc::clone(&sphere));

                } else if choose_mat < 0.6 {
                    // Metal
                    let albedo = rand_arr3();
                    let fuzz = rand_range(0.0, 0.5);
                    let mat_sphere: Arc<dyn Material> = Arc::new(metal(albedo, fuzz));
                    let sphere: Arc<dyn Hittable> = Arc::new(Sphere::stationary(center, 0.2, Arc::clone(&mat_sphere)));
                    scene.add(Arc::clone(&sphere));

                } else if choose_mat < 0.8 {
                    // Glass
                    let mat_sphere: Arc<dyn Material> = Arc::new(dielectric(1.5));
                    let sphere: Arc<dyn Hittable> = Arc::new(Sphere::stationary(center, 0.2, Arc::clone(&mat_sphere)));
                    scene.add(Arc::clone(&sphere));

                } else {
                    // Marble
                    let mat_sphere: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::clone(&perlin_texture)));
                    let sphere: Arc<dyn Hittable> = Arc::new(Sphere::stationary(center, 0.2, Arc::clone(&mat_sphere)));
                    scene.add(Arc::clone(&sphere));
                }
            }
        }
    }

    scene
}

fn camera_c(img_width: u32, img_height: u32) -> Camera {
    let camera_options = CameraOptions {
        img_width: img_width,
        img_height: img_height,
        vfov: 20.0,
        lookfrom: Vec3::new(13.0, 2.0, 3.0),
        lookat: Vec3::new(0.0, 0.0, 0.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
        defocus_angle: 0.0,
        focus_dist: 3.4,
        samples_per_pixel: 100,
        max_depth: 50,
        use_multithreading: true,
    };
    Camera::new(camera_options)
}

fn scene_c() -> Scene {
    let checker_texture: Arc<dyn Texture> = Arc::new(
        CheckerTexture::from_color_values(0.32, [0.2, 0.3, 0.1], [0.9, 0.9, 0.9])
    );
    let mat_ground: Arc<dyn Material> = Arc::new(Lambertian::new(checker_texture.clone()));

    let sphere1: Arc<dyn Hittable> = Arc::new(Sphere::stationary(Vec3::new(0.0, -10.0, 0.0), 10.0, mat_ground.clone()));
    let sphere2: Arc<dyn Hittable> = Arc::new(Sphere::stationary(Vec3::new(0.0, 10.0, 0.0), 10.0, mat_ground.clone()));

    let mut scene = Scene::new();
    scene.add(Arc::clone(&sphere1));
    scene.add(Arc::clone(&sphere2));
    scene
}

fn camera_d(img_width: u32, img_height: u32) -> Camera {
    let camera_options = CameraOptions {
        img_width: img_width,
        img_height: img_height,
        vfov: 20.0,
        lookfrom: Vec3::new(0.0, 4.0, 16.0),
        lookat: Vec3::new(0.0, 1.5, 0.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
        defocus_angle: 0.0,
        focus_dist: 3.4,
        samples_per_pixel: 100,
        max_depth: 50,
        use_multithreading: true,
    };
    Camera::new(camera_options)
}

fn scene_d() -> Scene {
    let earth_texture: Arc<dyn Texture> = Arc::new(
        ImageTexture::new("/Users/alex/src/okalex/rt-in-a-weekend/img/earthmap.jpg")
    );
    let earth_surface: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::clone(&earth_texture)));
    let globe: Arc<dyn Hittable> = Arc::new(Sphere::stationary(Vec3::new(0.0, 2.0, 0.0), 2.0, earth_surface.clone()));

    let checker_texture: Arc<dyn Texture> = Arc::new(
        CheckerTexture::from_color_values(0.32, [0.2, 0.3, 0.1], [0.9, 0.9, 0.9])
    );
    let mat_ground: Arc<dyn Material> = Arc::new(Lambertian::new(checker_texture.clone()));
    let sphere_ground: Arc<dyn Hittable> = Arc::new(Sphere::stationary(Vec3::new(0.0, -1000.0, 0.0), 1000.0, mat_ground.clone()));

    let mut scene = Scene::new();
    scene.add(Arc::clone(&globe));
    scene.add(Arc::clone(&sphere_ground));
    scene
}

fn camera_e(img_width: u32, img_height: u32) -> Camera {
    let camera_options = CameraOptions {
        img_width: img_width,
        img_height: img_height,
        vfov: 20.0,
        lookfrom: Vec3::new(13.0, 2.0, 3.0),
        lookat: Vec3::new(0.0, 0.0, 0.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
        defocus_angle: 0.0,
        focus_dist: 3.4,
        samples_per_pixel: 100,
        max_depth: 50,
        use_multithreading: true,
    };
    Camera::new(camera_options)
}

fn scene_e() -> Scene {
    let perlin_texture: Arc<dyn Texture> = Arc::new(NoiseTexture::new(4.0));
    let perlin_surface: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::clone(&perlin_texture)));
    let sphere1: Arc<dyn Hittable> = Arc::new(Sphere::stationary(Vec3::new(0.0, -1000.0, 0.0), 1000.0, perlin_surface.clone()));
    let sphere2: Arc<dyn Hittable> = Arc::new(Sphere::stationary(Vec3::new(0.0, 2.0, 0.0), 2.0, perlin_surface.clone()));

    let mut scene = Scene::new();
    scene.add(Arc::clone(&sphere1));
    scene.add(Arc::clone(&sphere2));
    scene
}
