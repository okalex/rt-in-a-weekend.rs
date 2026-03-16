use std::sync::Arc;

use crate::rt::color::Color;
use crate::rt::materials::dielectric::Dielectric;
use crate::rt::materials::diffuse_light::DiffuseLight;
use crate::rt::materials::isotropic::Isotropic;
use crate::rt::materials::lambertian::Lambertian;
use crate::rt::materials::metal::Metal;
use crate::rt::materials::pbr_material::PbrMaterial;
use crate::rt::objects::hit_record::HitRecord;
use crate::rt::pdf::Pdf;
use crate::rt::ray::Ray;
use crate::rt::types::{Float, Vector};

pub struct ScatterRecord {
    pub attenuation: Color,
    pub pdf: Arc<Pdf>,
    pub skip_pdf_ray: Option<Ray>,
}

pub enum Material {
    Dielectric(Dielectric),
    DiffuseLight(DiffuseLight),
    Isotropic(Isotropic),
    Lambertian(Lambertian),
    Metal(Metal),
    PbrMaterial(PbrMaterial),
}

impl Material {
    pub fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let default = None;
        match self {
            Self::Dielectric(mat) => mat.scatter(r_in, rec),
            Self::DiffuseLight(_) => default,
            Self::Isotropic(mat) => mat.scatter(r_in, rec),
            Self::Lambertian(mat) => mat.scatter(r_in, rec),
            Self::Metal(mat) => mat.scatter(r_in, rec),
            Self::PbrMaterial(mat) => mat.scatter(r_in, rec),
        }
    }

    pub fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> Float {
        let default = 0.0;
        match self {
            Self::Dielectric(_) => default,
            Self::DiffuseLight(_) => default,
            Self::Isotropic(mat) => mat.scattering_pdf(r_in, rec, scattered),
            Self::Lambertian(mat) => mat.scattering_pdf(r_in, rec, scattered),
            Self::Metal(_) => default,
            Self::PbrMaterial(_) => default,
        }
    }

    pub fn emitted(&self, r_in: &Ray, hit_record: &HitRecord) -> Color {
        let default = Color::black();
        match self {
            Self::Dielectric(_) => default,
            Self::DiffuseLight(mat) => mat.emitted(r_in, hit_record),
            Self::Isotropic(_) => default,
            Self::Lambertian(_) => default,
            Self::Metal(_) => default,
            Self::PbrMaterial(_) => default,
        }
    }
}

pub fn reflectance(cosine: Float, refraction_idx: Float) -> Float {
    let r0_tmp = (1.0 - refraction_idx) / (1.0 + refraction_idx);
    let r0 = r0_tmp * r0_tmp;
    return r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0);
}

pub fn reflect(vec: &Vector, normal: &Vector) -> Vector {
    *vec - *normal * vec.dot(normal) * 2.0
}

pub fn refract(vec: &Vector, n: &Vector, etai_over_etat: Float) -> Vector {
    let cos_theta = Float::min(-vec.dot(n), 1.0);
    let r_out_perp = (*vec + *n * cos_theta) * etai_over_etat;
    let r_out_parallel = *n * (-(1.0 - r_out_perp.magnitude_squared()).abs().sqrt());
    return r_out_perp + r_out_parallel;
}
