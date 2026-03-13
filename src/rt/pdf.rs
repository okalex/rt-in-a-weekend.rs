use std::{f64::consts::PI, sync::Arc};

use nalgebra::{Point3, Vector3};

use crate::rt::{
    objects::hittable::Hittable,
    onb::Onb,
    random::{rand, rand_cos_dir, rand_unit_vector},
};

pub trait Pdf {
    fn value(&self, direction: &Vector3<f64>) -> f64;
    fn generate(&self) -> Vector3<f64>;
}

pub struct SpherePdf {}

impl SpherePdf {
    pub fn new() -> Self {
        Self {}
    }
}

impl Pdf for SpherePdf {
    #[allow(unused)]
    fn value(&self, direction: &Vector3<f64>) -> f64 {
        1.0 / (4.0 * PI)
    }

    fn generate(&self) -> Vector3<f64> {
        rand_unit_vector()
    }
}

pub struct CosinePdf {
    uvw: Onb,
}

impl CosinePdf {
    pub fn new(w: &Vector3<f64>) -> Self {
        Self { uvw: Onb::new(w) }
    }
}

impl Pdf for CosinePdf {
    fn value(&self, direction: &Vector3<f64>) -> f64 {
        let cos_theta = direction.normalize().dot(&self.uvw.w());
        f64::max(0.0, cos_theta / PI)
    }

    fn generate(&self) -> Vector3<f64> {
        self.uvw.transform(rand_cos_dir())
    }
}

pub struct HittablePdf {
    scene: Arc<dyn Hittable>,
    orig: Point3<f64>,
}

impl HittablePdf {
    pub fn new(scene: Arc<dyn Hittable>, orig: Point3<f64>) -> Self {
        Self { scene, orig }
    }
}

impl Pdf for HittablePdf {
    fn value(&self, direction: &Vector3<f64>) -> f64 {
        self.scene.pdf_value(&self.orig, direction)
    }

    fn generate(&self) -> Vector3<f64> {
        self.scene.random(&self.orig)
    }
}

pub struct MixturePdf {
    p0: Arc<dyn Pdf>,
    p1: Arc<dyn Pdf>,
}

impl MixturePdf {
    pub fn new(p0: Arc<dyn Pdf>, p1: Arc<dyn Pdf>) -> Self {
        Self { p0, p1 }
    }
}

impl Pdf for MixturePdf {
    fn value(&self, direction: &Vector3<f64>) -> f64 {
        0.5 * self.p0.value(direction) + 0.5 * self.p1.value(direction)
    }

    fn generate(&self) -> Vector3<f64> {
        if rand() < 0.5 {
            self.p0.generate()
        } else {
            self.p1.generate()
        }
    }
}
