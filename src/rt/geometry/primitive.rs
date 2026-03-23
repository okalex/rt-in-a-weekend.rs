use crate::{
    rt::geometry::{
        quad::Quad,
        scene::{
            MeshDescriptor,
            MeshId,
        },
        sphere::Sphere,
        triangle::Triangle,
    },
    util::types::{
        Float,
        Point,
        Vector,
    },
};

#[derive(Clone)]
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

    pub fn mesh(id: MeshId) -> Primitive {
        Self::Mesh(MeshDescriptor { id })
    }

    pub fn pdf_value(&self, origin: &Point, direction: &Vector) -> Float {
        let default = 0.0;
        match self {
            // Self::ConstantMedium(_) => default,
            Self::Mesh(_) => default, // TODO
            Self::Quad(obj) => obj.pdf_value(origin, direction),
            Self::Sphere(obj) => obj.pdf_value(origin, direction),
            Self::Triangle(_) => default, // TODO
        }
    }

    pub fn random(&self, origin: &Point) -> Vector {
        let default = Vector::new(1.0, 0.0, 0.0);
        match self {
            // Self::ConstantMedium(_) => default,
            Self::Mesh(_) => default, // TODO
            Self::Quad(obj) => obj.random(origin),
            Self::Sphere(obj) => obj.random(origin),
            Self::Triangle(_) => default, // TODO
        }
    }
}
