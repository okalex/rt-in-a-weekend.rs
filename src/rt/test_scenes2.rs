use crate::rt::{
    camera::CameraOptions,
    file::load_model,
    geometry::{
        primitive::Primitive,
        scene::{
            Instance,
            Scene,
            SceneBuilder,
        },
    },
    random::{
        rand,
        rand_range,
        rand_range_vector,
    },
    test_helpers::{
        cornell_room,
        materials,
        primitives,
    },
    types::{
        Float,
        Point,
        Vector,
    },
    util::degrees_to_radians,
};

pub fn get_scene(scene_idx: u32) -> (CameraOptions, Scene) {
    match scene_idx {
        1 => scene_spheres(),
        2 => scene_marbles(),
        3 => scene_quads(),
        4 => scene_triangles(),
        5 => scene_mesh(),
        _ => panic!(),
    }
}

// Test sphere rendering
pub fn scene_spheres() -> (CameraOptions, Scene) {
    let mut scene_builder = SceneBuilder::new();
    let materials = materials::defaults();
    let primitives = primitives::defaults();

    // Setup camera
    let camera_options = CameraOptions::new()
        .vfov(50.0)
        .position([0.0, 1.5, 4.0])
        .target([0.0, 0.5, 0.0])
        .defocus_angle(0.5)
        .focus_dist(3.4);

    // Add materials
    let checkered_id = scene_builder.add_material(materials.checkered);
    let blue_id = scene_builder.add_material(materials.blue);
    let glass_id = scene_builder.add_material(materials.glass);
    let air_id = scene_builder.add_material(materials.air);
    let gold_id = scene_builder.add_material(materials.gold);
    let diffuse_light_id = scene_builder.add_material(materials.diffuse_light);

    // Make primitives
    let sphere_prim = primitives::sphere([0.0, 0.0, 0.0], 0.5);

    // Add primitives
    let ground_id = scene_builder.add_primitive(primitives.ground);
    let sphere_id = scene_builder.add_primitive(sphere_prim);

    // Make instances
    let sphere_blue = Instance::new(sphere_id, blue_id).translate([0.0, 0.5, 0.0]);
    let sphere_glass = Instance::new(sphere_id, glass_id).translate([-1.0, 0.5, 0.0]);
    let sphere_air = Instance::new(sphere_id, air_id).scale_uniform(0.6).translate([-1.0, 0.5, 0.0]);
    let sphere_gold = Instance::new(sphere_id, gold_id).translate([1.0, 0.5, 0.0]);
    let sphere_light = Instance::new(sphere_id, diffuse_light_id)
        .scale_uniform(0.6)
        .translate([0.0, 2.5, 0.0]);

    // Add instances
    scene_builder.create_instance(ground_id, checkered_id);
    scene_builder.add_instance(sphere_blue);
    scene_builder.add_instance(sphere_glass);
    scene_builder.add_instance(sphere_air);
    scene_builder.add_instance(sphere_gold);
    scene_builder.add_instance(sphere_light);

    (camera_options, scene_builder.build())
}

// Test lots of spheres
fn scene_marbles() -> (CameraOptions, Scene) {
    let mut scene_builder = SceneBuilder::new();
    let materials = materials::defaults();
    let primitives = primitives::defaults();

    // Setup camera
    let camera_scale = 1.3;
    let camera_options = CameraOptions::new()
        .position([camera_scale * 13.0, camera_scale * 2.0, camera_scale * 3.0])
        .target([0.0, 0.0, 0.0])
        .defocus_angle(0.6)
        .focus_dist(camera_scale * 10.0);

    // Add materials
    let checkered_id = scene_builder.add_material(materials.checkered);
    let red_id = scene_builder.add_material(materials.red);
    let glass_id = scene_builder.add_material(materials.glass);
    let gold_id = scene_builder.add_material(materials.gold);

    // Make primitives
    let sphere_prim = primitives::sphere([0.0, 0.0, 0.0], 1.0);

    // Add primitives
    let ground_id = scene_builder.add_primitive(primitives.ground);
    let sphere_id = scene_builder.add_primitive(sphere_prim);

    // Make instances
    let sphere_glass = Instance::new(sphere_id, glass_id).translate([0.0, 1.0, 0.0]);
    let sphere_red = Instance::new(sphere_id, red_id).translate([-4.0, 1.0, 0.0]);
    let sphere_gold = Instance::new(sphere_id, gold_id).translate([4.0, 1.0, 0.0]);

    // Add instances
    scene_builder.create_instance(ground_id, checkered_id);
    scene_builder.add_instance(sphere_glass);
    scene_builder.add_instance(sphere_red);
    scene_builder.add_instance(sphere_gold);

    // Marbles
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand();
            let center = [a as Float + 0.9 * rand(), 0.2, b as Float + 0.9 * rand()];

            let center1 = Point::from(center);
            if (center1 - Point::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.4 {
                    // Diffuse
                    // let center2 = center1 + Vector::new(0.0, rand_range(0.0, 0.5), 0.0);
                    let mat = materials::rand_lambertian();
                    let mat_id = scene_builder.add_material(mat);
                    let sphere = Instance::new(sphere_id, mat_id).scale_uniform(0.2).translate(center1.to_array());
                    scene_builder.add_instance(sphere);
                } else if choose_mat < 0.7 {
                    // Metal
                    let mat = materials::rand_metal();
                    let mat_id = scene_builder.add_material(mat);
                    let sphere = Instance::new(sphere_id, mat_id).scale_uniform(0.2).translate(center);
                    scene_builder.add_instance(sphere);
                } else if choose_mat < 0.9 {
                    // Glass
                    let sphere = Instance::new(sphere_id, glass_id).scale_uniform(0.2).translate(center);
                    scene_builder.add_instance(sphere);
                } else {
                    // Lights
                    let light = materials::emissive(rand_range_vector(12.0, 24.0).to_array());
                    let diffuse_light_id = scene_builder.add_material(light);
                    let center = center1 + Vector::new(0.0, rand_range(2.8, 3.2), 0.0);
                    let sphere = Instance::new(sphere_id, diffuse_light_id)
                        .scale_uniform(0.2)
                        .translate(center.to_array());
                    scene_builder.add_instance(sphere);
                }
            }
        }
    }

    (camera_options, scene_builder.build())
}

