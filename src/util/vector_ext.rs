use crate::util::types::{Float, Vector};

pub const HALF_VECTOR_EPSILON: Float = 1e-12;

pub struct VectorExt;

impl VectorExt {
    pub fn lerp(a: Vector, b: Vector, factor: Float) -> Vector {
        factor * a + (1.0 - factor) * b
    }

    pub fn normalize_if_valid(vector: Vector) -> Option<Vector> {
        let length_squared = vector.length_squared();
        if !length_squared.is_finite() || length_squared <= HALF_VECTOR_EPSILON {
            return None;
        }

        Some(vector / length_squared.sqrt())
    }

    pub fn half_vector(a: Vector, b: Vector) -> Option<Vector> {
        Self::normalize_if_valid(a + b)
    }

    pub fn reflect(vector: Vector, normal: Vector) -> Vector {
        2.0 * vector.dot(normal) * normal - vector
    }
}
