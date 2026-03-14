use std::sync::Arc;

use nalgebra::Vector3;

use crate::rt::{
    color::Color,
    materials::material::{ScatterRecord, reflect},
    objects::hit_record::HitRecord,
    pdf::{Pdf, SpherePdf},
    random::rand_on_hemisphere,
    ray::Ray,
};

pub struct PbrMaterial {
    pub albedo: Color,
    // pub roughness: f64,
    // pub opacity: f64,
    pub metallicity: f64,
    // pub color_map: Option<ImageMap>,
    // pub normal_map: Option<ImageMap>,
    // pub roughness_map: Option<ImageMap>,
    // pub metallicity_map: Option<ImageMap>,
}

impl PbrMaterial {
    pub fn new(albedo: Color, metallicity: f64) -> Self {
        Self {
            albedo,
            metallicity,
        }
    }

    #[allow(unused)]
    fn diffuse_scatter(&self, in_dir: &Vector3<f64>, normal: &Vector3<f64>) -> Vector3<f64> {
        let diffuse_scale = 1.0 - self.metallicity;
        if diffuse_scale > 0.0 {
            normal + rand_on_hemisphere(normal)
        } else {
            Vector3::zeros()
        }
    }

    fn metallic_scatter(&self, in_dir: &Vector3<f64>, normal: &Vector3<f64>) -> Vector3<f64> {
        if self.metallicity > 0.0 {
            reflect(&in_dir, &normal).normalize()
        } else {
            Vector3::zeros()
        }
    }

    #[allow(unused)]
    pub fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        // Diffuse scatter
        let mut scatter_dir = self.diffuse_scatter(&r_in.dir, &rec.normal)
            + self.metallic_scatter(&r_in.dir, &rec.normal);

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

fn all_are_less_than(vec: Vector3<f64>, limit: f64) -> bool {
    (vec.x.abs() < limit) && (vec.y.abs() < limit) && (vec.z.abs() < limit)
}
