use std::sync::Arc;

use nalgebra::{Point3, Vector3};

use crate::rt::{
    camera::CameraOptions,
    color::Color,
    file::load_model_with_mat,
    materials::{
        dielectric::Dielectric, diffuse_light::DiffuseLight, lambertian::Lambertian,
        material::Material, metal::Metal, pbr_material::PbrMaterial,
    },
    objects::{
        bvh_node::BvhNode,
        constant_medium::ConstantMedium,
        hittable::{Hittable, rotate_y, translate},
        quad::Quad,
        scene::{Box3d, Scene},
        sphere::Sphere,
        triangle::Triangle,
    },
    random::{rand, rand_range, rand_range_vector},
    textures::{checkered::Checkered, image_map::ImageMap, noise::Noise, texture::Texture},
};

pub fn get_camera_and_scene(scene_idx: u32) -> (CameraOptions, Scene) {
    let (camera_options, raw_scene) = match scene_idx {
        1 => (camera_a(), scene_a()),
        2 => (camera_b(), scene_b()),
        3 => (camera_c(), scene_c()),
        4 => (camera_d(), scene_d()),
        5 => (camera_e(), scene_e()),
        6 => (camera_f(), scene_f()),
        7 => (camera_g(), scene_g()),
        8 => (camera_cornell(555.0), scene_cornell()),
        9 => (camera_cornell(555.0), scene_cornell_smoke()),
        10 => (camera_book2_final(), scene_book2_final(true)),
        11 => (camera_triangle(), scene_triangle()),
        12 => (camera_cornell(10.0), scene_obj(10.0)),
        13 => (camera_cornell(550.0), scene_pbr(550.0)),
        _ => panic!(),
    };
    (camera_options, raw_scene)
}

fn rand_arr3() -> [f64; 3] {
    [rand(), rand(), rand()]
}

struct Textures {}

impl Textures {
    fn checkers() -> Arc<dyn Texture> {
        Arc::new(Checkered::from_color_values(
            0.32,
            [0.2, 0.3, 0.1],
            [0.9, 0.9, 0.9],
        ))
    }
}

struct Materials {
    default: Arc<dyn Material>,
    red: Arc<dyn Material>,
    white: Arc<dyn Material>,
    green: Arc<dyn Material>,
    blue: Arc<dyn Material>,
    orange: Arc<dyn Material>,
    teal: Arc<dyn Material>,
    diffuse_light: Arc<dyn Material>,
    checkered: Arc<dyn Material>,
    glass: Arc<dyn Material>,
    air: Arc<dyn Material>,
    mirror: Arc<dyn Material>,
    gold: Arc<dyn Material>,
    stone: Arc<dyn Material>,
    rusty_metal: Arc<dyn Material>,
    marble: Arc<dyn Material>,
    earth: Arc<dyn Material>,
    pbr: Arc<dyn Material>,
}

impl Materials {
    fn new() -> Self {
        Self {
            default: Self::lambertian([0.5, 0.5, 0.5]),
            red: Self::lambertian([0.65, 0.05, 0.05]),
            white: Self::lambertian([0.73, 0.73, 0.73]),
            green: Self::lambertian([0.12, 0.45, 0.15]),
            blue: Self::lambertian([0.1, 0.2, 0.5]),
            orange: Self::lambertian([1.0, 0.5, 0.0]),
            teal: Self::lambertian([0.2, 0.8, 0.8]),
            diffuse_light: Self::diffuse_light([7.0, 7.0, 7.0]),
            checkered: Self::from_texture(Textures::checkers()),
            glass: Self::dielectric(1.5),
            air: Self::dielectric(1.0 / 1.5),
            mirror: Self::metal([0.8, 0.85, 0.88], 0.0),
            gold: Self::metal([0.8, 0.6, 0.2], 0.2),
            stone: Self::image_map("assets/cube-diffuse.jpg", 1.0),
            rusty_metal: Self::image_map("assets/rusty-metal.jpg", 1.0),
            marble: Self::from_texture(Arc::new(Noise::new(8.0))),
            earth: Self::image_map("assets/earthmap.jpg", 1.0),
            pbr: Self::pbr([0.8, 0.6, 0.2], 0.7),
        }
    }

