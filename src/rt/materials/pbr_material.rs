use crate::{
    rt::{
        geometry::hit_record::HitRecord,
        materials::material::{ScatterRecord, reflect},
        pdf::CosinePdf,
        ray::Ray,
    },
    util::{
        color::Color,
        random::{rand},
        types::{Float},
    },
};

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

    pub fn mix_colors(a: Color, b: Color, control: Float) -> Color {
        a * control + b * (1.0 - control)
    }

    #[allow(unused)]
    pub fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let alpha = self.roughness * self.roughness;

        let f0 = Self::mix_colors(Color::fill(0.08 * self.specular), self.albedo, self.metallic);
        let cos_theta = r_in.dir.dot(hit_record.normal);
        // let fresnel = schlick(f0, cos_theta);

        // TODO: use luminance to determine which lobe to evaluate
        if rand() < 0.5 {
            // Specular lobe
        } else {
            // Diffuse lobe
        }

        // Diffuse scatter
        let cosine_pdf = CosinePdf::new(&hit_record.normal);
        let scatter_dir = (1.0 - self.roughness) * reflect(r_in.dir, hit_record.normal) + self.roughness * cosine_pdf.generate();

        Some(ScatterRecord::skip_pdf(
            self.albedo,
            Ray::new(hit_record.point, scatter_dir, r_in.time),
        ))
    }
}
