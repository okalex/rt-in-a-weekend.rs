use std::sync::Arc;

use crate::{
    rt::{
        geometry::hit_record::HitRecord,
        materials::{
            ggx::{ggx_d, schlick_fresnel, smith_g2},
            material::ScatterRecord,
        },
        pdf::Pdf,
        ray::Ray,
    },
    util::{
        color::Color,
        random::rand,
        types::{Float, PI, Vector},
        vector_ext::{HALF_VECTOR_EPSILON, VectorExt},
    },
};

const MIN_ALPHA: Float = 1e-4;
const MIRROR_ROUGHNESS_EPSILON: Float = 1e-6;

pub struct PbrMaterialProperties {
    pub roughness: Float,
    pub metallic: Float,
    pub ior: Float,
}

#[allow(unused)]
pub struct PbrMaterial {
    pub albedo: Color,
    pub roughness: Float,
    pub metallic: Float,
    pub ior: Float,
    pub alpha: Float,
    pub f0: Color,
    pub p_spec: Float,
}

struct MirrorLimitWeights {
    specular_color: Color,
    diffuse_color: Color,
    specular_probability: Float,
    diffuse_probability: Float,
}

impl PbrMaterial {
    pub fn new(albedo: Color, props: PbrMaterialProperties) -> Self {
        let alpha = Float::max(props.roughness * props.roughness, MIN_ALPHA);
        let dielectric_f0 = Color::fill(Self::dielectric_f0_from_ior(props.ior));
        let f0 = Color::mix(albedo, dielectric_f0, props.metallic);
        let p_spec = Self::specular_sample_weight(f0.luminance(), props.roughness);

        Self {
            albedo,
            roughness: props.roughness,
            metallic: props.metallic,
            ior: props.ior,
            alpha,
            f0,
            p_spec,
        }
    }

    pub fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let wo = -r_in.dir.normalize();
        if self.is_mirror_limit() {
            return self.scatter_mirror_limit(r_in, hit_record, wo);
        }

        Some(ScatterRecord::with_pdf(
            Color::white(), // TODO: unused because attenuation is calculated by brdf
            Arc::new(Pdf::mixture(
                Arc::new(Pdf::ggx(wo, hit_record.normal, self.alpha)),
                Arc::new(Pdf::cosine(&hit_record.normal)),
                self.p_spec,
            )),
        ))
    }

    pub fn brdf(&self, r_in: &Ray, hit_record: &HitRecord, scattered_dir: &Vector) -> Color {
        let wo = -r_in.dir.normalize();
        let wi = scattered_dir.normalize();

        if self.is_mirror_limit() {
            return self.mirror_limit_diffuse_brdf(hit_record, wo);
        }

        let h = match VectorExt::half_vector(wi, wo) {
            Some(h) => h,
            None => return Color::black(),
        };

        let v_dot_h = wi.dot(h);
        let n_dot_h = hit_record.normal.dot(h);
        let n_dot_i = hit_record.normal.dot(wi);
        let n_dot_o = hit_record.normal.dot(wo);

        if n_dot_i <= 0.0 || n_dot_o <= 0.0 || n_dot_h <= 0.0 || v_dot_h <= 0.0 {
            return Color::black();
        }

        let alpha_sqrd = self.alpha * self.alpha;
        let f = schlick_fresnel(self.f0, v_dot_h);
        let d = ggx_d(n_dot_h, alpha_sqrd);
        let g = smith_g2(n_dot_i, n_dot_o, alpha_sqrd);

        let diffuse = (Color::white() - f) * (1.0 - self.metallic) * self.albedo / PI;
        let specular = d * f * g / (4.0 * n_dot_i * n_dot_o);

        diffuse + specular
    }

    fn is_mirror_limit(&self) -> bool {
        self.roughness <= MIRROR_ROUGHNESS_EPSILON
    }

    fn diffuse_color(&self, fresnel: Color) -> Color {
        (Color::white() - fresnel) * (1.0 - self.metallic) * self.albedo
    }

    fn dielectric_f0_from_ior(ior: Float) -> Float {
        let clamped_ior = ior.max(HALF_VECTOR_EPSILON);
        let reflectance = (clamped_ior - 1.0) / (clamped_ior + 1.0);
        reflectance * reflectance
    }

    fn mirror_limit_weights(&self, normal: Vector, wo: Vector) -> Option<MirrorLimitWeights> {
        let n_dot_o = normal.dot(wo).clamp(0.0, 1.0);
        let specular_color = schlick_fresnel(self.f0, n_dot_o);
        let diffuse_color = self.diffuse_color(specular_color);
        let specular_luma = specular_color.luminance();
        let diffuse_luma = diffuse_color.luminance();
        let total_luma = specular_luma + diffuse_luma;

        if total_luma <= 0.0 {
            return None;
        }

        let specular_probability = (specular_luma / total_luma).clamp(0.0, 1.0);
        Some(MirrorLimitWeights {
            specular_color,
            diffuse_color,
            specular_probability,
            diffuse_probability: 1.0 - specular_probability,
        })
    }

    fn scatter_mirror_limit(&self, r_in: &Ray, hit_record: &HitRecord, wo: Vector) -> Option<ScatterRecord> {
        let Some(weights) = self.mirror_limit_weights(hit_record.normal, wo) else {
            return None;
        };

        if weights.specular_probability >= 1.0
            || (weights.specular_probability > 0.0 && rand() < weights.specular_probability)
        {
            let reflected = VectorExt::reflect(-r_in.dir.normalize(), hit_record.normal);
            return Some(ScatterRecord::skip_pdf(
                weights.specular_color / weights.specular_probability.max(HALF_VECTOR_EPSILON),
                Ray::new(hit_record.point, reflected, r_in.time),
            ));
        }

        if weights.diffuse_probability <= 0.0 {
            return None;
        }

        Some(ScatterRecord::with_pdf(
            Color::white(),
            Arc::new(Pdf::cosine(&hit_record.normal)),
        ))
    }

    fn mirror_limit_diffuse_brdf(&self, hit_record: &HitRecord, wo: Vector) -> Color {
        let Some(weights) = self.mirror_limit_weights(hit_record.normal, wo) else {
            return Color::black();
        };

        if weights.diffuse_probability <= 0.0 {
            return Color::black();
        }

        weights.diffuse_color / (PI * weights.diffuse_probability.clamp(HALF_VECTOR_EPSILON, 1.0))
    }

    fn specular_sample_weight(f0_luminance: Float, roughness: Float) -> Float {
        let smoothness = 1.0 - roughness.clamp(0.0, 1.0);
        (f0_luminance + 0.5 * smoothness).clamp(0.1, 0.9)
    }
}
