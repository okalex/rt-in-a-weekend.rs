use std::sync::Arc;

use encase::ShaderType;
use encase_enum::ShaderEnum;
use glam::Vec3;

use crate::rt::{
    camera::Camera,
    materials::material::Material,
    objects::{hittable::Hittable, scene::Scene},
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
pub struct GpuObjects {
    #[shader(size(runtime))]
    objects: Vec<GpuShape>,
}

impl GpuObjects {
    pub fn new(scene: Arc<Scene>) -> Self {
        Self {
            objects: vec![
                GpuShape::Sphere {
                    center: Vec3::new(1.0, -100.0, -1.0),
                    radius: 100.0,
                    mat_idx: 7,
                },
                GpuShape::Sphere {
                    center: Vec3::new(0.0, 0.5, 0.0),
                    radius: 0.5,
                    mat_idx: 3,
                },
                GpuShape::Sphere {
                    center: Vec3::new(1.0, 0.5, 0.0),
                    radius: 0.5,
                    mat_idx: 11,
                },
                GpuShape::Sphere {
                    center: Vec3::new(-1.0, 0.5, 0.0),
                    radius: 0.5,
                    mat_idx: 8,
                },
                GpuShape::Sphere {
                    center: Vec3::new(-1.0, 0.5, 0.0),
                    radius: 0.3,
                    mat_idx: 9,
                },
            ],
            // objects: scene.objects.iter().map(|object| {
            //     GpuShape::from(object)
            // })
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
        Self {
            materials: gpu_materials,
        }
    }
}

#[derive(ShaderEnum, Debug)]
pub enum GpuShape {
    Sphere {
        center: Vec3,
        radius: f32,
        mat_idx: u32,
    },
}

impl From<Arc<Hittable>> for GpuShape {
    fn from(hittable: Arc<Hittable>) -> Self {
        match hittable.as_ref() {
            Hittable::Sphere(obj) => GpuShape::Sphere {
                center: obj.center.orig, // TODO: support moving later
                radius: obj.radius,
                mat_idx: obj.mat_idx as u32,
            },
            _ => panic!(),
        }
    }
}

#[derive(ShaderEnum, Debug)]
pub enum GpuMaterial {
    Lambertian { texture: GpuTexture },
    Metal { albedo: Vec3, fuzz: f32 },
    Dielectric { refraction_idx: f32 },
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
    SolidColor {
        albedo: Vec3,
    },
    Checkered {
        inv_scale: f32,
        even: Vec3,
        odd: Vec3,
    },
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
