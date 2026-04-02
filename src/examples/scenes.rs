use crate::rt::{camera::CameraOptions, geometry::scene::Scene};

pub fn get_scene(scene_idx: u32) -> Scene {
    match scene_idx {
        1 => spheres::scene(),
        2 => marbles::scene(),
        3 => cornell::scene(),
        4 => cornell_smoke::scene(),
        5 => triangles::scene(),
        6 => mesh::scene(),
        7 => book2::scene(),
        8 => pbr::scene(),
        _ => panic!(),
    }
}

pub fn get_camera_options(scene_idx: u32) -> CameraOptions {
    match scene_idx {
        1 => spheres::camera(),
        2 => marbles::camera(),
        3 => cornell::camera(),
        4 => cornell_smoke::camera(),
        5 => triangles::camera(),
        6 => mesh::camera(),
        7 => book2::camera(),
        8 => pbr::camera(),
        _ => panic!(),
    }
}

// Test sphere rendering
mod spheres {
    use crate::{
        examples::helpers::{materials, primitives},
        rt::{
            camera::CameraOptions,
            geometry::scene::{Instance, Scene, SceneBuilder},
        },
    };

    pub fn camera() -> CameraOptions {
        CameraOptions::new()
            .vfov(50.0)
            .position([0.0, 1.5, 2.0])
            .target([0.0, 0.5, 0.0])
            .defocus_angle(0.5)
            .focus_dist(3.4)
    }

    pub fn scene() -> Scene {
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

        // Add ground
        let ground_id = scene_builder.add_primitive(primitives.ground);
        scene_builder.create_instance(ground_id, checkered_id);

        // Make primitives
        let sphere_prim = primitives::sphere([0.0, 0.0, 0.0], 0.5);

        // Add primitives
        let sphere_id = scene_builder.add_primitive(sphere_prim);

        // Make instances
        let sphere_blue = Instance::new(sphere_id, blue_id).translate([0.0, 0.5, 0.0]);
        let sphere_glass = Instance::new(sphere_id, glass_id).translate([-1.1, 0.5, 0.0]);
        let sphere_air = Instance::new(sphere_id, air_id)
            .scale_uniform(0.6)
            .translate([-1.1, 0.5, 0.0]);
        let sphere_gold = Instance::new(sphere_id, gold_id).translate([1.1, 0.5, 0.0]);
        let sphere_light = Instance::new(sphere_id, diffuse_light_id)
            .scale_uniform(0.6)
            .translate([0.0, 2.5, 0.0]);

        // Add instances
        scene_builder.add_instance(sphere_blue);
        scene_builder.add_instance(sphere_glass);
        scene_builder.add_instance(sphere_air);
        scene_builder.add_instance(sphere_gold);
        let light_id = scene_builder.add_instance(sphere_light);
        scene_builder.add_light(light_id);

        scene_builder.build()
    }
}

// Test lots of spheres
mod marbles {
    use crate::{
        examples::helpers::{materials, primitives},
        rt::{
            camera::CameraOptions,
            geometry::scene::{Instance, Scene, SceneBuilder},
        },
        util::{
            random::{rand, rand_range, rand_range_vector},
            types::{Float, Point, Vector},
        },
    };

    pub fn camera() -> CameraOptions {
        let camera_scale = 1.3;
        CameraOptions::new()
            .position([camera_scale * 13.0, camera_scale * 2.0, camera_scale * 3.0])
            .target([0.0, 0.0, 0.0])
            .defocus_angle(0.6)
            .focus_dist(camera_scale * 10.0)
    }

