use std::sync::Arc;

use crate::lib::color::Color;
use crate::lib::hittable::HitRecord;
use crate::lib::random::rand;
use crate::lib::ray::Ray;
use crate::lib::texture::{SolidColor, Texture};
use crate::lib::vec3::Vec3;

pub struct Scattered {
    pub ray: Ray,
    pub attenuation: Color,
}

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scattered> {
        None
    }

    fn emitted(&self, u: f64, v: f64, point: &Vec3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
}

pub struct Lambertian {
    texture: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(texture: Arc<dyn Texture>) -> Self {
        Self { texture }
    }

    pub fn from_color(color: Color) -> Self {
        let arc_color: Arc<dyn Texture> = Arc::new(SolidColor::new(color));
        Self::new(arc_color)
    }

    pub fn from_color_values(color_values: [f64; 3]) -> Self {
        Self::from_color(Color::from_arr(color_values))
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scattered> {
        let mut scatter_dir = rec.normal + Vec3::rand_unit();
        if scatter_dir.all_are_less_than(1e-8) {
            scatter_dir = rec.normal;
        }

        Some(Scattered {
            ray: Ray::new(rec.point, scatter_dir, r_in.time),
            attenuation: self.texture.value(rec.u, rec.v, &rec.point),
        })
    }
}

impl Clone for Lambertian {
    fn clone(&self) -> Self {
        Self {
            texture: Arc::clone(&self.texture),
        }
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: [f64; 3], fuzz: f64) -> Self {
        return Self {
            albedo: Color::from_arr(albedo),
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        };
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scattered> {
        let reflected = reflect(&r_in.dir, &rec.normal).unit() + Vec3::rand_unit() * self.fuzz;

        Some(Scattered {
            ray: Ray::new(rec.point, reflected, r_in.time),
            attenuation: self.albedo,
        })
    }
}

pub struct Dielectric {
    refraction_idx: f64,
}

impl Dielectric {
    pub fn new(refraction_idx: f64) -> Self {
        Self {
            refraction_idx: refraction_idx,
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scattered> {
        let ri = if rec.front_face {
            1.0 / self.refraction_idx
        } else {
            self.refraction_idx
        };

        let unit_dir = r_in.dir.unit();
        let cos_theta = f64::min((-unit_dir).dot(&rec.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = (ri * sin_theta) > 1.0;
        let direction = if cannot_refract || reflectance(cos_theta, ri) > rand() {
            reflect(&unit_dir, &rec.normal)
        } else {
            refract(&unit_dir, &rec.normal, ri)
        };

        Some(Scattered {
            ray: Ray::new(rec.point, direction, r_in.time),
            attenuation: Color::white(),
        })
    }
}

fn reflectance(cosine: f64, refraction_idx: f64) -> f64 {
    let r0_tmp = (1.0 - refraction_idx) / (1.0 + refraction_idx);
    let r0 = r0_tmp * r0_tmp;
    return r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0);
}

fn reflect(vec: &Vec3, normal: &Vec3) -> Vec3 {
    *vec - *normal * vec.dot(normal) * 2.0
}

fn refract(vec: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = f64::min(-vec.dot(n), 1.0);
    let r_out_perp = (*vec + *n * cos_theta) * etai_over_etat;
    let r_out_parallel = *n * (-(1.0 - r_out_perp.length_squared()).abs().sqrt());
    return r_out_perp + r_out_parallel;
}

pub struct DiffuseLight {
    texture: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(texture: Arc<dyn Texture>) -> Self {
        Self { texture }
    }

    pub fn from_color(color: Color) -> Self {
        let arc: Arc<dyn Texture> = Arc::new(SolidColor::new(color));
        Self::new(Arc::clone(&arc))
    }
}

impl Material for DiffuseLight {
    fn emitted(&self, u: f64, v: f64, point: &Vec3) -> Color {
        self.texture.value(u, v, point)
    }
}

impl Clone for DiffuseLight {
    fn clone(&self) -> Self {
        Self {
            texture: Arc::clone(&self.texture),
        }
    }
}

pub struct Isotropic {
    texture: Arc<dyn Texture>,
}

impl Isotropic {
    pub fn new(texture: Arc<dyn Texture>) -> Self {
        Self {
            texture: Arc::clone(&texture),
        }
    }

    pub fn from_color(albedo: Color) -> Self {
        let texture: Arc<dyn Texture> = Arc::new(SolidColor::new(albedo));
        Self::new(texture)
    }
}

impl Material for Isotropic {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scattered> {
        let scattered = Ray::new(rec.point, Vec3::rand_unit(), r_in.time);
        let attenuation = self.texture.value(rec.u, rec.v, &rec.point);
        Some(Scattered {
            ray: scattered,
            attenuation,
        })
    }
}