    fn get(&self, name: &str) -> Arc<dyn Material> {
        match name {
            "red" => Arc::clone(&self.red),
            "white" => Arc::clone(&self.white),
            "green" => Arc::clone(&self.green),
            "blue" => Arc::clone(&self.blue),
            "orange" => Arc::clone(&self.orange),
            "teal" => Arc::clone(&self.teal),
            "diffuse_light" => Arc::clone(&self.diffuse_light),
            "checkered" => Arc::clone(&self.checkered),
            "glass" => Arc::clone(&self.glass),
            "air" => Arc::clone(&self.air),
            "mirror" => Arc::clone(&self.mirror),
            "gold" => Arc::clone(&self.gold),
            "stone" => Arc::clone(&self.stone),
            "rusty_metal" => Arc::clone(&self.rusty_metal),
            "marble" => Arc::clone(&self.marble),
            "earth" => Arc::clone(&self.earth),
            "pbr" => Arc::clone(&self.pbr),
            _ => Arc::clone(&self.default),
        }
    }

    fn lambertian(color: [f64; 3]) -> Arc<dyn Material> {
        Arc::new(Lambertian::from_color_values(color))
    }

    fn from_texture(texture: Arc<dyn Texture>) -> Arc<dyn Material> {
        Arc::new(Lambertian::new(texture))
    }

    fn rand_lambertian() -> Arc<dyn Material> {
        let albedo = rand_arr3();
        Self::lambertian(albedo)
    }

    fn dielectric(refraction_idx: f64) -> Arc<dyn Material> {
        Arc::new(Dielectric::new(refraction_idx))
    }

    fn metal(color: [f64; 3], fuzz: f64) -> Arc<dyn Material> {
        Arc::new(Metal::new(color, fuzz))
    }

    fn rand_metal() -> Arc<dyn Material> {
        let albedo = rand_arr3();
        let fuzz = rand_range(0.0, 0.5);
        Self::metal(albedo, fuzz)
    }

    fn diffuse_light(color: [f64; 3]) -> Arc<dyn Material> {
        Arc::new(DiffuseLight::from_color(Color::from_arr(color)))
    }

    fn image_map(file_name: &str, scale_factor: f64) -> Arc<dyn Material> {
        let tex = Arc::new(ImageMap::new(file_name, scale_factor));
        Self::from_texture(tex)
    }

    fn pbr(albedo: [f64; 3], metallicity: f64) -> Arc<dyn Material> {
        let color = Color::from_arr(albedo);
        Arc::new(PbrMaterial::new(color, metallicity))
    }
}

pub struct Shapes {}

impl Shapes {
    pub fn sphere(center: [f64; 3], radius: f64, mat: Arc<dyn Material>) -> Arc<dyn Hittable> {
        Arc::new(Sphere::stationary(
            Point3::from(center),
            radius,
            Arc::clone(&mat),
        ))
    }

    pub fn quad(
        q: [f64; 3],
        u: [f64; 3],
        v: [f64; 3],
        mat: Arc<dyn Material>,
    ) -> Arc<dyn Hittable> {
        Arc::new(Quad::from_arr(q, u, v, mat))
    }

    pub fn triangle(
        a: [f64; 3],
        b: [f64; 3],
        c: [f64; 3],
        mat: Arc<dyn Material>,
    ) -> Arc<dyn Hittable> {
        Arc::new(Triangle::new(a, b, c, mat))
    }

    pub fn box3d(a: [f64; 3], b: [f64; 3], mat: Arc<dyn Material>) -> Arc<dyn Hittable> {
        Arc::new(Box3d::new(
            Vector3::from(a),
            Vector3::from(b),
            Arc::clone(&mat),
        ))
    }

    pub fn constant_medium(
        boundary: Arc<dyn Hittable>,
        color: [f64; 3],
        density: f64,
    ) -> Arc<dyn Hittable> {
        Arc::new(ConstantMedium::from_color(
            boundary,
            density,
            Color::from_arr(color),
        ))
    }

    pub fn checkered_ground(materials: &Materials) -> Arc<dyn Hittable> {
        Shapes::sphere([1.0, -100.0, -1.0], 100.0, materials.get("checkered"))
    }
}