    pub fn scene() -> Scene {
        let mut scene_builder = SceneBuilder::new();
        let materials = materials::defaults();
        let primitives = primitives::defaults();

        // Setup camera

        // Add materials
        let checkered_id = scene_builder.add_material(materials.checkered);
        let red_id = scene_builder.add_material(materials.red);
        let glass_id = scene_builder.add_material(materials.glass);
        let gold_id = scene_builder.add_material(materials.gold);

        // Add ground
        let ground_id = scene_builder.add_primitive(primitives.ground);
        scene_builder.create_instance(ground_id, checkered_id);

        // Make primitives
        let sphere_prim = primitives::sphere([0.0, 0.0, 0.0], 1.0);

        // Add primitives
        let sphere_id = scene_builder.add_primitive(sphere_prim);

        // Make instances
        let sphere_glass = Instance::new(sphere_id, glass_id).translate([0.0, 1.0, 0.0]);
        let sphere_red = Instance::new(sphere_id, red_id).translate([-4.0, 1.0, 0.0]);
        let sphere_gold = Instance::new(sphere_id, gold_id).translate([4.0, 1.0, 0.0]);

        // Add instances
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
                        let sphere = Instance::new(sphere_id, mat_id)
                            .scale_uniform(0.2)
                            .translate(center1.to_array());
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
                        let instance_id = scene_builder.add_instance(sphere);
                        let _ = scene_builder.add_light(instance_id);
                    }
                }
            }
        }

        scene_builder.build()
    }
}

// Test quad and mesh rendering
mod cornell {
    use crate::{
        examples::helpers::{cornell_room, materials, meshes},
        rt::{
            camera::CameraOptions,
            geometry::{
                primitive::Primitive,
                scene::{Instance, Scene, SceneBuilder},
            },
        },
        util::trig::degrees_to_radians,
    };

    pub fn camera() -> CameraOptions {
        cornell_room::camera()
    }

    pub fn scene() -> Scene {
        let mut scene_builder = SceneBuilder::new();
        let materials = materials::defaults();

        cornell_room::add_to_scene(&mut scene_builder);

        let white_id = scene_builder.add_material(materials.white);

        {
            // Right box
            let mesh = meshes::box3d(300.0, 300.0, 300.0);
            let mesh_id = scene_builder.add_mesh(mesh);
            let primitive_id = scene_builder.add_primitive(Primitive::mesh(mesh_id));
            let _ = scene_builder.add_instance(
                Instance::new(primitive_id, white_id)
                    .rotate_y(degrees_to_radians(-18.0))
                    .translate([236.0, 0.0, 118.0]),
            );
        }

        {
            // Left box
            let mesh = meshes::box3d(300.0, 600.0, 300.0);
            let mesh_id = scene_builder.add_mesh(mesh);
            let primitive_id = scene_builder.add_primitive(Primitive::mesh(mesh_id));
            let _ = scene_builder.add_instance(
                Instance::new(primitive_id, white_id)
                    .rotate_y(degrees_to_radians(15.0))
                    .translate([482.0, 0.0, 536.0]),
            );
        }

        scene_builder.build()
    }
}

// Test constant medium rendering
mod cornell_smoke {
    use crate::{
        examples::helpers::{cornell_room, materials, meshes, primitives},
        rt::{
            camera::CameraOptions,
            geometry::{
                primitive::Primitive,
                scene::{Instance, Scene, SceneBuilder},
            },
        },
        util::trig::degrees_to_radians,
    };

    pub fn camera() -> CameraOptions {
        cornell_room::camera()
    }