// Test quad rendering
fn scene_quads() -> (CameraOptions, Scene) {
    let mut scene_builder = SceneBuilder::new();
    let camera_options = cornell_room::camera();

    cornell_room::add_to_scene(&mut scene_builder);

    (camera_options, scene_builder.build())
}

// Test triangle rendering
pub fn scene_triangles() -> (CameraOptions, Scene) {
    let mut scene_builder = SceneBuilder::new();
    let materials = materials::defaults();
    let primitives = primitives::defaults();

    // Setup camera
    let camera_options = CameraOptions::new()
        .vfov(50.0)
        .position([0.0, 1.5, 4.0])
        .target([0.0, 0.5, 0.0])
        .defocus_angle(0.5)
        .focus_dist(3.4);

    // Add materials
    let checkered_id = scene_builder.add_material(materials.checkered);
    let blue_id = scene_builder.add_material(materials.blue);
    let red_id = scene_builder.add_material(materials.red);
    let green_id = scene_builder.add_material(materials.green);
    let white_id = scene_builder.add_material(materials.white);

    // Make primitives
    let tri1_prim = primitives::triangle([0.0, 1.0, 0.0], [-1.0, 0.0, -1.0], [1.0, 0.0, -1.0]);
    let tri2_prim = primitives::triangle([0.0, 1.0, 0.0], [1.0, 0.0, -1.0], [1.0, 0.0, 1.0]);
    let tri3_prim = primitives::triangle([0.0, 1.0, 0.0], [1.0, 0.0, 1.0], [-1.0, 0.0, 1.0]);
    let tri4_prim = primitives::triangle([0.0, 1.0, 0.0], [-1.0, 0.0, 1.0], [-1.0, 0.0, -1.0]);

    // Add primitives
    let ground_id = scene_builder.add_primitive(primitives.ground);
    let tri1_id = scene_builder.add_primitive(tri1_prim);
    let tri2_id = scene_builder.add_primitive(tri2_prim);
    let tri3_id = scene_builder.add_primitive(tri3_prim);
    let tri4_id = scene_builder.add_primitive(tri4_prim);

    // Make instances
    let tri1 = Instance::new(tri1_id, blue_id);
    let tri2 = Instance::new(tri2_id, red_id);
    let tri3 = Instance::new(tri3_id, green_id);
    let tri4 = Instance::new(tri4_id, white_id);

    // Add instances
    scene_builder.create_instance(ground_id, checkered_id);
    scene_builder.add_instance(tri1);
    scene_builder.add_instance(tri2);
    scene_builder.add_instance(tri3);
    scene_builder.add_instance(tri4);

    (camera_options, scene_builder.build())
}

pub fn scene_mesh() -> (CameraOptions, Scene) {
    let mut scene_builder = SceneBuilder::new();
    let materials = materials::defaults();
    let primitives = primitives::defaults();

    // Setup camera
    let camera_options = CameraOptions::new()
        .vfov(50.0)
        .position([0.0, 1.0, 3.0])
        .target([0.0, 0.5, 0.0])
        .defocus_angle(0.5)
        .focus_dist(3.4);

    // Add materials
    let checkered_id = scene_builder.add_material(materials.checkered);
    let material_id = scene_builder.add_material(materials.gold);

    // Add ground
    let ground_id = scene_builder.add_primitive(primitives.ground);
    scene_builder.create_instance(ground_id, checkered_id);

    // Add meshes
    let model = match load_model("bunny.obj") {
        Ok(meshes) => meshes,
        _ => vec![],
    };
    for mesh in model {
        let triangle_count = mesh.triangle_count();
        let mesh_id = scene_builder.add_mesh(mesh);
        let primitive = Primitive::mesh(mesh_id, triangle_count);
        let primitive_id = scene_builder.add_primitive(primitive);
        let instance = Instance::new(primitive_id, material_id)
            .scale_uniform(10.0)
            .rotate_y(degrees_to_radians(20.0))
            .translate([0.0, -0.36, 0.0]);
        let _ = scene_builder.add_instance(instance);
    }

    (camera_options, scene_builder.build())
}
