use crate::rt::{
    objects::{hittable::Hittable, hittable_list::HittableList, quad::Quad},
    types::{Point, Vector},
};

pub struct Box3d {}

impl Box3d {
    pub fn new(a: Vector, b: Vector, mat_idx: usize) -> HittableList {
        let min = a.min(b);
        let max = a.max(b);

        let dx = Vector::new(max.x - min.x, 0.0, 0.0);
        let dy = Vector::new(0.0, max.y - min.y, 0.0);
        let dz = Vector::new(0.0, 0.0, max.z - min.z);

        let quad1 = Hittable::Quad(Quad::new(Point::new(min.x, min.y, max.z), dx, dy, mat_idx));
        let quad2 = Hittable::Quad(Quad::new(Point::new(max.x, min.y, max.z), -dz, dy, mat_idx));
        let quad3 = Hittable::Quad(Quad::new(Point::new(max.x, min.y, min.z), -dx, dy, mat_idx));
        let quad4 = Hittable::Quad(Quad::new(Point::new(min.x, min.y, min.z), dz, dy, mat_idx));
        let quad5 = Hittable::Quad(Quad::new(Point::new(min.x, max.y, max.z), dx, -dz, mat_idx));
        let quad6 = Hittable::Quad(Quad::new(Point::new(min.x, min.y, min.z), dx, dz, mat_idx));

        let mut scene = HittableList::new();
        scene.add(quad1);
        scene.add(quad2);
        scene.add(quad3);
        scene.add(quad4);
        scene.add(quad5);
        scene.add(quad6);
        scene
    }
}
