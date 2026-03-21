use crate::rt::{
    random::rand,
    types::Float,
};

pub mod materials {
    use std::sync::Arc;

    use crate::rt::{
        color::Color,
        materials::{
            dielectric::Dielectric,
            emissive::Emissive,
            isotropic::Isotropic,
            lambertian::Lambertian,
            material::Material,
            metal::Metal,
            pbr_material::PbrMaterial,
        },
        random::rand_range,
        test_helpers::{
            rand_arr3,
            textures,
        },
        textures::{
            image_map::ImageMap,
            perlin_noise::PerlinNoise,
            texture::Texture,
        },
        types::Float,
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
            checkered: from_texture(textures::checkers()),
            glass: dielectric(1.5),
            air: dielectric(1.0 / 1.5),
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

    #[allow(dead_code)]
    pub fn rand_lambertian() -> Material {
        let albedo = rand_arr3();
        lambertian(albedo)
    }

    pub fn dielectric(refraction_idx: Float) -> Material {
        Material::Dielectric(Dielectric::new(refraction_idx))
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

    #[allow(dead_code)]
    pub fn isotropic(albedo: [Float; 3]) -> Material {
        Material::Isotropic(Isotropic::from(albedo))
    }
}

pub mod textures {
    use std::sync::Arc;

    use crate::rt::textures::{
        checkered::Checkered,
        texture::Texture,
    };

    pub fn checkers() -> Arc<Texture> {
        Arc::new(Texture::Checkered(Checkered::from_color_values(
            0.32,
            [0.2, 0.3, 0.1],
            [0.9, 0.9, 0.9],
        )))
    }
}

fn rand_arr3() -> [Float; 3] {
    [rand(), rand(), rand()]
}

pub mod primitives {

    use crate::rt::{
        geometry::{
            primitive::Primitive,
        },
        types::{
            Float,
            Point,
        },
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

    // pub fn quad(q: [Float; 3], u: [Float; 3], v: [Float; 3], mat_idx: usize) -> Primitive {
    //     Hittable::Quad(Quad::from_arr(q, u, v, mat_idx))
    // }

    // pub fn triangle(a: [Float; 3], b: [Float; 3], c: [Float; 3], mat_idx: usize) -> Primitive {
    //     Hittable::Triangle(Triangle::new(a, b, c, mat_idx))
    // }

    // pub fn box3d(a: [Float; 3], b: [Float; 3], mat_idx: usize) -> Primitive {
    //     Hittable::HittableList(Box3d::new(Vector::from(a), Vector::from(b), mat_idx))
    // }

    // pub fn constant_medium(
    //     materials: &mut Materials,
    //     boundary: Arc<Hittable>,
    //     color: [Float; 3],
    //     density: Float,
    // ) -> Hittable {
    //     let mat = Materials::isotropic(color);
    //     let mat_idx = materials.add(mat);
    //     Hittable::ConstantMedium(ConstantMedium::new(boundary, density, mat_idx))
    // }

    // pub fn checkered_ground(materials: &Materials) -> Hittable {
    //     ground_sphere(materials.get("checkered"))
    // }
}
