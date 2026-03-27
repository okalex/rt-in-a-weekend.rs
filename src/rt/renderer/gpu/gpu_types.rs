use std::sync::Arc;

use encase::ShaderType;
use encase_enum::ShaderEnum;
use glam::{Mat4, Vec2, Vec3};
use obvhs::bvh2::Bvh2;

use crate::rt::{
    geometry::{
        primitive::Primitive,
        scene::{Instance, InstanceId, Scene},
        triangle::Triangle,
    },
    materials::material::Material,
    textures::texture::Texture,
};

pub struct GpuScene {
    pub primitives: GpuPrimitives,
    pub instances: GpuInstances,
    pub instance_bvh: GpuBvh,
    pub meshes: GpuMeshes,
    pub mesh_bvhs: GpuBvh,
    pub materials: GpuMaterials,
    pub lights: GpuLights,
}

#[derive(ShaderType, Debug)]
pub struct GpuMeshes {
    #[shader(size(runtime))]
    pub triangles: Vec<GpuPrimitive>,
}

impl GpuMeshes {
    pub fn new(triangles: Vec<GpuPrimitive>) -> Self {
        Self { triangles }
    }
}

impl From<&Arc<Scene>> for GpuScene {
    fn from(scene: &Arc<Scene>) -> Self {
        let mut mesh_bvhs: Vec<GpuBvhNode> = vec![];
        let mut mesh_triangles: Vec<GpuPrimitive> = vec![];

        let primitives: Vec<_> = scene
            .primitives
            .iter()
            .map(|primitive| match primitive {
                Primitive::Mesh(m) => {
                    let mesh = scene.get_mesh(&m.id).unwrap();

                    let first_triangle_id = mesh_triangles.len() as u32;
                    for triangle in &mesh.triangles {
                        let gpu_primitive = GpuPrimitive::triangle(triangle);
                        mesh_triangles.push(gpu_primitive);
                    }

                    let bvh_id = mesh_bvhs.len() as u32;
                    let bvh = GpuBvh::from_bvh(&mesh.bvh, bvh_id, first_triangle_id);
                    for node in bvh.nodes {
                        mesh_bvhs.push(node);
                    }

                    GpuPrimitive::Mesh { mesh_bvh_id: bvh_id }
                }

                _ => GpuPrimitive::from(primitive),
            })
            .collect();

        // let primitives = GpuPrimitives::new(&scene.primitives);
        let instances = GpuInstances::new(&scene.instances);
        let materials = GpuMaterials::new(&scene.materials);
        let bvh = GpuBvh::from_bvh(&scene.bvh, 0, 0);

        Self {
            primitives: GpuPrimitives::new(primitives),
            instances,
            instance_bvh: bvh,
            meshes: GpuMeshes::new(mesh_triangles),
            mesh_bvhs: GpuBvh::new(mesh_bvhs),
            materials,
            lights: GpuLights::new(&scene.lights),
        }
    }
}

#[derive(ShaderType, Debug)]
pub struct GpuBvh {
    #[shader(size(runtime))]
    nodes: Vec<GpuBvhNode>,
}

impl GpuBvh {
    fn new(nodes: Vec<GpuBvhNode>) -> Self {
        Self { nodes }
    }

    fn from_bvh(bvh: &Bvh2, bvh_offset: u32, leaf_offset: u32) -> Self {
        Self {
            nodes: bvh
                .nodes
                .iter()
                .map(|node| {
                    let aabb = node.aabb;
                    let left_or_prim = if node.is_leaf() {
                        leaf_offset + bvh.primitive_indices[node.first_index as usize]
                    } else {
                        bvh_offset + node.first_index
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
    pub fn new(primitives: Vec<GpuPrimitive>) -> Self {
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
        v0: Vec3,
        v1: Vec3,
        v2: Vec3,
        uv0: Vec2,
        uv1: Vec2,
        uv2: Vec2,
        e01: Vec3,
        e02: Vec3,
        normal0: Vec3,
        normal1: Vec3,
        normal2: Vec3,
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

    Mesh {
        mesh_bvh_id: u32,
    },

    Medium {
        boundary_primitive_id: u32,
        neg_inv_density: f32,
    },
}

impl GpuPrimitive {
    fn triangle(tri: &Triangle) -> Self {
        GpuPrimitive::Triangle {
            v0: tri.v[0],
            v1: tri.v[1],
            v2: tri.v[2],
            uv0: tri.uv[0],
            uv1: tri.uv[1],
            uv2: tri.uv[2],
            e01: tri.e01,
            e02: tri.e02,
            normal0: tri.normal[0],
            normal1: tri.normal[1],
            normal2: tri.normal[2],
        }
    }
}

impl From<&Primitive> for GpuPrimitive {
    fn from(primitive: &Primitive) -> Self {
        match primitive {
            Primitive::Sphere(sphere) => GpuPrimitive::Sphere {
                center: sphere.center.orig,
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

            Primitive::Triangle(triangle) => GpuPrimitive::triangle(triangle),

            Primitive::Medium(medium) => GpuPrimitive::Medium {
                boundary_primitive_id: medium.boundary_id.id as u32,
                neg_inv_density: medium.neg_inv_density,
            },

            Primitive::Mesh(_) => panic!(), // This is added in scene construction
        }
    }
}

#[derive(ShaderType, Debug)]
pub struct GpuInstances {
    #[shader(size(runtime))]
    instances: Vec<GpuInstance>,
}

impl GpuInstances {
    pub fn new(instances: &Vec<Instance>) -> Self {
        let instances = instances.iter().map(|instance| GpuInstance::from(instance)).collect();
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

impl GpuMaterials {
    fn new(materials: &Vec<Material>) -> Self {
        let gpu_materials: Vec<GpuMaterial> = materials.iter().map(GpuMaterial::from).collect();
        Self {
            materials: gpu_materials,
        }
    }
}

#[derive(ShaderEnum, Debug)]
pub enum GpuMaterial {
    Lambertian { texture: GpuTexture },
    Metal { albedo: Vec3, fuzz: f32 },
    Dielectric { albedo: Vec3, refraction_idx: f32 },
    Emissive { color: Vec3 },
    Isotropic { texture: GpuTexture },
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
                albedo: mat.albedo.base,
                refraction_idx: mat.refraction_idx,
            },

            Material::Emissive(mat) => match mat.texture.as_ref() {
                Texture::Solid(color) => Self::Emissive {
                    color: color.albedo.base,
                },
                _ => panic!(),
            },

            Material::Isotropic(mat) => Self::Isotropic {
                texture: GpuTexture::from(&mat.texture),
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
            Texture::Solid(tex) => Self::SolidColor {
                albedo: tex.albedo.base,
            },

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

#[derive(ShaderType, Debug)]
pub struct GpuLights {
    #[shader(size(runtime))]
    lights: Vec<u32>,
}

impl GpuLights {
    pub fn new(lights: &Vec<InstanceId>) -> Self {
        Self {
            lights: lights.iter().map(|id| id.id as u32).collect(),
        }
    }
}