impl Scene {
    fn add_cornell_room(scene: &mut Scene, materials: &Materials, width: f64) {
        let light = Shapes::quad(
            [0.6 * width, width - 0.1, 0.6 * width],
            [-0.2 * width, 0.0, 0.0],
            [0.0, 0.0, -0.2 * width],
            materials.get("diffuse_light"),
        );
        let left = Shapes::quad(
            [width, 0.0, 0.0],
            [0.0, width, 0.0],
            [0.0, 0.0, width],
            materials.get("green"),
        );
        let right = Shapes::quad(
            [0.0, 0.0, 0.0],
            [0.0, width, 0.0],
            [0.0, 0.0, width],
            materials.get("red"),
        );
        let floor = Shapes::quad(
            [0.0, 0.0, 0.0],
            [width, 0.0, 0.0],
            [0.0, 0.0, width],
            materials.get("white"),
        );
        let ceiling = Shapes::quad(
            [width, width, width],
            [-width, 0.0, 0.0],
            [0.0, 0.0, -width],
            materials.get("white"),
        );
        let back = Shapes::quad(
            [0.0, 0.0, width],
            [width, 0.0, 0.0],
            [0.0, width, 0.0],
            materials.get("white"),
        );

        scene.add(left);
        scene.add(right);
        scene.add(floor);
        scene.add(ceiling);
        scene.add(back);
        scene.add(light);
    }
}

fn camera_a() -> CameraOptions {
    CameraOptions::new()
        .vfov(50.0)
        .position([-1.0, 1.0, 1.0])
        .target([0.0, 0.0, -1.0])
        .defocus_angle(0.5)
        .focus_dist(3.4)
}

fn scene_a() -> Scene {
    let mut scene = Scene::new();
    let materials = Materials::new();

    let ground = Shapes::checkered_ground(&materials);
    let sphere2 = Shapes::sphere([0.0, 0.0, -1.2], 0.5, materials.get("blue"));
    let sphere3 = Shapes::sphere([-1.0, 0.0, -1.0], 0.5, materials.get("glass"));
    let sphere4 = Shapes::sphere([-1.0, 0.0, -1.0], 0.4, materials.get("air"));
    let sphere5 = Shapes::sphere([1.0, 0.0, -1.0], 0.5, materials.get("gold"));

    scene.add(ground);
    scene.add(sphere2);
    scene.add(sphere3);
    scene.add(sphere4);
    scene.add(sphere5);
    scene
}

fn camera_triangle() -> CameraOptions {
    CameraOptions::new()
        .vfov(50.0)
        .position([0.0, 1.0, 2.0])
        .target([0.0, 0.5, 0.0])
        .defocus_angle(0.5)
        .focus_dist(3.4)
}

fn scene_triangle() -> Scene {
    let mut scene = Scene::new();
    let materials = Materials::new();

    let ground = Shapes::checkered_ground(&materials);
    let tri1 = Shapes::triangle(
        [0.0, 1.0, -1.0],
        [-1.0, 0.0, -1.0],
        [1.0, 0.5, -1.0],
        materials.get("blue"),
    );

    scene.add(ground);
    scene.add(tri1);
    scene
}

fn scene_obj(scale: f64) -> Scene {
    let mut scene = Scene::new();
    let materials = Materials::new();

    Scene::add_cornell_room(&mut scene, &materials, scale);

    let objs = match load_model_with_mat("cube.obj", materials.get("rusty_metal")) {
        Ok(os) => os,
        _ => panic!(),
    };
    for obj in objs {
        let moved = translate(rotate_y(Arc::new(obj), 205.0), [10.0, 1.0, 5.0]);
        scene.add(moved);
    }

    scene
}

