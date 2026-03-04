mod lib;
use std::sync::Arc;
use crate::lib::writer::Writer;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let img_width = 400;
    let img_height = (img_width as f32/ aspect_ratio) as u32;

    let camera = camera_a(img_width, img_height);
    let writer: Arc<dyn Writer> = Arc::new(lib::writer::new_ppm_writer(img_width, img_height, 255));

    let scene = Arc::new(scene_a());

    writer.init();
    camera.render(Arc::clone(&scene), Arc::clone(&writer));
    writer.close();
}

fn rand_arr3() -> [f64; 3] {
    return [lib::random::rand(), lib::random::rand(), lib::random::rand()];
}

fn camera_a(img_width: u32, img_height: u32) -> lib::camera::Camera {
    let camera_options = lib::camera::CameraOptions {
        img_width: img_width,
        img_height: img_height,
        vfov: 50.0,
        lookfrom: lib::vec3::new(-1.0, 1.0, 1.0),
        lookat: lib::vec3::new(0.0, 0.0, -1.0),
        vup: lib::vec3::new(0.0, 1.0, 0.0),
        defocus_angle: 0.5,
        focus_dist: 3.4,
        samples_per_pixel: 100,
        max_depth: 50,
        use_multithreading: true,
    };
    return lib::camera::new(camera_options);
}

fn scene_a() -> lib::scene::Scene {
    let material_ground: Arc<dyn lib::material::Material> = Arc::new(lib::material::lambertian([0.8, 0.8, 0.0]));
    let material_center: Arc<dyn lib::material::Material> = Arc::new(lib::material::lambertian([0.1, 0.2, 0.5]));
    let material_left: Arc<dyn lib::material::Material> = Arc::new(lib::material::dielectric(1.5));
    let material_bubble: Arc<dyn lib::material::Material> = Arc::new(lib::material::dielectric(1.0/1.5));
    let material_right: Arc<dyn lib::material::Material> = Arc::new(lib::material::metal([0.8, 0.6, 0.2], 0.2));

    let sphere1: Arc<dyn lib::hittable::Hittable> = Arc::new(lib::sphere::new(lib::vec3::new(1.0, -100.5, -1.0), 100.0, Arc::clone(&material_ground)));
    let sphere2: Arc<dyn lib::hittable::Hittable> = Arc::new(lib::sphere::new(lib::vec3::new(0.0, 0.0, -1.2), 0.5, Arc::clone(&material_center)));
    let sphere3: Arc<dyn lib::hittable::Hittable> = Arc::new(lib::sphere::new(lib::vec3::new(-1.0, 0.0, -1.0), 0.5, Arc::clone(&material_left)));
    let sphere4: Arc<dyn lib::hittable::Hittable> = Arc::new(lib::sphere::new(lib::vec3::new(-1.0, 0.0, -1.0), 0.4, Arc::clone(&material_bubble)));
    let sphere5: Arc<dyn lib::hittable::Hittable> = Arc::new(lib::sphere::new(lib::vec3::new(1.0, 0.0, -1.0), 0.5, Arc::clone(&material_right)));

    return lib::scene::new(vec![
        Arc::clone(&sphere1),
        Arc::clone(&sphere2),
        Arc::clone(&sphere3),
        Arc::clone(&sphere4),
        Arc::clone(&sphere5),
    ]);
}

fn camera_b(img_width: u32, img_height: u32) -> lib::camera::Camera {
    let camera_options = lib::camera::CameraOptions {
        img_width: img_width,
        img_height: img_height,
        vfov: 20.0,
        lookfrom: lib::vec3::new(13.0, 2.0, 3.0),
        lookat: lib::vec3::new(0.0, 0.0, 0.0),
        vup: lib::vec3::new(0.0, 1.0, 0.0),
        defocus_angle: 0.6,
        focus_dist: 10.0,
        samples_per_pixel: 50,
        max_depth: 5,
        use_multithreading: true,
    };
    return lib::camera::new(camera_options);
}

fn scene_b() -> lib::scene::Scene {
    let mat_ground: Arc<dyn lib::material::Material> = Arc::new(lib::material::lambertian([0.5, 0.5, 0.5]));
    let mat1: Arc<dyn lib::material::Material> = Arc::new(lib::material::dielectric(1.5));
    let mat2: Arc<dyn lib::material::Material> = Arc::new(lib::material::lambertian([0.4, 0.2, 0.1]));
    let mat3: Arc<dyn lib::material::Material> = Arc::new(lib::material::metal([0.7, 0.6, 0.5], 0.0));

    let sphere_ground: Arc<dyn lib::hittable::Hittable> = Arc::new(lib::sphere::new(lib::vec3::new(0.0, -1000.0, 0.0), 1000.0, Arc::clone(&mat_ground)));
    let sphere1: Arc<dyn lib::hittable::Hittable> = Arc::new(lib::sphere::new(lib::vec3::new(0.0, 1.0, 0.0), 1.0, Arc::clone(&mat1)));
    let sphere2: Arc<dyn lib::hittable::Hittable> = Arc::new(lib::sphere::new(lib::vec3::new(-4.0, 1.0, 0.0), 1.0, Arc::clone(&mat2)));
    let sphere3: Arc<dyn lib::hittable::Hittable> = Arc::new(lib::sphere::new(lib::vec3::new(4.0, 1.0, 0.0), 1.0, Arc::clone(&mat3)));

    let mut scene = lib::scene::new(vec![
        Arc::clone(&sphere_ground),
        Arc::clone(&sphere1),
        Arc::clone(&sphere2),
        Arc::clone(&sphere3),
    ]);

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = lib::random::rand();
            let center = lib::vec3::new(a as f64 + 0.9 * lib::random::rand(), 0.2, b as f64 + 0.9 * lib::random::rand());

            if (center - lib::vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // Diffuse
                    let albedo = rand_arr3();
                    let mat_sphere: Arc<dyn lib::material::Material> = Arc::new(lib::material::lambertian(albedo));
                    let sphere: Arc<dyn lib::hittable::Hittable> = Arc::new(lib::sphere::new(center, 0.2, Arc::clone(&mat_sphere)));
                    scene.add(Arc::clone(&sphere));

                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo = rand_arr3();
                    let fuzz = lib::random::rand_range(0.0, 0.5);
                    let mat_sphere: Arc<dyn lib::material::Material> = Arc::new(lib::material::metal(albedo, fuzz));
                    let sphere: Arc<dyn lib::hittable::Hittable> = Arc::new(lib::sphere::new(center, 0.2, Arc::clone(&mat_sphere)));
                    scene.add(Arc::clone(&sphere));

                } else {
                    // Glass
                    let mat_sphere: Arc<dyn lib::material::Material> = Arc::new(lib::material::dielectric(1.5));
                    let sphere: Arc<dyn lib::hittable::Hittable> = Arc::new(lib::sphere::new(center, 0.2, Arc::clone(&mat_sphere)));
                    scene.add(Arc::clone(&sphere));
                }
            }
        }
    }

    return scene;
}
