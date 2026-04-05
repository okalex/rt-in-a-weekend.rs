use std::sync::Arc;

use glam::Mat4;

use crate::{
    rt::{
        geometry::primitive::Primitive,
        materials::ggx::{ggx_d, smith_g1},
        onb::Onb,
    },
    util::{
        random::{rand, rand_cos_dir, rand_int, rand_on_hemisphere, rand_on_unit_disk, rand_unit_vector},
        types::{Float, Int, PI, Point, Vector},
        vector_ext::VectorExt,
    },
};

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
    #[allow(unused)]
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
        let Some(incoming_dir) = VectorExt::normalize_if_valid(*direction) else {
            return 0.0;
        };
        let Some(half_vector) = VectorExt::half_vector(self.view_dir, incoming_dir) else {
            return 0.0;
        };

        let n_dot_h = self.normal.dot(half_vector);
        let n_dot_i = self.normal.dot(incoming_dir);
        let n_dot_v = self.normal.dot(self.view_dir);

        if n_dot_h <= 0.0 || n_dot_i <= 0.0 || n_dot_v <= 0.0 {
            return 0.0;
        }

        Self::sanitize_pdf(self.visible_microfacet_pdf(n_dot_h, n_dot_v))
    }

    pub fn generate(&self) -> Vector {
        let shading_basis = Onb::new(&self.normal);
        let local_view_dir = shading_basis.inv_transform(self.view_dir);
        let stretched_view_dir = Self::stretch_view_dir(local_view_dir, self.alpha);
        let disk_sample = Self::sample_visible_normal_disk(stretched_view_dir);
        let (tangent, bitangent) = Self::visible_normal_frame(stretched_view_dir);
        let stretched_half_vector = Self::stretched_half_vector(disk_sample, tangent, bitangent, stretched_view_dir);
        let local_half_vector = Self::unstretch_half_vector(stretched_half_vector, self.alpha);
        let world_half_vector = shading_basis.transform(local_half_vector);

        VectorExt::reflect(self.view_dir, world_half_vector)
    }

    fn visible_microfacet_pdf(&self, n_dot_h: Float, n_dot_v: Float) -> Float {
        let alpha_sqrd = self.alpha * self.alpha;
        let normal_distribution = ggx_d(n_dot_h, alpha_sqrd);
        let masking = smith_g1(n_dot_v, alpha_sqrd);
        normal_distribution * masking / (4.0 * n_dot_v)
    }

    fn sanitize_pdf(pdf: Float) -> Float {
        if pdf.is_finite() && pdf > 0.0 { pdf } else { 0.0 }
    }

    fn stretch_view_dir(local_view_dir: Vector, alpha: Float) -> Vector {
        VectorExt::normalize_if_valid(Vector::new(
            alpha * local_view_dir.x,
            alpha * local_view_dir.y,
            local_view_dir.z,
        ))
        .unwrap_or(Vector::Z)
    }

    fn sample_visible_normal_disk(stretched_view_dir: Vector) -> Vector {
        let mut disk_sample = rand_on_unit_disk();
        let blend = 0.5 * (1.0 + stretched_view_dir.z);
        disk_sample.y = (1.0 - blend) * (1.0 - disk_sample.x * disk_sample.x).sqrt() + blend * disk_sample.y;
        disk_sample.z = Float::max(0.0, 1.0 - disk_sample.x * disk_sample.x - disk_sample.y * disk_sample.y).sqrt();
        disk_sample
    }

    fn visible_normal_frame(stretched_view_dir: Vector) -> (Vector, Vector) {
        let tangent = if stretched_view_dir.z.abs() < 0.999 {
            Vector::Z.cross(stretched_view_dir).normalize()
        } else {
            Vector::X.cross(stretched_view_dir).normalize()
        };
        let bitangent = stretched_view_dir.cross(tangent);

        (tangent, bitangent)
    }

    fn stretched_half_vector(
        disk_sample: Vector,
        tangent: Vector,
        bitangent: Vector,
        stretched_view_dir: Vector,
    ) -> Vector {
        disk_sample.x * tangent + disk_sample.y * bitangent + disk_sample.z * stretched_view_dir
    }

    fn unstretch_half_vector(stretched_half_vector: Vector, alpha: Float) -> Vector {
        VectorExt::normalize_if_valid(Vector::new(
            alpha * stretched_half_vector.x,
            alpha * stretched_half_vector.y,
            Float::max(stretched_half_vector.z, 0.0),
        ))
        .unwrap_or(Vector::Z)
    }
}
