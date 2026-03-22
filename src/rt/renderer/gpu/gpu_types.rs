use std::sync::Arc;

use encase::ShaderType;
use encase_enum::ShaderEnum;
use glam::{
    Mat4,
    Vec3,
};
use obvhs::bvh2::Bvh2;

use crate::rt::{
    camera::Camera,
    geometry::{
        primitive::Primitive,
        scene::{
            Instance,
            Scene,
        },
    },
    materials::material::Material,
    renderer::render_options::RenderOptions,
    textures::texture::Texture,
    viewport::Viewport,
};

#[derive(ShaderType, Debug)]
pub struct GpuMeta {
    pub width: u32,
    pub height: u32,
    pub num_samples: u32,
    pub frame_num: u32,
    pub max_depth: u32,
    pub background: Vec3,
    pub camera: GpuCamera,
    pub viewport: GpuViewport,
}

impl GpuMeta {
    pub fn new(render_options: Arc<RenderOptions>, camera: Arc<Camera>, frame_num: u32) -> Self {
        Self {
            width: render_options.img_width,
            height: render_options.img_height,
            num_samples: render_options.dispatch_size,
            frame_num,
            background: render_options.background.base,
            max_depth: render_options.max_depth,
            camera: GpuCamera::from(Arc::clone(&camera)),
            viewport: GpuViewport::from(&Arc::clone(&camera).viewport),
        }
    }
}

#[derive(ShaderType, Debug)]
pub struct GpuCamera {
    position: Vec3,
    lookat: Vec3,
    defocus_angle: f32,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl From<Arc<Camera>> for GpuCamera {
    fn from(camera: Arc<Camera>) -> Self {
        Self {
            position: camera.options.position,
            lookat: camera.options.target,
            defocus_angle: camera.options.defocus_angle,
            defocus_disk_u: camera.defocus_disk.u,
            defocus_disk_v: camera.defocus_disk.v,
        }
    }
}

#[derive(ShaderType, Debug)]
pub struct GpuViewport {
    delta_u: Vec3,
    delta_v: Vec3,
    pixel00_loc: Vec3,
}

impl From<&Viewport> for GpuViewport {
    fn from(viewport: &Viewport) -> Self {
        Self {
            delta_u: viewport.delta_u,
            delta_v: viewport.delta_v,
            pixel00_loc: viewport.pixel00_loc,
        }
    }
}

#[derive(ShaderType, Debug)]
pub struct GpuBvh {
    #[shader(size(runtime))]
    nodes: Vec<GpuBvhNode>,
}

impl From<&Bvh2> for GpuBvh {
    fn from(bvh: &Bvh2) -> Self {
        Self {
            nodes: bvh
                .nodes
                .iter()
                .map(|node| {
                    let aabb = node.aabb;
                    let left_or_prim = if node.is_leaf() {
                        bvh.primitive_indices[node.first_index as usize]
                    } else {
                        node.first_index
                    };
                    GpuBvhNode {
                        aabb_min: Vec3::from(aabb.min),
                        aabb_max: Vec3::from(aabb.max),
                        left_or_prim,
                        count: node.prim_count,
                    }
                })
                .collect(),
        }
    }
}

#[derive(ShaderType, Debug)]
pub struct GpuBvhNode {
    aabb_min: Vec3,
    aabb_max: Vec3,
    left_or_prim: u32,
    count: u32,
}

#[derive(ShaderType, Debug)]
pub struct GpuPrimitives {
    #[shader(size(runtime))]
    primitives: Vec<GpuPrimitive>,
}

impl GpuPrimitives {
    pub fn new(scene: Arc<Scene>) -> Self {
        let primitives = scene.primitives.iter().map(|prim| GpuPrimitive::from(prim)).collect();

        Self { primitives }
    }
}

#[derive(ShaderEnum, Debug)]
pub enum GpuPrimitive {
    Sphere {
        center: Vec3,
        radius: f32,
        radius_sqrd: f32,
    },

    Triangle {
        a: Vec3,
        b: Vec3,
        c: Vec3,
        e1: Vec3,
        e2: Vec3,
        normal: Vec3,
    },

