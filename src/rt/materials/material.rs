use std::sync::Arc;

use crate::{
    rt::{
        geometry::hit_record::HitRecord,
        materials::{
            dielectric::Dielectric, emissive::Emissive, isotropic::Isotropic, lambertian::Lambertian, metal::Metal,
            pbr_material::PbrMaterial,
        },
        pdf::Pdf,
        ray::Ray,
    },
    util::{
        color::Color,
        types::{Float, Vector},
    },
};

pub struct ScatterRecord {
    pub attenuation: Color,
    pub pdf: Option<Arc<Pdf>>,
    pub skip_pdf_ray: Option<Ray>,
}

impl ScatterRecord {
    pub fn with_pdf(attenuation: Color, pdf: Arc<Pdf>) -> Self {
        Self {
            attenuation,
            pdf: Some(pdf),
            skip_pdf_ray: None,
        }
    }

    pub fn skip_pdf(attenuation: Color, skip_pdf_ray: Ray) -> Self {
        Self {
            attenuation,
            pdf: None,
            skip_pdf_ray: Some(skip_pdf_ray),
        }
    }
}

pub enum Material {
    Dielectric(Dielectric),
    Emissive(Emissive),
    Isotropic(Isotropic),
    Lambertian(Lambertian),
    Metal(Metal),
    PbrMaterial(PbrMaterial),
}

impl Material {
    pub fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let default = None;
        match self {
            Self::Dielectric(mat) => mat.scatter(r_in, hit_record),
            Self::Emissive(_) => default,
            Self::Isotropic(mat) => mat.scatter(r_in, hit_record),
            Self::Lambertian(mat) => mat.scatter(r_in, hit_record),
            Self::Metal(mat) => mat.scatter(r_in, hit_record),
            Self::PbrMaterial(mat) => mat.scatter(r_in, hit_record),
        }
    }

    pub fn emitted(&self, r_in: &Ray, hit_record: &HitRecord) -> Color {
        let default = Color::black();
        match self {
            Self::Dielectric(_) => default,
            Self::Emissive(mat) => mat.emitted(r_in, hit_record),
            Self::Isotropic(_) => default,
            Self::Lambertian(_) => default,
            Self::Metal(_) => default,
            Self::PbrMaterial(_) => default,
        }
    }

    pub fn brdf(&self, r_in: &Ray, hit_record: &HitRecord, scattered_dir: &Vector) -> Color {
        let default = Color::black();
        match self {
            Self::Dielectric(_) => default,
            Self::Emissive(_) => default,
            Self::Isotropic(mat) => mat.brdf(r_in, hit_record, scattered_dir),
            Self::Lambertian(mat) => mat.brdf(r_in, hit_record, scattered_dir),
            Self::Metal(_) => default,
            Self::PbrMaterial(mat) => mat.brdf(r_in, hit_record, scattered_dir),
        }
    }
}

pub fn reflectance(cosine: Float, refraction_idx: Float) -> Float {
    let r0_tmp = (1.0 - refraction_idx) / (1.0 + refraction_idx);
    let r0 = r0_tmp * r0_tmp;
    return r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0);
}

pub fn reflect(vec: Vector, normal: Vector) -> Vector {
    vec - normal * vec.dot(normal) * 2.0
}

pub fn refract(vec: Vector, n: Vector, etai_over_etat: Float) -> Vector {
    let cos_theta = Float::min(-vec.dot(n), 1.0);
    let r_out_perp = (vec + n * cos_theta) * etai_over_etat;
    let r_out_parallel = n * -(1.0 - r_out_perp.length_squared()).abs().sqrt();
    return r_out_perp + r_out_parallel;
}