fn scene_pbr(scale: f64) -> Scene {
    let mut scene = Scene::new();
    let materials = Materials::new();

    Scene::add_cornell_room(&mut scene, &materials, scale);

    let purple_light = Materials::diffuse_light([8.0, 0.0, 10.0]);
    scene.add(Shapes::sphere([530.0, 530.0, 20.0], 10.0, purple_light));

    let teal_light = Materials::diffuse_light([0.0, 8.0, 10.0]);
    scene.add(Shapes::sphere([20.0, 530.0, 20.0], 10.0, teal_light));

    let mut box_right = Shapes::box3d(
        [0.0, 0.0, 0.0],
        [165.0, 165.0, 165.0],
        materials.get("white"),
    );
    box_right = rotate_y(box_right, -18.0);
    box_right = translate(box_right, [130.0, 0.0, 65.0]);
    scene.add(box_right);

    let mut box_left = Shapes::box3d(
        [0.0, 0.0, 0.0],
        [165.0, 330.0, 165.0],
        materials.get("white"),
    );
    box_left = rotate_y(box_left, 15.0);
    box_left = translate(box_left, [265.0, 0.0, 295.0]);
    scene.add(box_left);

    let mat = Materials::pbr([0.8, 0.6, 0.2], 0.9);
    let sphere = Shapes::sphere([420.0, 100.0, 180.0], 100.0, mat);
    scene.add(sphere);

    scene
}

fn camera_b() -> CameraOptions {
    CameraOptions::new()
        .position([13.0, 2.0, 3.0])
        .target([0.0, 0.0, 0.0])
        .defocus_angle(0.6)
        .focus_dist(10.0)
}

fn scene_b() -> Scene {
    let mut scene = Scene::new();
    let materials = Materials::new();

    // Objects
    let ground = Shapes::checkered_ground(&materials);
    let sphere1 = Shapes::sphere([0.0, 1.0, 0.0], 1.0, materials.get("glass"));
    let sphere2 = Shapes::sphere([-4.0, 1.0, 0.0], 1.0, materials.get("red"));
    let sphere3 = Shapes::sphere([4.0, 1.0, 0.0], 1.0, materials.get("gold"));

    scene.add(ground);
    scene.add(sphere1);
    scene.add(sphere2);
    scene.add(sphere3);

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand();
            let center = [a as f64 + 0.9 * rand(), 0.2, b as f64 + 0.9 * rand()];

            let center1 = Point3::from(center);
            if (center1 - Point3::new(4.0, 0.2, 0.0)).magnitude() > 0.9 {
                if choose_mat < 0.4 {
                    // Diffuse
                    let center2 = center1 + Vector3::new(0.0, rand_range(0.0, 0.5), 0.0);
                    let sphere: Arc<dyn Hittable> = Arc::new(Sphere::moving(
                        center1,
                        center2,
                        0.2,
                        Materials::rand_lambertian(),
                    ));
                    scene.add(sphere);
                } else if choose_mat < 0.6 {
                    // Metal
                    let sphere = Shapes::sphere(center, 0.2, Materials::rand_metal());
                    scene.add(sphere);
                } else if choose_mat < 0.8 {
                    // Glass
                    let sphere = Shapes::sphere(center, 0.2, materials.get("glass"));
                    scene.add(sphere);
                } else {
                    // Marble
                    let sphere = Shapes::sphere(center, 0.2, materials.get("marble"));
                    scene.add(sphere);
                }
            }
        }
    }

    scene
}

fn camera_c() -> CameraOptions {
    CameraOptions::new()
        .position([13.0, 2.0, 3.0])
        .target([0.0, 0.0, 0.0])
}

fn scene_c() -> Scene {
    let mut scene = Scene::new();
    let materials = Materials::new();

    let sphere1 = Shapes::sphere([0.0, -10.0, 0.0], 10.0, materials.get("checkered"));
    let sphere2 = Shapes::sphere([0.0, 10.0, 0.0], 10.0, materials.get("checkered"));

    scene.add(sphere1);
    scene.add(sphere2);
    scene
}

fn camera_d() -> CameraOptions {
    CameraOptions::new()
        .position([0.0, 4.0, 16.0])
        .target([0.0, 1.5, 0.0])
}

fn scene_d() -> Scene {
    let mut scene = Scene::new();
    let materials = Materials::new();

    let ground = Shapes::checkered_ground(&materials);
    let globe = Shapes::sphere([0.0, 2.0, 0.0], 2.0, materials.get("earth"));

    scene.add(globe);
    scene.add(ground);
    scene
}

fn camera_e() -> CameraOptions {
    CameraOptions::new()
        .position([13.0, 2.0, 3.0])
        .target([0.0, 0.0, 0.0])
}