    pub fn scene() -> Scene {
        let mut scene_builder = SceneBuilder::new();

        cornell_room::add_to_scene(&mut scene_builder);

        let light_smoke_id = scene_builder.add_material(materials::isotropic([0.9, 0.9, 0.9]));
        let dark_smoke_id = scene_builder.add_material(materials::isotropic([0.2, 0.2, 0.2]));

        {
            // Right box
            let mesh = meshes::box3d(300.0, 300.0, 300.0);
            let mesh_id = scene_builder.add_mesh(mesh);
            let boundary_id = scene_builder.add_primitive(Primitive::mesh(mesh_id));
            let primitive_id = scene_builder.add_primitive(primitives::medium(boundary_id, 0.006));
            let _ = scene_builder.add_instance(
                Instance::new(primitive_id, light_smoke_id)
                    .rotate_y(degrees_to_radians(-18.0))
                    .translate([236.0, 0.0, 118.0]),
            );
        }

        {
            // Left box
            let mesh = meshes::box3d(300.0, 600.0, 300.0);
            let mesh_id = scene_builder.add_mesh(mesh);
            let boundary_id = scene_builder.add_primitive(Primitive::mesh(mesh_id));
            let primitive_id = scene_builder.add_primitive(primitives::medium(boundary_id, 0.003));
            let _ = scene_builder.add_instance(
                Instance::new(primitive_id, dark_smoke_id)
                    .rotate_y(degrees_to_radians(15.0))
                    .translate([482.0, 0.0, 536.0]),
            );
        }

        scene_builder.build()
    }
}

// Test triangle rendering
mod triangles {
    use crate::{
        examples::helpers::{materials, primitives},
        rt::{
            camera::CameraOptions,
            geometry::scene::{Instance, Scene, SceneBuilder},
        },
    };

    pub fn camera() -> CameraOptions {
        CameraOptions::new()
            .vfov(50.0)
            .position([0.0, 1.5, 4.0])
            .target([0.0, 0.5, 0.0])
            .defocus_angle(0.5)
            .focus_dist(3.4)
    }

    pub fn scene() -> Scene {
        let mut scene_builder = SceneBuilder::new();
        let materials = materials::defaults();
        let primitives = primitives::defaults();

        // Add materials
        let checkered_id = scene_builder.add_material(materials.checkered);
        let blue_id = scene_builder.add_material(materials.blue);
        let red_id = scene_builder.add_material(materials.red);
        let green_id = scene_builder.add_material(materials.green);
        let white_id = scene_builder.add_material(materials.white);

        // Add ground
        let ground_id = scene_builder.add_primitive(primitives.ground);
        scene_builder.create_instance(ground_id, checkered_id);

        // Make primitives
        let tri1_prim = primitives::triangle([0.0, 1.0, 0.0], [-1.0, 0.0, -1.0], [1.0, 0.0, -1.0]);
        let tri2_prim = primitives::triangle([0.0, 1.0, 0.0], [1.0, 0.0, -1.0], [1.0, 0.0, 1.0]);
        let tri3_prim = primitives::triangle([0.0, 1.0, 0.0], [1.0, 0.0, 1.0], [-1.0, 0.0, 1.0]);
        let tri4_prim = primitives::triangle([0.0, 1.0, 0.0], [-1.0, 0.0, 1.0], [-1.0, 0.0, -1.0]);

        // Add primitives
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
        scene_builder.add_instance(tri1);
        scene_builder.add_instance(tri2);
        scene_builder.add_instance(tri3);
        scene_builder.add_instance(tri4);

        scene_builder.build()
    }
}

mod mesh {
    use crate::{
        examples::helpers::{materials, primitives},
        rt::{
            camera::CameraOptions,
            geometry::{
                primitive::Primitive,
                scene::{Instance, Scene, SceneBuilder},
            },
        },
        util::{file::load_model, random::rand_range, trig::degrees_to_radians},
    };

    pub fn camera() -> CameraOptions {
        CameraOptions::new()
            .vfov(50.0)
            .position([0.0, 1.0, 3.0])
            .target([0.0, 0.5, 0.0])
            .defocus_angle(0.1)
            .focus_dist(3.4)
    }

