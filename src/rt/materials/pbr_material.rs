use std::sync::Arc;

use crate::{
    rt::{geometry::hit_record::HitRecord, materials::material::ScatterRecord, pdf::Pdf, ray::Ray},
    util::{
        color::Color,
        types::{Float, PI, Vector},
    },
};

const MIN_ROUGHNESS: Float = 0.1;
const HALF_VECTOR_EPSILON: Float = 1e-12;

pub struct PbrMaterialProperties {
    pub roughness: Float,
    pub specular: Float,
    pub metallic: Float,
    pub fresnel: Float,
}

#[allow(unused)]
pub struct PbrMaterial {
    pub albedo: Color,
    pub roughness: Float,
    pub specular: Float,
    pub metallic: Float,
    pub fresnel: Float,
    pub diffuse: Float,
}

impl PbrMaterial {
    pub fn new(albedo: Color, props: PbrMaterialProperties) -> Self {
        let diffuse = (1.0 - props.specular) * (1.0 - props.metallic);
        Self {
            albedo,
            roughness: props.roughness,
            specular: props.specular,
            metallic: props.metallic,
            fresnel: props.fresnel,
            diffuse,
        }
    }

    pub fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let alpha = self.alpha();
        let wo = -r_in.dir.normalize();

        let f0 = Self::mix_colors(self.albedo, Color::fill(0.08 * self.specular), self.metallic);
        let p_spec = Self::specular_sample_weight(f0.luminance(), self.roughness, self.specular);

        Some(ScatterRecord::with_pdf(
            Color::white(), // TODO: unused because attenuation is calculated by brdf
            Arc::new(Pdf::mixture(
                Arc::new(Pdf::ggx(wo, hit_record.normal, alpha)),
                Arc::new(Pdf::cosine(&hit_record.normal)),
                p_spec,
            )),
        ))
    }

    pub fn brdf(&self, r_in: &Ray, hit_record: &HitRecord, scattered_dir: &Vector) -> Color {
        let alpha = self.alpha();
        let wo = -r_in.dir.normalize();
        let wi = scattered_dir.normalize();
        let h = match Self::half_vector(wi, wo) {
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

        let f0 = Self::mix_colors(self.albedo, Color::fill(0.08 * self.specular), self.metallic);
        let f = Self::schlick_fresnel(f0, v_dot_h);
        let d = Self::ggx_d(n_dot_h, alpha);
        let g = Self::smith_g2(n_dot_i, n_dot_o, alpha);

        let diffuse = (Color::white() - f) * (1.0 - self.metallic) * self.albedo / PI;
        let specular = d * f * g / (4.0 * n_dot_i * n_dot_o);

        diffuse + specular
    }

    pub fn mix_colors(a: Color, b: Color, control: Float) -> Color {
        a * control + b * (1.0 - control)
    }

    pub fn alpha_from_roughness(roughness: Float) -> Float {
        let clamped_roughness = roughness.max(MIN_ROUGHNESS);
        clamped_roughness * clamped_roughness
    }

    fn alpha(&self) -> Float {
        Self::alpha_from_roughness(self.roughness)
    }

    fn half_vector(wi: Vector, wo: Vector) -> Option<Vector> {
        let half_vec = wi + wo;
        let len_sq = half_vec.length_squared();

        if !len_sq.is_finite() || len_sq <= HALF_VECTOR_EPSILON {
            return None;
        }

        Some(half_vec / len_sq.sqrt())
    }

    fn specular_sample_weight(f0_luminance: Float, roughness: Float, specular: Float) -> Float {
        let smoothness = 1.0 - roughness.clamp(0.0, 1.0);
        (f0_luminance + smoothness * specular).clamp(0.1, 0.9)
    }

    pub fn ggx_d(n_dot_h: Float, alpha: Float) -> Float {
        let alpha_sqrd = alpha * alpha;
        let n_dot_h_sqrd = n_dot_h * n_dot_h;
        let denom = n_dot_h_sqrd * (alpha_sqrd - 1.0) + 1.0;
        alpha_sqrd / (PI * denom * denom)
    }

    pub fn smith_g1(n_dot_o: Float, alpha: Float) -> Float {
        let alpha_sqrd = alpha * alpha;
        let n_dot_o_sqrd = n_dot_o * n_dot_o;
        let denom = n_dot_o + (alpha_sqrd + (1.0 - alpha_sqrd) * n_dot_o_sqrd).sqrt();
        2.0 * n_dot_o / denom
    }

    pub fn smith_g2(n_dot_i: Float, n_dot_o: Float, alpha: Float) -> Float {
        let alpha_sqrd = alpha * alpha;
        let n_dot_i_sqrd = n_dot_i * n_dot_i;
        let n_dot_o_sqrd = n_dot_o * n_dot_o;
        let a = alpha_sqrd + (1.0 - alpha_sqrd) * n_dot_i_sqrd;
        let b = alpha_sqrd + (1.0 - alpha_sqrd) * n_dot_o_sqrd;
        let denom = n_dot_o * a.sqrt() + n_dot_i * b.sqrt();
        2.0 * n_dot_i * n_dot_o / denom
    }

    pub fn schlick_fresnel(f0: Color, v_dot_h: Float) -> Color {
        let cos_theta = v_dot_h.clamp(0.0, 1.0);
        f0 + (1.0 - f0) * (1.0 - cos_theta).powf(5.0)
    }
}

#[cfg(test)]
mod tests {
    use super::{PbrMaterial, PbrMaterialProperties};
    use crate::{
        rt::{geometry::hit_record::HitRecord, ray::Ray},
        util::{color::Color, types::Vector},
    };

    #[test]
    fn brdf_returns_black_for_degenerate_half_vector() {
        let material = PbrMaterial::new(
            Color::from([0.3, 0.2, 0.8]),
            PbrMaterialProperties {
                roughness: 0.0,
                specular: 0.5,
                metallic: 0.0,
                fresnel: 0.0,
            },
        );
        let hit_record = HitRecord::new(Vector::ZERO, Vector::Z, true, 1.0, 0.0, 0.0);
        let ray = Ray::new(Vector::ZERO, -Vector::Z, 0.0);

        let brdf = material.brdf(&ray, &hit_record, &(-Vector::Z));

        assert!(brdf.is_finite());
        assert_eq!(brdf.r(), 0.0);
        assert_eq!(brdf.g(), 0.0);
        assert_eq!(brdf.b(), 0.0);
    }
}
