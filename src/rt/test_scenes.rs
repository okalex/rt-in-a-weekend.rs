use std::sync::Arc;

use crate::rt::{
    camera::CameraOptions,
    color::Color,
    file::load_model_with_mat,
    materials::{
        dielectric::Dielectric, diffuse_light::DiffuseLight, isotropic::Isotropic,
        lambertian::Lambertian, material::Material, metal::Metal, pbr_material::PbrMaterial,
    },
    objects::{
        box3d::Box3d,
        bvh_node::BvhNode,
        constant_medium::ConstantMedium,
        hittable::Hittable,
        hittable_list::HittableList,
        quad::Quad,
        scene::Scene,
        sphere::Sphere,
        transformations::{rotate_y, translate},
        triangle::Triangle,
    },
    random::{rand, rand_range, rand_range_vector},
    textures::{
        checkered::Checkered, image_map::ImageMap, perlin_noise::PerlinNoise, texture::Texture,
    },
    types::{Float, Point, Uint, Vector},
};

pub fn get_camera_and_scene(scene_idx: Uint) -> (CameraOptions, Scene) {
    let (camera_options, scene) = match scene_idx {
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
    (camera_options, scene)
}

fn rand_arr3() -> [Float; 3] {
    [rand(), rand(), rand()]
}

struct Textures {}

impl Textures {
    fn checkers() -> Arc<Texture> {
        Arc::new(Texture::Checkered(Checkered::from_color_values(
            0.32,
            [0.2, 0.3, 0.1],
            [0.9, 0.9, 0.9],
        )))
    }
}

pub struct Materials {
    pub materials: Vec<Material>,
}

impl Materials {
    fn new() -> Self {
        Self {
            materials: vec![
                Self::lambertian([0.65, 0.05, 0.05]),
                Self::lambertian([0.73, 0.73, 0.73]),
                Self::lambertian([0.12, 0.45, 0.15]),
                Self::lambertian([0.1, 0.2, 0.5]),
                Self::lambertian([1.0, 0.5, 0.0]),
                Self::lambertian([0.2, 0.8, 0.8]),
                Self::diffuse_light([15.0, 15.0, 15.0]),
                Self::from_texture(Textures::checkers()),
                Self::dielectric(1.5),
                Self::dielectric(1.0 / 1.5),
                Self::metal([0.8, 0.85, 0.88], 0.0),
                Self::metal([0.8, 0.6, 0.2], 0.2),
                Self::image_map("assets/cube-diffuse.jpg", 1.0),
                Self::image_map("assets/rusty-metal.jpg", 1.0),
                Self::from_texture(Arc::new(Texture::Perlin(PerlinNoise::new(8.0)))),
                Self::image_map("assets/earthmap.jpg", 1.0),
                Self::pbr([0.8, 0.6, 0.2], 0.7),
                Self::lambertian([0.5, 0.5, 0.5]),
            ],
        }
    }

    fn add(&mut self, mat: Material) -> usize {
        let new_idx = self.materials.len();
        self.materials.push(mat);
        new_idx
    }

    fn get(&self, name: &str) -> usize {
        match name {
            "red" => 0,
            "white" => 1,
            "green" => 2,
            "blue" => 3,
            "orange" => 4,
            "teal" => 5,
            "diffuse_light" => 6,
            "checkered" => 7,
            "glass" => 8,
            "air" => 9,
            "mirror" => 10,
            "gold" => 11,
            "stone" => 12,
            "rusty_metal" => 13,
            "marble" => 14,
            "earth" => 15,
            "pbr" => 16,
            _ => 17,
        }
    }

    fn lambertian(color: [Float; 3]) -> Material {
        Material::Lambertian(Lambertian::from(color))
    }

    fn from_texture(texture: Arc<Texture>) -> Material {
        Material::Lambertian(Lambertian::new(texture))
    }

    fn rand_lambertian() -> Material {
        let albedo = rand_arr3();
        Self::lambertian(albedo)
    }

    fn dielectric(refraction_idx: Float) -> Material {
        Material::Dielectric(Dielectric::new(refraction_idx))
    }

    fn metal(color: [Float; 3], fuzz: Float) -> Material {
        Material::Metal(Metal::new(color, fuzz))
    }

    fn rand_metal() -> Material {
        let albedo = rand_arr3();
        let fuzz = rand_range(0.0, 0.5);
        Self::metal(albedo, fuzz)
    }

    fn diffuse_light(color: [Float; 3]) -> Material {
        Material::DiffuseLight(DiffuseLight::from(color))
    }

    fn image_map(file_name: &str, scale_factor: Float) -> Material {
        let tex = Arc::new(Texture::ImageMap(ImageMap::new(file_name, scale_factor)));
        Self::from_texture(tex)
    }

    fn pbr(albedo: [Float; 3], metallicity: Float) -> Material {
        let color = Color::from(albedo);
        Material::PbrMaterial(PbrMaterial::new(color, metallicity))
    }

    fn isotropic(albedo: [Float; 3]) -> Material {
        Material::Isotropic(Isotropic::from(albedo))
    }
}

pub struct Shapes {}

impl Shapes {
    pub fn sphere(center: [Float; 3], radius: Float, mat_idx: usize) -> Hittable {
        Hittable::Sphere(Sphere::stationary(Point::from(center), radius, mat_idx))
    }

    pub fn quad(q: [Float; 3], u: [Float; 3], v: [Float; 3], mat_idx: usize) -> Hittable {
        Hittable::Quad(Quad::from_arr(q, u, v, mat_idx))
    }

    pub fn triangle(a: [Float; 3], b: [Float; 3], c: [Float; 3], mat_idx: usize) -> Hittable {
        Hittable::Triangle(Triangle::new(a, b, c, mat_idx))
    }

    pub fn box3d(a: [Float; 3], b: [Float; 3], mat_idx: usize) -> Hittable {
        Hittable::HittableList(Box3d::new(Vector::from(a), Vector::from(b), mat_idx))
    }

    pub fn constant_medium(
        materials: &mut Materials,
        boundary: Arc<Hittable>,
        color: [Float; 3],
        density: Float,
    ) -> Hittable {
        let mat = Materials::isotropic(color);
        let mat_idx = materials.add(mat);
        Hittable::ConstantMedium(ConstantMedium::new(boundary, density, mat_idx))
    }

    pub fn ground_sphere(mat_idx: usize) -> Hittable {
        Shapes::sphere([1.0, -100.0, -1.0], 100.0, mat_idx)
    }

    pub fn checkered_ground(materials: &Materials) -> Hittable {
        Shapes::ground_sphere(materials.get("checkered"))
    }
}

impl HittableList {
    fn cornell_room(materials: &Materials, width: Float) -> (Hittable, Hittable) {
        let mut objects = HittableList::new();
        let mut lights = HittableList::new();

        let light = Arc::new(Shapes::quad(
            [0.6 * width, width - 0.1, 0.6 * width],
            [-0.2 * width, 0.0, 0.0],
            [0.0, 0.0, -0.2 * width],
            materials.get("diffuse_light"),
        ));
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

        objects.add(left);
        objects.add(right);
        objects.add(floor);
        objects.add(ceiling);
        objects.add(back);
        objects.add_arc(light.clone());
        lights.add_arc(light.clone());

        (
            Hittable::HittableList(objects),
            Hittable::HittableList(lights),
        )
    }
}

fn camera_a() -> CameraOptions {
    CameraOptions::new()
        .vfov(50.0)
        .position([0.0, 0.5, 4.0])
        .target([0.0, 0.5, 0.0])
        .defocus_angle(0.5)
        .focus_dist(3.4)
}

fn scene_a() -> Scene {
    let mut scene = HittableList::new();
    let materials = Materials::new();

    let ground = Shapes::ground_sphere(materials.get("green"));
    let sphere2 = Shapes::sphere([0.0, 0.5, 0.0], 0.5, materials.get("blue"));
    let sphere3 = Shapes::sphere([-1.0, 0.5, 0.0], 0.5, materials.get("glass"));
    let sphere4 = Shapes::sphere([-1.0, 0.5, 0.0], 0.3, materials.get("air"));
    let sphere5 = Shapes::sphere([1.0, 0.5, 0.0], 0.5, materials.get("gold"));

    scene.add(ground);
    scene.add(sphere2);
    scene.add(sphere3);
    scene.add(sphere4);
    scene.add(sphere5);

    Scene::no_lights(scene, materials.materials)
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
    let mut scene = HittableList::new();
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

    Scene::no_lights(scene, materials.materials)
}

fn scene_obj(scale: Float) -> Scene {
    let mut scene = HittableList::new();
    let mut lights = HittableList::new();
    let materials = Materials::new();

    let (room, room_lights) = HittableList::cornell_room(&materials, scale);
    scene.add(room);
    lights.add(room_lights);

    let objs = match load_model_with_mat("cube.obj", materials.get("rusty_metal")) {
        Ok(os) => os,
        _ => panic!(),
    };
    for obj in objs {
        let moved = translate(rotate_y(obj, 205.0), [10.0, 1.0, 5.0]);
        scene.add(moved);
    }

    Scene::new(scene, materials.materials, lights)
}

fn scene_pbr(scale: Float) -> Scene {
    let mut scene = HittableList::new();
    let mut lights = HittableList::new();
    let materials = Materials::new();

    let (room, room_lights) = HittableList::cornell_room(&materials, scale);
    scene.add(room);
    lights.add(room_lights);

    // let purple = Materials::diffuse_light([8.0, 0.0, 10.0]);
    // let purple_light = Shapes::sphere([530.0, 530.0, 20.0], 10.0, purple);
    // scene.add(Arc::clone(&purple_light));
    // lights.add(Arc::clone(&purple_light));

    // let teal = Materials::diffuse_light([0.0, 8.0, 10.0]);
    // let teal_light = Shapes::sphere([20.0, 530.0, 20.0], 10.0, teal);
    // scene.add(Arc::clone(&teal_light));
    // lights.add(Arc::clone(&teal_light));

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

    let sphere = Shapes::sphere([420.0, 100.0, 180.0], 100.0, materials.get("pbr"));
    scene.add(sphere);

    Scene::new(scene, materials.materials, lights)
}

fn camera_b() -> CameraOptions {
    CameraOptions::new()
        .position([13.0, 2.0, 3.0])
        .target([0.0, 0.0, 0.0])
        .defocus_angle(0.6)
        .focus_dist(10.0)
}

fn scene_b() -> Scene {
    let mut scene = HittableList::new();
    let mut materials = Materials::new();

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
            let center = [a as Float + 0.9 * rand(), 0.2, b as Float + 0.9 * rand()];

            let center1 = Point::from(center);
            if (center1 - Point::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.4 {
                    // Diffuse
                    let center2 = center1 + Vector::new(0.0, rand_range(0.0, 0.5), 0.0);
                    let mat = Materials::rand_lambertian();
                    let mat_idx = materials.add(mat);
                    let sphere = Hittable::Sphere(Sphere::moving(center1, center2, 0.2, mat_idx));
                    scene.add(sphere);
                } else if choose_mat < 0.6 {
                    // Metal
                    let mat = Materials::rand_metal();
                    let mat_idx = materials.add(mat);
                    let sphere = Shapes::sphere(center, 0.2, mat_idx);
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

    Scene::no_lights(scene, materials.materials)
}

fn camera_c() -> CameraOptions {
    CameraOptions::new()
        .position([13.0, 2.0, 3.0])
        .target([0.0, 0.0, 0.0])
}

fn scene_c() -> Scene {
    let mut scene = HittableList::new();
    let materials = Materials::new();

    let sphere1 = Shapes::sphere([0.0, -10.0, 0.0], 10.0, materials.get("checkered"));
    let sphere2 = Shapes::sphere([0.0, 10.0, 0.0], 10.0, materials.get("checkered"));

    scene.add(sphere1);
    scene.add(sphere2);

    Scene::no_lights(scene, materials.materials)
}

fn camera_d() -> CameraOptions {
    CameraOptions::new()
        .position([0.0, 4.0, 16.0])
        .target([0.0, 1.5, 0.0])
}

fn scene_d() -> Scene {
    let mut scene = HittableList::new();
    let materials = Materials::new();

    let ground = Shapes::checkered_ground(&materials);
    let globe = Shapes::sphere([0.0, 2.0, 0.0], 2.0, materials.get("earth"));

    scene.add(globe);
    scene.add(ground);

    Scene::no_lights(scene, materials.materials)
}

fn camera_e() -> CameraOptions {
    CameraOptions::new()
        .position([13.0, 2.0, 3.0])
        .target([0.0, 0.0, 0.0])
}

fn scene_e() -> Scene {
    let mut scene = HittableList::new();
    let materials = Materials::new();

    let ground = Shapes::checkered_ground(&materials);
    let sphere2 = Shapes::sphere([0.0, 2.0, 0.0], 2.0, materials.get("marble"));

    scene.add(ground);
    scene.add(sphere2);

    Scene::no_lights(scene, materials.materials)
}

fn camera_f() -> CameraOptions {
    CameraOptions::new()
        .vfov(80.0)
        .position([0.0, 0.0, 9.0])
        .target([0.0, 0.0, 0.0])
}

fn scene_f() -> Scene {
    let mut scene = HittableList::new();
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

    Scene::no_lights(scene, materials.materials)
}

fn camera_g() -> CameraOptions {
    CameraOptions::new()
        .position([26.0, 3.0, 6.0])
        .target([0.0, 2.0, 0.0])
}

fn scene_g() -> Scene {
    let mut scene = HittableList::new();
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

    Scene::no_lights(scene, materials.materials)
}

fn camera_cornell(room_width: Float) -> CameraOptions {
    CameraOptions::new()
        .vfov(40.0)
        .position([room_width / 2.0, room_width / 2.0, -1.44 * room_width])
        .target([room_width / 2.0, room_width / 2.0, 0.0])
}

fn scene_cornell() -> Scene {
    let mut scene = HittableList::new();
    let mut lights = HittableList::new();
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

    let (room, room_lights) = HittableList::cornell_room(&materials, 555.0);
    scene.add(room);
    lights.add(room_lights);

    scene.add(sphere_right);
    scene.add(box_left);

    Scene::new(scene, materials.materials, lights)
}

fn scene_cornell_smoke() -> Scene {
    let mut scene = HittableList::new();
    let mut lights = HittableList::new();
    let mut materials = Materials::new();

    let mut box_right_boundary = Shapes::box3d(
        [0.0, 0.0, 0.0],
        [165.0, 165.0, 165.0],
        materials.get("white"),
    );
    box_right_boundary = rotate_y(box_right_boundary, -18.0);
    box_right_boundary = translate(box_right_boundary, [130.0, 0.0, 65.0]);
    let box_right = Shapes::constant_medium(
        &mut materials,
        Arc::new(box_right_boundary),
        [1.0, 1.0, 1.0],
        0.01,
    );

    let mut box_left_boundary = Shapes::box3d(
        [0.0, 0.0, 0.0],
        [165.0, 330.0, 165.0],
        materials.get("white"),
    );
    box_left_boundary = rotate_y(box_left_boundary, 15.0);
    box_left_boundary = translate(box_left_boundary, [265.0, 0.0, 295.0]);
    let box_left = Shapes::constant_medium(
        &mut materials,
        Arc::new(box_left_boundary),
        [0.0, 0.0, 0.0],
        0.01,
    );

    let (room, room_lights) = HittableList::cornell_room(&materials, 555.0);
    scene.add(room);
    lights.add(room_lights);

    scene.add(box_right);
    scene.add(box_left);

    Scene::new(scene, materials.materials, lights)
}

fn camera_book2_final() -> CameraOptions {
    CameraOptions::new()
        .vfov(40.0)
        .position([478.0, 278.0, -600.0])
        .target([278.0, 278.0, 0.0])
}

fn scene_book2_final(with_haze: bool) -> Scene {
    let mut scene = HittableList::new();
    let mut materials = Materials::new();
    let mut lights = HittableList::new();

    // Floor boxes
    let ground_idx = materials.add(Materials::lambertian([0.48, 0.83, 0.53]));
    let mut boxes = HittableList::new();
    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let fi = i as Float;
            let fj = j as Float;

            let w = 100.0;

            let x0 = -1000.0 + fi * w;
            let y0 = 0.0;
            let z0 = -1000.0 + fj * w;

            let x1 = x0 + w;
            let y1 = rand_range(1.0, 101.0);
            let z1 = z0 + w;

            boxes.add(Shapes::box3d([x0, y0, z0], [x1, y1, z1], ground_idx));
        }
    }
    let boxes_bvh = Hittable::BvhNode(BvhNode::from(boxes));
    scene.add(boxes_bvh);

    // Light
    let diffuse_idx = materials.add(Materials::diffuse_light([7.0, 7.0, 7.0]));
    let light = Arc::new(Shapes::quad(
        [123.0, 554.0, 147.0],
        [300.0, 0.0, 0.0],
        [0.0, 0.0, 265.0],
        diffuse_idx,
    ));
    scene.add_arc(Arc::clone(&light));
    lights.add_arc(Arc::clone(&light));

    // Moving sphere
    let center1 = Point::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vector::new(30.0, 0.0, 0.0);
    let sphere_mat_idx = materials.add(Materials::lambertian([0.7, 0.3, 0.1]));
    let sphere = Hittable::Sphere(Sphere::moving(center1, center2, 50.0, sphere_mat_idx));
    scene.add(sphere);

    // Glass sphere
    let sphere = Shapes::sphere([260.0, 150.0, 45.0], 50.0, materials.get("glass"));
    scene.add(sphere);

    // Metal sphere
    let metal_idx = materials.add(Materials::metal([0.8, 0.8, 0.9], 1.0));
    let sphere = Shapes::sphere([0.0, 150.0, 145.0], 50.0, metal_idx);
    scene.add(sphere);

    // Blue glass
    let boundary = Shapes::sphere([360.0, 150.0, 145.0], 70.0, materials.get("glass"));
    let boundary_arc = Arc::new(boundary);
    scene.add_arc(Arc::clone(&boundary_arc));
    let medium = Shapes::constant_medium(
        &mut materials,
        Arc::clone(&boundary_arc),
        [0.2, 0.4, 0.9],
        0.2,
    );
    scene.add(medium);

    // Haze
    if with_haze {
        let boundary = Shapes::sphere([0.0, 0.0, 0.0], 5000.0, materials.get("glass"));
        let medium =
            Shapes::constant_medium(&mut materials, Arc::new(boundary), [1.0, 1.0, 1.0], 0.0001);
        scene.add(medium);
    }

    // Globe
    let globe = Shapes::sphere([400.0, 200.0, 400.0], 100.0, materials.get("earth"));
    scene.add(globe);

    // Noisy ball
    let sphere = Shapes::sphere([220.0, 280.0, 300.0], 80.0, materials.get("marble"));
    scene.add(sphere);

    // Bubbly box
    let mut boxes2 = HittableList::new();
    for _ in 0..1000 {
        let sphere = Hittable::Sphere(Sphere::stationary(
            Point::from(rand_range_vector(0.0, 165.0)),
            10.0,
            materials.get("white"),
        ));
        boxes2.add(sphere);
    }
    let mut boxes2_hittable = Hittable::BvhNode(BvhNode::from(boxes2));
    boxes2_hittable = rotate_y(boxes2_hittable, 15.0);
    boxes2_hittable = translate(boxes2_hittable, [-100.0, 270.0, 395.0]);
    scene.add(boxes2_hittable);

    Scene::new(scene, materials.materials, lights)
}
