use glam::{
    Mat4,
    Vec3,
    Vec3A,
};

use crate::rt::types::Vector;

#[derive(Clone, Copy)]
pub struct Aabb {
    pub min: Vector,
    pub max: Vector,
}

impl Aabb {
    pub fn new(min: Vector, max: Vector) -> Self {
        Self { min, max }
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
}
