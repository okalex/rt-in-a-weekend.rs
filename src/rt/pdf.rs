use std::{f64::consts::PI, sync::Arc};

use nalgebra::{Point3, Vector3};

use crate::rt::{
    objects::hittable_list::HittableList,
    onb::Onb,
    random::{rand, rand_cos_dir, rand_on_hemisphere, rand_unit_vector},
};

#[allow(unused)]
pub enum Pdf {
    Sphere(SpherePdf),
    Hemisphere(HemispherePdf),
    Cosine(CosinePdf),
    Hittable(HittablePdf),
    Mixture(MixturePdf),
}

impl Pdf {
    pub fn value(&self, direction: &Vector3<f64>) -> f64 {
        match self {
            Self::Sphere(pdf) => pdf.value(direction),
            Self::Hemisphere(pdf) => pdf.value(direction),
            Self::Cosine(pdf) => pdf.value(direction),
            Self::Hittable(pdf) => pdf.value(direction),
            Self::Mixture(pdf) => pdf.value(direction),
        }
    }

    pub fn generate(&self) -> Vector3<f64> {
        match self {
            Self::Sphere(pdf) => pdf.generate(),
            Self::Hemisphere(pdf) => pdf.generate(),
            Self::Cosine(pdf) => pdf.generate(),
            Self::Hittable(pdf) => pdf.generate(),
            Self::Mixture(pdf) => pdf.generate(),
        }
    }
}

pub struct SpherePdf {}

impl SpherePdf {
    pub fn new() -> Self {
        Self {}
    }

    #[allow(unused)]
    fn value(&self, direction: &Vector3<f64>) -> f64 {
        1.0 / (4.0 * PI)
    }

    fn generate(&self) -> Vector3<f64> {
        rand_unit_vector()
    }
}

pub struct HemispherePdf {
    normal: Vector3<f64>,
}

impl HemispherePdf {
    #[allow(dead_code)]
    pub fn new(normal: Vector3<f64>) -> Self {
        Self { normal }
    }

    #[allow(unused)]
    fn value(&self, direction: &Vector3<f64>) -> f64 {
        1.0 / (2.0 * PI)
    }

    fn generate(&self) -> Vector3<f64> {
        rand_on_hemisphere(&self.normal)
    }
}

pub struct CosinePdf {
    uvw: Onb,
}

impl CosinePdf {
    pub fn new(w: &Vector3<f64>) -> Self {
        Self { uvw: Onb::new(w) }
    }

    fn value(&self, direction: &Vector3<f64>) -> f64 {
        let cos_theta = direction.normalize().dot(&self.uvw.w());
        f64::max(0.0, cos_theta / PI)
    }

    fn generate(&self) -> Vector3<f64> {
        self.uvw.transform(rand_cos_dir())
    }
}

pub struct HittablePdf {
    object: Arc<HittableList>,
    orig: Point3<f64>,
}

impl HittablePdf {
    pub fn new(object: Arc<HittableList>, orig: Point3<f64>) -> Self {
        Self { object, orig }
    }

    fn value(&self, direction: &Vector3<f64>) -> f64 {
        self.object.pdf_value(&self.orig, direction)
    }

    fn generate(&self) -> Vector3<f64> {
        self.object.random(&self.orig)
    }
}

pub struct MixturePdf {
    p0: Arc<Pdf>,
    p1: Arc<Pdf>,
}

impl MixturePdf {
    pub fn new(p0: Arc<Pdf>, p1: Arc<Pdf>) -> Self {
        Self { p0, p1 }
    }

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
