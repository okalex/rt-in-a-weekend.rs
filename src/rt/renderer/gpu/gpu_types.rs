use std::sync::Arc;

use encase::ShaderType;
use encase_enum::ShaderEnum;
use glam::Vec3;

use crate::rt::{
    camera::Camera, materials::material::Material, renderer::render_options::RenderOptions,
    textures::texture::Texture,
};

#[derive(ShaderType, Debug)]
pub struct GpuMeta {
    pub width: u32,
    pub height: u32,
    pub camera: GpuCamera,
}

impl GpuMeta {
    pub fn new(render_options: Arc<RenderOptions>, camera: Arc<Camera>) -> Self {
        Self {
            width: render_options.img_width,
            height: render_options.img_height,
            camera: GpuCamera::from(camera),
        }
    }
}

#[derive(ShaderType, Debug)]
pub struct GpuCamera {
    position: Vec3,
    lookat: Vec3,
}

impl From<Arc<Camera>> for GpuCamera {
    fn from(camera: Arc<Camera>) -> Self {
        Self {
            position: camera.options.position,
            lookat: camera.options.target,
        }
    }
}

#[derive(ShaderEnum, Debug)]
pub enum GpuMaterial {
    Lambertian { texture: GpuTexture },
}

impl From<&Material> for GpuMaterial {
    fn from(material: &Material) -> Self {
        match material {
            Material::Lambertian(mat) => Self::Lambertian {
                texture: GpuTexture::from(&mat.texture),
            },
            _ => panic!(),
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
}

impl From<&Texture> for GpuTexture {
    fn from(texture: &Texture) -> Self {
        match texture {
            Texture::Solid(tex) => Self::SolidColor {
                albedo: tex.albedo.base,
            },
            _ => panic!(),
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
