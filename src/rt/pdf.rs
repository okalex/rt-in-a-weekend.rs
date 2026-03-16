use std::sync::Arc;

use crate::rt::{
    objects::hittable_list::HittableList,
    onb::Onb,
    random::{rand, rand_cos_dir, rand_on_hemisphere, rand_unit_vector},
    types::{Float, PI, Point, Vector},
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
    pub fn value(&self, direction: &Vector) -> Float {
        match self {
            Self::Sphere(pdf) => pdf.value(direction),
            Self::Hemisphere(pdf) => pdf.value(direction),
            Self::Cosine(pdf) => pdf.value(direction),
            Self::Hittable(pdf) => pdf.value(direction),
            Self::Mixture(pdf) => pdf.value(direction),
        }
    }

    pub fn generate(&self) -> Vector {
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
    fn value(&self, direction: &Vector) -> Float {
        1.0 / (4.0 * PI)
    }

    fn generate(&self) -> Vector {
        rand_unit_vector()
    }
}

pub struct HemispherePdf {
    normal: Vector,
}

impl HemispherePdf {
    #[allow(dead_code)]
    pub fn new(normal: Vector) -> Self {
        Self { normal }
    }

    #[allow(unused)]
    fn value(&self, direction: &Vector) -> Float {
        1.0 / (2.0 * PI)
    }

    fn generate(&self) -> Vector {
        rand_on_hemisphere(self.normal)
    }
}

pub struct CosinePdf {
    uvw: Onb,
}

impl CosinePdf {
    pub fn new(w: &Vector) -> Self {
        Self { uvw: Onb::new(w) }
    }

    fn value(&self, direction: &Vector) -> Float {
        let cos_theta = direction.normalize().dot(self.uvw.w());
        Float::max(0.0, cos_theta / PI)
    }

    fn generate(&self) -> Vector {
        self.uvw.transform(rand_cos_dir())
    }
}

pub struct HittablePdf {
    object: Arc<HittableList>,
    orig: Point,
}

impl HittablePdf {
    pub fn new(object: Arc<HittableList>, orig: Point) -> Self {
        Self { object, orig }
    }

    fn value(&self, direction: &Vector) -> Float {
        self.object.pdf_value(&self.orig, direction)
    }

    fn generate(&self) -> Vector {
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

    fn value(&self, direction: &Vector) -> Float {
        0.5 * self.p0.value(direction) + 0.5 * self.p1.value(direction)
    }

    fn generate(&self) -> Vector {
        if rand() < 0.5 {
            self.p0.generate()
        } else {
            self.p1.generate()
        }
    }
}
