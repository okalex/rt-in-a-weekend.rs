use std::sync::Arc;

use glam::Mat4;

use crate::{
    rt::{geometry::primitive::Primitive, onb::Onb},
    util::{
        random::{rand, rand_cos_dir, rand_int, rand_on_hemisphere, rand_unit_vector},
        types::{Float, Int, PI, Point, Vector},
    },
};

#[allow(unused)]
pub enum Pdf {
    Sphere(SpherePdf),
    Hemisphere(HemispherePdf),
    Cosine(CosinePdf),
    Multi(MultiPdf),
    Mixture(MixturePdf),
}

impl Pdf {
    pub fn multi(origin: Point, primitives: Vec<TransformedPrimitive>) -> Self {
        Self::Multi(MultiPdf::new(origin, primitives))
    }

    pub fn mixture(p0: Arc<Pdf>, p1: Arc<Pdf>) -> Self {
        Self::Mixture(MixturePdf::new(p0, p1))
    }

    pub fn value(&self, direction: &Vector) -> Float {
        match self {
            Self::Sphere(pdf) => pdf.value(direction),
            Self::Hemisphere(pdf) => pdf.value(direction),
            Self::Cosine(pdf) => pdf.value(direction),
            Self::Multi(pdf) => pdf.value(direction),
            Self::Mixture(pdf) => pdf.value(direction),
        }
    }

    pub fn generate(&self) -> Vector {
        match self {
            Self::Sphere(pdf) => pdf.generate(),
            Self::Hemisphere(pdf) => pdf.generate(),
            Self::Cosine(pdf) => pdf.generate(),
            Self::Multi(pdf) => pdf.generate(),
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

#[derive(Clone)]
pub struct TransformedPrimitive {
    pub primitive: Primitive,
    pub transform: Mat4,
    pub inv_transform: Mat4,
}

pub struct MultiPdf {
    origin: Point,
    primitives: Vec<TransformedPrimitive>,
}

impl MultiPdf {
    pub fn new(origin: Point, primitives: Vec<TransformedPrimitive>) -> Self {
        Self { origin, primitives }
    }

    fn value(&self, direction: &Vector) -> Float {
        let weight = 1.0 / self.primitives.len() as Float;
        let mut sum = 0.0;
        for tp in &self.primitives {
            // Transform origin and direction to the primitive's local space
            let local_origin = tp.inv_transform.transform_point3(self.origin);
            let local_dir = tp.inv_transform.transform_vector3(*direction);
            sum += weight * tp.primitive.pdf_value(&local_origin, &local_dir)
        }

        sum
    }

    fn generate(&self) -> Vector {
        let count = self.primitives.len() as Int;
        let tp = &self.primitives[rand_int(0, count - 1) as usize];
        // Generate direction in local space, then transform to world space
        let local_origin = tp.inv_transform.transform_point3(self.origin);
        let local_dir = tp.primitive.random(&local_origin);
        tp.transform.transform_vector3(local_dir).normalize()
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
        if rand() < 0.5 { self.p0.generate() } else { self.p1.generate() }
    }
}
