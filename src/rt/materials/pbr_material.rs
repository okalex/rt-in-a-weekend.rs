use std::sync::Arc;

use crate::{
    rt::{
        geometry::hit_record::HitRecord,
        materials::material::{
            reflect,
            ScatterRecord,
        },
        pdf::{
            Pdf,
            SpherePdf,
        },
        ray::Ray,
    },
    util::{
        color::Color,
        random::rand_on_hemisphere,
        types::{
            Float,
            Vector,
        },
    },
};

pub struct PbrMaterial {
    pub albedo: Color,
    // pub roughness: f64,
    // pub opacity: f64,
    pub metallicity: Float,
    // pub color_map: Option<ImageMap>,
    // pub normal_map: Option<ImageMap>,
    // pub roughness_map: Option<ImageMap>,
    // pub metallicity_map: Option<ImageMap>,
}

impl PbrMaterial {
    pub fn new(albedo: Color, metallicity: Float) -> Self {
        Self { albedo, metallicity }
    }

    #[allow(unused)]
    fn diffuse_scatter(&self, in_dir: Vector, normal: Vector) -> Vector {
        let diffuse_scale = 1.0 - self.metallicity;
        if diffuse_scale > 0.0 {
            normal + rand_on_hemisphere(normal)
        } else {
            Vector::ZERO
        }
    }

    fn metallic_scatter(&self, in_dir: Vector, normal: Vector) -> Vector {
        if self.metallicity > 0.0 {
            reflect(in_dir, normal).normalize()
        } else {
            Vector::ZERO
        }
    }

    #[allow(unused)]
    pub fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        // Diffuse scatter
        let mut scatter_dir = self.diffuse_scatter(r_in.dir, rec.normal) + self.metallic_scatter(r_in.dir, rec.normal);

        if all_are_less_than(scatter_dir, 1e-8) {
            scatter_dir = rec.normal;
        }

        Some(ScatterRecord {
            attenuation: self.albedo,
            pdf: Arc::new(Pdf::Sphere(SpherePdf::new())), // TODO
            skip_pdf_ray: Some(Ray::new(rec.point, scatter_dir, r_in.time)),
        })
    }
}

fn all_are_less_than(vec: Vector, limit: Float) -> bool {
    (vec.x.abs() < limit) && (vec.y.abs() < limit) && (vec.z.abs() < limit)
}
