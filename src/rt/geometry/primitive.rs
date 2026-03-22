use crate::rt::{
    geometry::{
        quad::Quad,
        scene::{MeshDescriptor, MeshId},
        sphere::Sphere,
        triangle::Triangle,
    },
    types::{
        Float,
        Point,
        Uint,
        Vector,
    },
};

pub enum Primitive {
    Sphere(Sphere),
    Quad(Quad),
    Triangle(Triangle),
    Mesh(MeshDescriptor),
}

impl Primitive {
    pub fn sphere(center: Point, radius: Float) -> Primitive {
        Self::Sphere(Sphere::stationary(center, radius))
    }

    pub fn triangle(a: Point, b: Point, c: Point) -> Primitive {
        Self::Triangle(Triangle::new(a, b, c))
    }

    pub fn quad(q: Point, u: Vector, v: Vector) -> Primitive {
        Self::Quad(Quad::new(q, u, v))
    }

    pub fn mesh(id: MeshId, triangle_count: Uint) -> Primitive {
        Self::Mesh(MeshDescriptor { id, triangle_count })
    }
}
