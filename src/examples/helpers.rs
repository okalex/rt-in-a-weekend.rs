use crate::util::{random::rand, types::Float};

pub mod materials {
    use std::sync::Arc;

    use crate::{
        examples::helpers::{rand_arr3, textures},
        rt::{
            materials::{
                dielectric::Dielectric, emissive::Emissive, isotropic::Isotropic, lambertian::Lambertian, material::Material, metal::Metal,
                pbr_material::PbrMaterial,
            },
            textures::{image_map::ImageMap, perlin_noise::PerlinNoise, texture::Texture},
        },
        util::{color::Color, random::rand_range, types::Float},
    };

    #[allow(unused)]
    pub struct Defaults {
        pub default: Material,
        pub red: Material,
        pub white: Material,
        pub green: Material,
        pub blue: Material,
        pub orange: Material,
        pub teal: Material,
        pub diffuse_light: Material,
        pub checkered: Material,
        pub glass: Material,
        pub air: Material,
        pub mirror: Material,
        pub gold: Material,
        pub stone: Material,
        pub rusty_metal: Material,
        pub marble: Material,
        pub earth: Material,
        pub pbr: Material,
    }

    pub fn defaults() -> Defaults {
        Defaults {
            default: lambertian([0.5, 0.5, 0.5]),
            red: lambertian([0.65, 0.05, 0.05]),
            white: lambertian([0.73, 0.73, 0.73]),
            green: lambertian([0.12, 0.45, 0.15]),
            blue: lambertian([0.1, 0.2, 0.5]),
            orange: lambertian([1.0, 0.5, 0.0]),
            teal: lambertian([0.2, 0.8, 0.8]),
            diffuse_light: emissive([15.0, 15.0, 15.0]),
            checkered: checkered(0.32, [0.2, 0.3, 0.1], [0.9, 0.9, 0.9]),
            glass: dielectric([1.0, 1.0, 1.0], 1.5),
            air: dielectric([1.0, 1.0, 1.0], 1.0 / 1.5),
            mirror: metal([0.8, 0.85, 0.88], 0.0),
            gold: metal([0.8, 0.6, 0.2], 0.2),
            stone: image_map("assets/cube-diffuse.jpg", 1.0),
            rusty_metal: image_map("assets/rusty-metal.jpg", 1.0),
            marble: from_texture(Arc::new(Texture::Perlin(PerlinNoise::new(8.0)))),
            earth: image_map("assets/earthmap.jpg", 1.0),
            pbr: pbr([0.8, 0.6, 0.2], 0.7),
        }
    }

    pub fn lambertian(color: [Float; 3]) -> Material {
        Material::Lambertian(Lambertian::from(color))
    }

    pub fn from_texture(texture: Arc<Texture>) -> Material {
        Material::Lambertian(Lambertian::new(texture))
    }

    pub fn checkered(scale: Float, even: [Float; 3], odd: [Float; 3]) -> Material {
        from_texture(textures::checkers(scale, even, odd))
    }

    #[allow(dead_code)]
    pub fn rand_lambertian() -> Material {
        let albedo = rand_arr3();
        lambertian(albedo)
    }

    pub fn dielectric(albedo: [Float; 3], refraction_idx: Float) -> Material {
        Material::Dielectric(Dielectric::new(Color::from(albedo), refraction_idx))
    }

    pub fn metal(color: [Float; 3], fuzz: Float) -> Material {
        Material::Metal(Metal::new(color, fuzz))
    }

    #[allow(dead_code)]
    pub fn rand_metal() -> Material {
        let albedo = rand_arr3();
        let fuzz = rand_range(0.0, 0.5);
        metal(albedo, fuzz)
    }

    pub fn emissive(color: [Float; 3]) -> Material {
        Material::Emissive(Emissive::from(color))
    }

    pub fn image_map(file_name: &str, scale_factor: Float) -> Material {
        let tex = Arc::new(Texture::ImageMap(ImageMap::new(file_name, scale_factor)));
        from_texture(tex)
    }

    pub fn pbr(albedo: [Float; 3], metallicity: Float) -> Material {
        let color = Color::from(albedo);
        Material::PbrMaterial(PbrMaterial::new(color, metallicity))
    }

    pub fn isotropic(albedo: [Float; 3]) -> Material {
        Material::Isotropic(Isotropic::from(albedo))
    }
}

pub mod textures {
    use std::sync::Arc;

    use crate::{
        rt::textures::{checkered::Checkered, texture::Texture},
        util::types::Float,
    };

    pub fn checkers(scale: Float, even: [Float; 3], odd: [Float; 3]) -> Arc<Texture> {
        Arc::new(Texture::Checkered(Checkered::from_color_values(scale, even, odd)))
    }
}

fn rand_arr3() -> [Float; 3] {
    [rand(), rand(), rand()]
}

pub mod primitives {