    Quad {
        q: Vec3,
        u: Vec3,
        v: Vec3,
        w: Vec3,
        normal: Vec3,
        d: f32,
        area: f32,
    },
}

impl From<&Primitive> for GpuPrimitive {
    fn from(hittable: &Primitive) -> Self {
        match hittable {
            Primitive::Sphere(sphere) => GpuPrimitive::Sphere {
                center: sphere.center.orig, // TODO: support moving later
                radius: sphere.radius,
                radius_sqrd: sphere.radius * sphere.radius,
            },

            Primitive::Quad(quad) => GpuPrimitive::Quad {
                q: quad.q,
                u: quad.u,
                v: quad.v,
                w: quad.w,
                normal: quad.normal,
                d: quad.d,
                area: quad.area,
            },

            Primitive::Triangle(triangle) => GpuPrimitive::Triangle {
                a: triangle.a,
                b: triangle.b,
                c: triangle.c,
                e1: triangle.e1,
                e2: triangle.e2,
                normal: triangle.normal,
            },
        }
    }
}

#[derive(ShaderType, Debug)]
pub struct GpuInstances {
    #[shader(size(runtime))]
    instances: Vec<GpuInstance>,
}

impl GpuInstances {
    pub fn new(scene: Arc<Scene>) -> Self {
        let instances = scene.instances.iter().map(|instance| GpuInstance::from(instance)).collect();

        Self { instances }
    }
}

#[derive(ShaderType, Debug)]
pub struct GpuInstance {
    primitive_id: u32,
    material_id: u32,
    transform: Mat4,
    inv_transform: Mat4,
}

impl From<&Instance> for GpuInstance {
    fn from(instance: &Instance) -> Self {
        Self {
            primitive_id: instance.primitive_id.id as u32,
            material_id: instance.material_id.id as u32,
            transform: instance.transform,
            inv_transform: instance.inv_transform,
        }
    }
}

#[derive(ShaderType, Debug)]
pub struct GpuMaterials {
    #[shader(size(runtime))]
    materials: Vec<GpuMaterial>,
}

impl From<&Vec<Material>> for GpuMaterials {
    fn from(materials: &Vec<Material>) -> Self {
        let gpu_materials: Vec<GpuMaterial> = materials.iter().map(GpuMaterial::from).collect();
        Self { materials: gpu_materials }
    }
}

#[derive(ShaderEnum, Debug)]
pub enum GpuMaterial {
    Lambertian { texture: GpuTexture },
    Metal { albedo: Vec3, fuzz: f32 },
    Dielectric { refraction_idx: f32 },
    Emissive { color: Vec3 },
}

impl From<&Material> for GpuMaterial {
    fn from(material: &Material) -> Self {
        match material {
            Material::Lambertian(mat) => Self::Lambertian {
                texture: GpuTexture::from(&mat.texture),
            },

            Material::Metal(mat) => Self::Metal {
                albedo: mat.albedo.base,
                fuzz: mat.fuzz,
            },

            Material::Dielectric(mat) => Self::Dielectric {
                refraction_idx: mat.refraction_idx,
            },

            Material::Emissive(mat) => match mat.texture.as_ref() {
                Texture::Solid(color) => Self::Emissive { color: color.albedo.base },
                _ => panic!(),
            },

            // TODO: Handle other materials properly
            _ => Self::Lambertian {
                texture: GpuTexture::SolidColor {
                    albedo: Vec3::new(0.5, 0.5, 0.5),
                },
            },
        }
    }
}

impl From<Material> for GpuMaterial {
    fn from(material: Material) -> Self {
        Self::from(&material)
    }
}

impl From<Arc<Material>> for GpuMaterial {
    fn from(material: Arc<Material>) -> Self {
        Self::from(material.as_ref())
    }
}

#[derive(ShaderEnum, Debug)]
pub enum GpuTexture {
    SolidColor { albedo: Vec3 },
    Checkered { inv_scale: f32, even: Vec3, odd: Vec3 },
}

impl From<&Texture> for GpuTexture {
    fn from(texture: &Texture) -> Self {
        match texture {
            Texture::Solid(tex) => Self::SolidColor { albedo: tex.albedo.base },

            Texture::Checkered(tex) => Self::Checkered {
                inv_scale: tex.inv_scale,
                even: tex.even.albedo.base,
                odd: tex.odd.albedo.base,
            },

            // TODO: other textures
            _ => Self::SolidColor {
                albedo: Vec3::new(0.5, 0.5, 0.5),
            },
        }
    }
}

impl From<Texture> for GpuTexture {
    fn from(texture: Texture) -> Self {
        Self::from(&texture)
    }
}

impl From<Arc<Texture>> for GpuTexture {
    fn from(texture: Arc<Texture>) -> Self {
        Self::from(texture.as_ref())
    }
}

impl From<&Arc<Texture>> for GpuTexture {
    fn from(texture: &Arc<Texture>) -> Self {
        Self::from(texture.as_ref())
    }
}