    pub fn scene() -> Scene {
        let mut scene_builder = SceneBuilder::new();
        let materials = materials::defaults();
        let primitives = primitives::defaults();

        // Add ground
        let checkered_id = scene_builder.add_material(materials.checkered);
        let ground_id = scene_builder.add_primitive(primitives.ground);
        scene_builder.create_instance(ground_id, checkered_id);

        // Add lights
        let diffuse_light_id = scene_builder.add_material(materials.diffuse_light);
        let sphere_prim = primitives::sphere([0.0, 0.0, 0.0], 0.2);
        let sphere_id = scene_builder.add_primitive(sphere_prim);
        for _ in -9..=9 {
            let light = Instance::new(sphere_id, diffuse_light_id).translate([
                rand_range(-3.0, 3.0),
                3.0,
                rand_range(-3.0, 3.0),
            ]);
            let instance_id = scene_builder.add_instance(light);
            scene_builder.add_light(instance_id);
        }

        // Add meshes
        let mesh_mat_id = scene_builder.add_material(materials::checkered(0.05, [0.8, 0.3, 0.2], [0.9, 0.9, 0.9]));
        let model = load_model("teapot.obj").unwrap();
        for mesh in model {
            let mesh_id = scene_builder.add_mesh(mesh);
            let primitive = Primitive::mesh(mesh_id);
            let primitive_id = scene_builder.add_primitive(primitive);
            let instance = Instance::new(primitive_id, mesh_mat_id)
                .scale_uniform(0.3)
                .rotate_y(degrees_to_radians(30.0))
                .translate([-0.50, -0.0, 0.0]);
            let _ = scene_builder.add_instance(instance);
        }

        scene_builder.build()
    }
}

mod book2 {
    use crate::{
        examples::helpers::{materials, meshes, primitives},
        rt::{
            camera::CameraOptions,
            geometry::{
                primitive::Primitive,
                scene::{Instance, Scene, SceneBuilder},
            },
        },
        util::{
            random::{rand_range, rand_range_vector},
            types::{Float, Vector},
        },
    };

    pub fn camera() -> CameraOptions {
        CameraOptions::new()
            .vfov(40.0)
            .position([478.0, 278.0, -600.0])
            .target([278.0, 278.0, 0.0])
    }

