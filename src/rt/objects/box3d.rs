use nalgebra::{Point3, Vector3};
use nalgebra_glm::{max2, min2};

use crate::rt::objects::{hittable::Hittable, hittable_list::HittableList, quad::Quad};

pub struct Box3d {}

impl Box3d {
    pub fn new(a: Vector3<f64>, b: Vector3<f64>, mat_idx: usize) -> HittableList {
        let min = min2(&a, &b);
        let max = max2(&a, &b);

        let dx = Vector3::new(max.x - min.x, 0.0, 0.0);
        let dy = Vector3::new(0.0, max.y - min.y, 0.0);
        let dz = Vector3::new(0.0, 0.0, max.z - min.z);

        let quad1 = Hittable::Quad(Quad::new(Point3::new(min.x, min.y, max.z), dx, dy, mat_idx));
        let quad2 = Hittable::Quad(Quad::new(
            Point3::new(max.x, min.y, max.z),
            -dz,
            dy,
            mat_idx,
        ));
        let quad3 = Hittable::Quad(Quad::new(
            Point3::new(max.x, min.y, min.z),
            -dx,
            dy,
            mat_idx,
        ));
        let quad4 = Hittable::Quad(Quad::new(Point3::new(min.x, min.y, min.z), dz, dy, mat_idx));
        let quad5 = Hittable::Quad(Quad::new(
            Point3::new(min.x, max.y, max.z),
            dx,
            -dz,
            mat_idx,
        ));
        let quad6 = Hittable::Quad(Quad::new(Point3::new(min.x, min.y, min.z), dx, dz, mat_idx));

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