fn scene_e() -> Scene {
    let mut scene = Scene::new();
    let materials = Materials::new();

    let ground = Shapes::checkered_ground(&materials);
    let sphere2 = Shapes::sphere([0.0, 2.0, 0.0], 2.0, materials.get("marble"));

    scene.add(ground);
    scene.add(sphere2);
    scene
}

fn camera_f() -> CameraOptions {
    CameraOptions::new()
        .vfov(80.0)
        .position([0.0, 0.0, 9.0])
        .target([0.0, 0.0, 0.0])
}

fn scene_f() -> Scene {
    let mut scene = Scene::new();
    let materials = Materials::new();

    let left = Shapes::quad(
        [-3.0, -2.0, 5.0],
        [0.0, 0.0, -4.0],
        [0.0, 4.0, 0.0],
        materials.get("red"),
    );
    let back = Shapes::quad(
        [-2.0, -2.0, 0.0],
        [4.0, 0.0, 0.0],
        [0.0, 4.0, 0.0],
        materials.get("green"),
    );
    let right = Shapes::quad(
        [3.0, -2.0, 1.0],
        [0.0, 0.0, 4.0],
        [0.0, 4.0, 0.0],
        materials.get("blue"),
    );
    let upper = Shapes::quad(
        [-2.0, 3.0, 1.0],
        [4.0, 0.0, 0.0],
        [0.0, 0.0, 4.0],
        materials.get("orange"),
    );
    let lower = Shapes::quad(
        [-2.0, -3.0, 5.0],
        [4.0, 0.0, 0.0],
        [0.0, 0.0, -4.0],
        materials.get("teal"),
    );

    scene.add(left);
    scene.add(back);
    scene.add(right);
    scene.add(upper);
    scene.add(lower);
    scene
}

fn camera_g() -> CameraOptions {
    CameraOptions::new()
        .position([26.0, 3.0, 6.0])
        .target([0.0, 2.0, 0.0])
}

fn scene_g() -> Scene {
    let mut scene = Scene::new();
    let materials = Materials::new();

    let ground = Shapes::checkered_ground(&materials);
    let sphere = Shapes::sphere([0.0, 2.0, 0.0], 2.0, materials.get("marble"));

    let quad_light = Shapes::quad(
        [3.0, 1.0, -2.0],
        [2.0, 0.0, 0.0],
        [0.0, 2.0, 0.0],
        materials.get("diffuse_light"),
    );
    let sphere_light = Shapes::sphere([0.0, 6.5, 0.0], 0.5, materials.get("diffuse_light"));

    scene.add(ground);
    scene.add(sphere);
    scene.add(quad_light);
    scene.add(sphere_light);
    scene
}

fn camera_cornell(room_width: f64) -> CameraOptions {
    CameraOptions::new()
        .vfov(40.0)
        .position([room_width / 2.0, room_width / 2.0, -1.44 * room_width])
        .target([room_width / 2.0, room_width / 2.0, 0.0])
}

fn scene_cornell() -> Scene {
    let mut scene = Scene::new();
    let materials = Materials::new();

    // let mut box_right = Shapes::box3d(
    //     [0.0, 0.0, 0.0],
    //     [165.0, 165.0, 165.0],
    //     materials.get("white"),
    // );
    // box_right = rotate_y(box_right, -18.0);
    // box_right = translate(box_right, [130.0, 0.0, 65.0]);
    let sphere_right = Shapes::sphere([190.0, 90.0, 190.0], 90.0, materials.get("glass"));

    let mut box_left = Shapes::box3d(
        [0.0, 0.0, 0.0],
        [165.0, 330.0, 165.0],
        materials.get("white"),
    );
    box_left = rotate_y(box_left, 15.0);
    box_left = translate(box_left, [265.0, 0.0, 295.0]);

    Scene::add_cornell_room(&mut scene, &materials, 555.0);
    scene.add(Arc::clone(&sphere_right));
    scene.add(Arc::clone(&box_left));
    scene
}