    use crate::{
        rt::geometry::{constant_medium::ConstantMedium, primitive::Primitive, scene::PrimitiveId},
        util::types::{Float, Point, Vector},
    };

    pub struct Defaults {
        pub ground: Primitive,
    }

    pub fn defaults() -> Defaults {
        Defaults {
            ground: sphere([0.0, -200.0, 0.0], 200.0),
        }
    }

    pub fn sphere(center: [Float; 3], radius: Float) -> Primitive {
        Primitive::sphere(Point::from(center), radius)
    }

    pub fn quad(q: [Float; 3], u: [Float; 3], v: [Float; 3]) -> Primitive {
        Primitive::quad(Point::from(q), Vector::from(u), Vector::from(v))
    }

    pub fn triangle(a: [Float; 3], b: [Float; 3], c: [Float; 3]) -> Primitive {
        Primitive::triangle(Point::from_array(a), Point::from_array(b), Point::from_array(c))
    }

    pub fn medium(boundary_id: PrimitiveId, density: Float) -> Primitive {
        Primitive::Medium(ConstantMedium::new(boundary_id, density))
    }
}

pub mod meshes {
    use crate::{
        rt::geometry::{mesh::Mesh, triangle::Triangle},
        util::types::{Float, Point, Vector},
    };

    fn rect(q: Point, u: Vector, v: Vector) -> Vec<Triangle> {
        vec![Triangle::new(q, q + u + v, q + v), Triangle::new(q, q + u, q + u + v)]
    }

    pub fn make_box3d(q: Point, u: Vector, v: Vector, w: Vector) -> Mesh {
        let triangles: Vec<_> = vec![
            rect(q, u, v),     // front
            rect(q + w, u, v), // back
            rect(q, u, w),     // bottom
            rect(q + v, u, w), // top
            rect(q, v, w),     // left
            rect(q + u, v, w), // right
        ]
        .into_iter()
        .flatten()
        .collect();

        Mesh::new(triangles)
    }

    pub fn box3d(width: Float, height: Float, depth: Float) -> Mesh {
        make_box3d(
            Point::new(0.0, 0.0, 0.0),
            Vector::new(width, 0.0, 0.0),
            Vector::new(0.0, height, 0.0),
            Vector::new(0.0, 0.0, depth),
        )
    }
}

pub mod cornell_room {
    use crate::{
        examples::helpers::{materials, primitives},
        rt::{
            camera::CameraOptions,
            geometry::scene::{Instance, SceneBuilder},
        },
        util::trig::degrees_to_radians,
    };

    pub fn camera() -> CameraOptions {
        CameraOptions::new()
            .vfov(40.0)
            .position([500.0, 500.0, -1440.0])
            .target([500.0, 500.0, 0.0])
    }

    pub fn add_to_scene(scene_builder: &mut SceneBuilder) {
        let materials = materials::defaults();

        // Add materials
        let red_id = scene_builder.add_material(materials.red);
        let green_id = scene_builder.add_material(materials.green);
        let white_id = scene_builder.add_material(materials.white);
        let diffuse_light_id = scene_builder.add_material(materials.diffuse_light);

        // Make primitives
        let floor_prim = primitives::quad([0.0, 0.0, 0.0], [1000.0, 0.0, 0.0], [0.0, 0.0, 1000.0]);
        let wall_prim = primitives::quad([0.0, 0.0, 0.0], [1000.0, 0.0, 0.0], [0.0, 1000.0, 0.0]);
        let light_prim = primitives::quad([400.0, 999.9, 400.0], [200.0, 0.0, 0.0], [0.0, 0.0, 200.0]);

        // Add primitives
        let floor_id = scene_builder.add_primitive(floor_prim);
        let wall_id = scene_builder.add_primitive(wall_prim);
        let light_id = scene_builder.add_primitive(light_prim);

        // Make instances
        let floor = Instance::new(floor_id, white_id).translate([0.0, 0.0, 0.0]);
        let ceiling = Instance::new(floor_id, white_id).translate([0.0, 1000.0, 0.0]);
        let back_wall = Instance::new(wall_id, white_id).translate([0.0, 0.0, 1000.0]);
        let left_wall = Instance::new(wall_id, green_id)
            .rotate_y(degrees_to_radians(90.0))
            .translate([1000.0, 0.0, 1000.0]);
        let right_wall = Instance::new(wall_id, red_id)
            .rotate_y(degrees_to_radians(90.0))
            .translate([0.0, 0.0, 1000.0]);
        let light = Instance::new(light_id, diffuse_light_id);

        // Add instances
        scene_builder.add_instance(floor);
        scene_builder.add_instance(ceiling);
        scene_builder.add_instance(back_wall);
        scene_builder.add_instance(left_wall);
        scene_builder.add_instance(right_wall);

        let light_instance_id = scene_builder.add_instance(light);
        scene_builder.add_light(light_instance_id);
    }
}
