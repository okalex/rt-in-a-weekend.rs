use core::f64;

use nalgebra::{Point3, Vector3};

#[derive(Clone, Copy)]
pub struct Camera {
    pub position: Point3<f64>,
    pub target: Point3<f64>,
    pub vup: Vector3<f64>,
    pub vfov: f64,
    pub focus_dist: f64,
    pub defocus_angle: f64,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            position: Point3::new(0.0, 1.0, 0.0),
            target: Point3::new(0.0, 0.0, 0.0),
            vup: Vector3::new(0.0, 1.0, 0.0),
            vfov: 20.0,
            defocus_angle: 0.0,
            focus_dist: 1.0,
        }
    }

    pub fn vfov(mut self, new_vfov: f64) -> Self {
        self.vfov = new_vfov;
        self
    }

    pub fn position(mut self, new_position: [f64; 3]) -> Self {
        self.position = Point3::from(new_position);
        self
    }

    pub fn target(mut self, new_target: [f64; 3]) -> Self {
        self.target = Point3::from(new_target);
        self
    }

    pub fn vup(mut self, new_vup: [f64; 3]) -> Self {
        self.vup = Vector3::from(new_vup);
        self
    }

    pub fn defocus_angle(mut self, new_defocus_angle: f64) -> Self {
        self.defocus_angle = new_defocus_angle;
        self
    }

    pub fn focus_dist(mut self, new_focus_dist: f64) -> Self {
        self.focus_dist = new_focus_dist;
        self
    }
}