fn scene_cornell_smoke() -> Scene {
    let mut scene = Scene::new();
    let materials = Materials::new();

    let mut box_right_boundary = Shapes::box3d(
        [0.0, 0.0, 0.0],
        [165.0, 165.0, 165.0],
        materials.get("white"),
    );
    box_right_boundary = rotate_y(box_right_boundary, -18.0);
    box_right_boundary = translate(box_right_boundary, [130.0, 0.0, 65.0]);
    let box_right = Shapes::constant_medium(box_right_boundary, [1.0, 1.0, 1.0], 0.01);

    let mut box_left_boundary = Shapes::box3d(
        [0.0, 0.0, 0.0],
        [165.0, 330.0, 165.0],
        materials.get("white"),
    );
    box_left_boundary = rotate_y(box_left_boundary, 15.0);
    box_left_boundary = translate(box_left_boundary, [265.0, 0.0, 295.0]);
    let box_left = Shapes::constant_medium(box_left_boundary, [0.0, 0.0, 0.0], 0.01);

    Scene::add_cornell_room(&mut scene, &materials, 555.0);
    scene.add(box_right);
    scene.add(box_left);
    scene
}

fn camera_book2_final() -> CameraOptions {
    CameraOptions::new()
        .vfov(40.0)
        .position([478.0, 278.0, -600.0])
        .target([278.0, 278.0, 0.0])
}

fn scene_book2_final(with_haze: bool) -> Scene {
    let mut scene = Scene::new();
    let materials = Materials::new();

    // Floor boxes
    let ground = Materials::lambertian([0.48, 0.83, 0.53]);
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

            boxes.add(Shapes::box3d(
                [x0, y0, z0],
                [x1, y1, z1],
                Arc::clone(&ground),
            ));
        }
    }
    let boxes_bvh: Arc<dyn Hittable> = Arc::new(BvhNode::from_scene(boxes));
    scene.add(Arc::clone(&boxes_bvh));

    // Light
    let diffuse = Materials::diffuse_light([7.0, 7.0, 7.0]);
    let light = Shapes::quad(
        [123.0, 554.0, 147.0],
        [300.0, 0.0, 0.0],
        [0.0, 0.0, 265.0],
        Arc::clone(&diffuse),
    );
    scene.add(Arc::clone(&light));

    // Moving sphere
    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vector3::new(30.0, 0.0, 0.0);
    let sphere_mat = Materials::lambertian([0.7, 0.3, 0.1]);
    let sphere: Arc<dyn Hittable> = Arc::new(Sphere::moving(
        center1,
        center2,
        50.0,
        Arc::clone(&sphere_mat),
    ));
    scene.add(Arc::clone(&sphere));

    // Glass sphere
    let sphere = Shapes::sphere([260.0, 150.0, 45.0], 50.0, materials.get("glass"));
    scene.add(Arc::clone(&sphere));

    // Metal sphere
    let metal = Materials::metal([0.8, 0.8, 0.9], 1.0);
    let sphere = Shapes::sphere([0.0, 150.0, 145.0], 50.0, Arc::clone(&metal));
    scene.add(Arc::clone(&sphere));

    // Blue glass
    let boundary = Shapes::sphere([360.0, 150.0, 145.0], 70.0, materials.get("glass"));
    scene.add(Arc::clone(&boundary));
    let medium: Arc<dyn Hittable> = Arc::new(ConstantMedium::from_color(
        boundary,
        0.2,
        Color::new(0.2, 0.4, 0.9),
    ));
    scene.add(Arc::clone(&medium));

    // Haze
    if with_haze {
        let boundary = Shapes::sphere([0.0, 0.0, 0.0], 5000.0, materials.get("glass"));
        let medium: Arc<dyn Hittable> =
            Arc::new(ConstantMedium::from_color(boundary, 0.0001, Color::white()));
        scene.add(Arc::clone(&medium));
    }

    // Globe
    let globe: Arc<dyn Hittable> =
        Shapes::sphere([400.0, 200.0, 400.0], 100.0, materials.get("earth"));
    scene.add(Arc::clone(&globe));

    // Noisy ball
    let sphere = Shapes::sphere([220.0, 280.0, 300.0], 80.0, materials.get("marble"));
    scene.add(Arc::clone(&sphere));

    // Bubbly box
    let white = Materials::lambertian([0.73, 0.73, 0.73]);
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