    pub fn scene() -> Scene {
        let mut scene_builder = SceneBuilder::new();
        let materials = materials::defaults();

        // Materials
        let mat_light = scene_builder.add_material(materials::emissive([7.0, 7.0, 7.0]));
        let mat_floor = scene_builder.add_material(materials::lambertian([0.48, 0.83, 0.53]));
        let mat_orange = scene_builder.add_material(materials::lambertian([0.7, 0.3, 0.1]));
        let mat_glass = scene_builder.add_material(materials.glass);
        let mat_metal = scene_builder.add_material(materials::metal([0.8, 0.8, 0.9], 1.0));
        let mat_white = scene_builder.add_material(materials.white);

        // Ceiling light
        {
            let prim_id = scene_builder.add_primitive(primitives::quad(
                [123.0, 554.0, 147.0],
                [300.0, 0.0, 0.0],
                [0.0, 0.0, 265.0],
            ));
            let instance_id = scene_builder.add_instance(Instance::new(prim_id, mat_light));
            let _ = scene_builder.add_light(instance_id);
        }

        // Floor
        {
            let mesh = meshes::box3d(100.0, 100.0, 100.0);
            let mesh_id = scene_builder.add_mesh(mesh);
            let prim_id = scene_builder.add_primitive(Primitive::mesh(mesh_id));

            let boxes_per_side = 20;
            for i in 0..boxes_per_side {
                for j in 0..boxes_per_side {
                    let w = 100.0;
                    let x = -1000.0 + w * i as Float;
                    let y = rand_range(-99.0, 1.0);
                    let z = -1000.0 + w * j as Float;
                    let _ = scene_builder.add_instance(Instance::new(prim_id, mat_floor).translate([x, y, z]));
                }
            }
        }

        // Orange sphere
        {
            let prim_id = scene_builder.add_primitive(primitives::sphere([400.0, 400.0, 200.0], 50.0));
            let _ = scene_builder.add_instance(Instance::new(prim_id, mat_orange));
        }

        // Glass sphere
        {
            let prim_id = scene_builder.add_primitive(primitives::sphere([260.0, 150.0, 45.0], 50.0));
            let _ = scene_builder.add_instance(Instance::new(prim_id, mat_glass));
        }

        // Metal sphere
        {
            let prim_id = scene_builder.add_primitive(primitives::sphere([0.0, 150.0, 145.0], 50.0));
            let _ = scene_builder.add_instance(Instance::new(prim_id, mat_metal));
        }

        // Blue glass sphere
        {
            let mat_inner = scene_builder.add_material(materials::isotropic([0.2, 0.4, 0.9]));
            let boundary_id = scene_builder.add_primitive(primitives::sphere([360.0, 150.0, 145.0], 70.0));
            let _ = scene_builder.add_instance(Instance::new(boundary_id, mat_glass));
            let inner_id = scene_builder.add_primitive(primitives::medium(boundary_id, 0.2));
            let _ = scene_builder.add_instance(Instance::new(inner_id, mat_inner));
        }

        // Globe
        // let globe = Shapes::sphere([400.0, 200.0, 400.0], 100.0, materials.get("earth"));
        // scene.add(globe);

        // Perlin sphere
        // let sphere = Shapes::sphere([220.0, 280.0, 300.0], 80.0, materials.get("marble"));
        // scene.add(sphere);

        // Bubbles
        {
            let prim_id = scene_builder.add_primitive(primitives::sphere([0.0, 0.0, 0.0], 10.0));
            for _ in 0..1000 {
                let translation = Vector::new(-100.0, 270.0, 395.0) + rand_range_vector(0.0, 165.0);
                let _ = scene_builder.add_instance(Instance::new(prim_id, mat_white).translate(translation.to_array()));
            }
        }

        // Haze
        {
            let mat_haze = scene_builder.add_material(materials::isotropic([1.0, 1.0, 1.0]));
            let boundary_id = scene_builder.add_primitive(primitives::sphere([0.0, 0.0, 0.0], 5000.0));
            let medium = scene_builder.add_primitive(primitives::medium(boundary_id, 0.0001));
            let _ = scene_builder.add_instance(Instance::new(medium, mat_haze));
        }

        scene_builder.build()
    }
}

// Test PBR material rendering
mod pbr {
    use crate::{
        examples::helpers::{materials, primitives},
        rt::{
            camera::CameraOptions,
            geometry::scene::{Instance, Scene, SceneBuilder},
            materials::pbr_material::PbrMaterialProperties,
        },
        util::types::Float,
    };

    pub fn camera() -> CameraOptions {
        CameraOptions::new()
            .vfov(20.0)
            .position([0.0, 1.5, 10.0])
            .target([0.0, 0.5, 0.0])
            .defocus_angle(0.0)
            .focus_dist(3.4)
    }

    pub fn scene() -> Scene {
        let mut scene_builder = SceneBuilder::new();
        let materials = materials::defaults();
        let primitives = primitives::defaults();

        // Add primitives
        let sphere_id = scene_builder.add_primitive(primitives::sphere([0.0, 0.0, 0.0], 0.5));

        // Add ground
        let checkered_id = scene_builder.add_material(materials.checkered);
        let ground_id = scene_builder.add_primitive(primitives.ground);
        scene_builder.create_instance(ground_id, checkered_id);

        // Add sphere
        for i in 0..=5 {
            let props = PbrMaterialProperties {
                roughness: 0.2 * i as Float,
                specular: 0.5,
                metallic: 0.0,
                fresnel: 0.0,
            };
            let mat_id = scene_builder.add_material(materials::pbr_metal([0.3, 0.2, 0.8], props));
            scene_builder.add_instance(Instance::new(sphere_id, mat_id).translate([1.1 * i as Float - 2.75, 0.5, 0.0]));
        }

        scene_builder.build()
    }
}
