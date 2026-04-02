use std::sync::Arc;

use glam::Mat4;

use crate::{
    rt::{geometry::primitive::Primitive, materials::pbr_material::PbrMaterial, onb::Onb},
    util::{
        random::{rand, rand_cos_dir, rand_int, rand_on_hemisphere, rand_on_unit_disk, rand_unit_vector},
        types::{Float, Int, PI, Point, Vector},
    },
};

const HALF_VECTOR_EPSILON: Float = 1e-12;

#[allow(unused)]
pub enum Pdf {
    Sphere(SpherePdf),
    Hemisphere(HemispherePdf),
    Cosine(CosinePdf),
    Multi(MultiPdf),
    Mixture(MixturePdf),
    Ggx(GgxPdf),
}

impl Pdf {
    pub fn sphere() -> Self {
        Self::Sphere(SpherePdf::new())
    }

    #[allow(unused)]
    pub fn hemisphere(normal: Vector) -> Self {
        Self::Hemisphere(HemispherePdf::new(normal))
    }

    pub fn cosine(w: &Vector) -> Self {
        Self::Cosine(CosinePdf::new(w))
    }

    pub fn multi(origin: Point, primitives: Vec<TransformedPrimitive>) -> Self {
        Self::Multi(MultiPdf::new(origin, primitives))
    }

    pub fn mixture(p0: Arc<Pdf>, p1: Arc<Pdf>, weight: Float) -> Self {
        Self::Mixture(MixturePdf::new(p0, p1, weight))
    }

    pub fn ggx(view_dir: Vector, normal: Vector, alpha: Float) -> Self {
        Self::Ggx(GgxPdf::new(view_dir, normal, alpha))
    }

    pub fn value(&self, direction: &Vector) -> Float {
        match self {
            Self::Sphere(pdf) => pdf.value(direction),
            Self::Hemisphere(pdf) => pdf.value(direction),
            Self::Cosine(pdf) => pdf.value(direction),
            Self::Multi(pdf) => pdf.value(direction),
            Self::Mixture(pdf) => pdf.value(direction),
            Self::Ggx(pdf) => pdf.value(direction),
        }
    }

    pub fn generate(&self) -> Vector {
        match self {
            Self::Sphere(pdf) => pdf.generate(),
            Self::Hemisphere(pdf) => pdf.generate(),
            Self::Cosine(pdf) => pdf.generate(),
            Self::Multi(pdf) => pdf.generate(),
            Self::Mixture(pdf) => pdf.generate(),
            Self::Ggx(pdf) => pdf.generate(),
        }
    }
}

pub struct SpherePdf;

impl SpherePdf {
    pub fn new() -> Self {
        Self
    }

    #[allow(unused_variables)]
    pub fn value(&self, direction: &Vector) -> Float {
        1.0 / (4.0 * PI)
    }

    pub fn generate(&self) -> Vector {
        rand_unit_vector()
    }
}

pub struct HemispherePdf {
    normal: Vector,
}

impl HemispherePdf {
    pub fn new(normal: Vector) -> Self {
        Self { normal }
    }

    #[allow(unused_variables)]
    pub fn value(&self, direction: &Vector) -> Float {
        1.0 / (2.0 * PI)
    }

    pub fn generate(&self) -> Vector {
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

    pub fn value(&self, direction: &Vector) -> Float {
        let cos_theta = direction.normalize().dot(self.uvw.w());
        Float::max(0.0, cos_theta / PI)
    }

    pub fn generate(&self) -> Vector {
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

    pub fn value(&self, direction: &Vector) -> Float {
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

    pub fn generate(&self) -> Vector {
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
    weight: Float,
}

impl MixturePdf {
    pub fn new(p0: Arc<Pdf>, p1: Arc<Pdf>, weight: Float) -> Self {
        Self { p0, p1, weight }
    }

    pub fn value(&self, direction: &Vector) -> Float {
        self.weight * self.p0.value(direction) + (1.0 - self.weight) * self.p1.value(direction)
    }

    pub fn generate(&self) -> Vector {
        if rand() < self.weight {
            self.p0.generate()
        } else {
            self.p1.generate()
        }
    }
}

pub struct GgxPdf {
    view_dir: Vector,
    normal: Vector,
    alpha: Float,
}

impl GgxPdf {
    pub fn new(view_dir: Vector, normal: Vector, alpha: Float) -> Self {
        Self {
            view_dir: view_dir.normalize(),
            normal: normal.normalize(),
            alpha,
        }
    }

    pub fn value(&self, direction: &Vector) -> Float {
        let direction_len_sq = direction.length_squared();
        if !direction_len_sq.is_finite() || direction_len_sq <= HALF_VECTOR_EPSILON {
            return 0.0;
        }

        let wi = *direction / direction_len_sq.sqrt();
        let half_vec = self.view_dir + wi;
        let half_len_sq = half_vec.length_squared();
        if !half_len_sq.is_finite() || half_len_sq <= HALF_VECTOR_EPSILON {
            return 0.0;
        }

        let h = half_vec / half_len_sq.sqrt();
        let n_dot_h = self.normal.dot(h);
        let n_dot_i = self.normal.dot(wi);
        let n_dot_v = self.normal.dot(self.view_dir);

        if n_dot_h <= 0.0 || n_dot_i <= 0.0 || n_dot_v <= 0.0 {
            return 0.0;
        }

        let d_h = PbrMaterial::ggx_d(n_dot_h, self.alpha);
        let g1_o = PbrMaterial::smith_g1(n_dot_v, self.alpha);
        let pdf = d_h * g1_o / (4.0 * n_dot_v);

        if pdf.is_finite() && pdf > 0.0 { pdf } else { 0.0 }
    }

    pub fn generate(&self) -> Vector {
        let onb = Onb::new(&self.normal);
        let v = onb.inv_transform(self.view_dir);
        let v_stretched = Vector::new(self.alpha * v.x, self.alpha * v.y, v.z).normalize();

        let mut r = rand_on_unit_disk();
        let s = 0.5 * (1.0 + v_stretched.z);
        r.y = (1.0 - s) * (1.0 - r.x * r.x).sqrt() + s * r.y;
        r.z = Float::max(0.0, 1.0 - r.x * r.x - r.y * r.y).sqrt();

        let t1 = if v_stretched.z.abs() < 0.999 {
            Vector::new(0.0, 0.0, 1.0).cross(v_stretched).normalize()
        } else {
            Vector::new(1.0, 0.0, 0.0).cross(v_stretched).normalize()
        };
        let t2 = v_stretched.cross(t1);

        let h_stretched = r.x * t1 + r.y * t2 + r.z * v_stretched;
        let h_local = Vector::new(
            self.alpha * h_stretched.x,
            self.alpha * h_stretched.y,
            Float::max(h_stretched.z, 0.0),
        )
        .normalize();
        let h = onb.transform(h_local);

        2.0 * self.view_dir.dot(h) * h - self.view_dir
    }
}
