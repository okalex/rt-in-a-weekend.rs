use crate::rt::{
    camera::CameraOptions,
    geometry::scene::{
        Instance,
        Scene,
        SceneBuilder,
    },
    test_helpers::{
        materials,
        primitives,
    },
};

pub fn scene1() -> (CameraOptions, Scene) {
    let mut scene_builder = SceneBuilder::new();
    let materials = materials::defaults();
    let primitives = primitives::defaults();

    // Add materials
    let checkered_id = scene_builder.add_material(materials.checkered);
    let blue_id = scene_builder.add_material(materials.blue);
    let glass_id = scene_builder.add_material(materials.glass);
    let air_id = scene_builder.add_material(materials.air);
    let gold_id = scene_builder.add_material(materials.gold);
    let diffuse_light_id = scene_builder.add_material(materials.diffuse_light);

    // Make primitives
    let sphere_prim = primitives::sphere([0.0, 0.0, 0.0], 0.5);
    let sphere_aabb = sphere_prim.aabb();

    // Add primitives
    let ground_id = scene_builder.add_primitive(primitives.ground);
    let sphere_id = scene_builder.add_primitive(sphere_prim);

    // Make instances
    let sphere_blue = Instance::new(sphere_id, blue_id, sphere_aabb).translate([0.0, 0.5, 0.0]);
    let sphere_glass = Instance::new(sphere_id, glass_id, sphere_aabb).translate([-1.0, 0.5, 0.0]);
    let sphere_air = Instance::new(sphere_id, air_id, sphere_aabb)
        .scale_uniform(0.6)
        .translate([-1.0, 0.5, 0.0]);
    let sphere_gold = Instance::new(sphere_id, gold_id, sphere_aabb).translate([1.0, 0.5, 0.0]);
    let sphere_light = Instance::new(sphere_id, diffuse_light_id, sphere_aabb)
        .scale_uniform(0.6)
        .translate([0.0, 2.5, 0.0]);

    // Add instances
    scene_builder.create_instance(ground_id, checkered_id);
    scene_builder.add_instance(sphere_blue);
    scene_builder.add_instance(sphere_glass);
    scene_builder.add_instance(sphere_air);
    scene_builder.add_instance(sphere_gold);
    scene_builder.add_instance(sphere_light);

    // Build scene
    let scene = scene_builder.build();

    let camera = CameraOptions::new()
        .vfov(50.0)
        .position([0.0, 1.5, 4.0])
        .target([0.0, 0.5, 0.0])
        .defocus_angle(0.5)
        .focus_dist(3.4);

    (camera, scene)
}
