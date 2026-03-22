use glam::{
    Mat4,
    Vec3,
    Vec3A,
};

use crate::rt::types::{Float, Point, Vector, to_parry_vec};

#[derive(Clone, Copy)]
pub struct Aabb {
    pub min: Vector,
    pub max: Vector,
}

impl Aabb {
    pub fn new(min: Vector, max: Vector) -> Self {
        let mins = Vector::new(
            Float::min(min.x, max.x),
            Float::min(min.y, max.y),
            Float::min(min.z, max.z),
        );
        let maxs = Vector::new(
            Float::max(min.x, max.x),
            Float::max(min.y, max.y),
            Float::max(min.z, max.z),
        );
        let margin = Vector::splat(0.1); // TODO - what's a good choice here?
        Self { min: mins - margin, max: maxs + margin }
    }

    pub fn from_points(points: Vec<Point>) -> Self {
        if points.is_empty() {
            return Self::new(Vector::splat(0.0), Vector::splat(0.0));
        }

        let mut min = points[0];
        let mut max = points[0];
        for point in points[1..points.len()].iter() {
            min = min.min(*point);
            max = max.max(*point);
        }

        Self::new(min, max)
    }

    pub fn transform(&self, transform: Mat4) -> Self {
        let corners = [
            Vec3::new(self.min.x, self.min.y, self.min.z),
            Vec3::new(self.max.x, self.min.y, self.min.z),
            Vec3::new(self.min.x, self.max.y, self.min.z),
            Vec3::new(self.max.x, self.max.y, self.min.z),
            Vec3::new(self.min.x, self.min.y, self.max.z),
            Vec3::new(self.max.x, self.min.y, self.max.z),
            Vec3::new(self.min.x, self.max.y, self.max.z),
            Vec3::new(self.max.x, self.max.y, self.max.z),
        ];

        // Transform each corner and accumulate min/max
        let mut new_min = Vec3::splat(f32::INFINITY);
        let mut new_max = Vec3::splat(f32::NEG_INFINITY);

        for corner in &corners {
            let transformed = transform.transform_point3(*corner);
            new_min = new_min.min(transformed);
            new_max = new_max.max(transformed);
        }

        Self::new(new_min, new_max)
    }

    pub fn to_obvhs(&self) -> obvhs::aabb::Aabb {
        obvhs::aabb::Aabb::new(Vec3A::from(self.min), Vec3A::from(self.max))
    }

    pub fn to_parry3d(&self) -> parry3d_f64::bounding_volume::Aabb {
        parry3d_f64::bounding_volume::Aabb::new(
            to_parry_vec(self.min), 
            to_parry_vec(self.max)
        )
    }
}
