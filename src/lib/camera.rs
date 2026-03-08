use core::f64;

use crate::lib::util::degrees_to_radians;
use crate::lib::vec3::Vec3;

pub struct Disk {
    pub u: Vec3,
    pub v: Vec3,
}

pub struct Camera {
    pub position: Vec3,
    pub aspect_ratio: f64,
    pub vfov: f64,
    pub focus_dist: f64,
    pub defocus_angle: f64,
    pub defocus_disk: Disk,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
}

impl Camera {
    pub fn new(options: &CameraOptions) -> Camera {
        let w = (options.lookfrom - options.lookat).unit();
        let u = options.vup.cross(&w).unit();
        let v = w.cross(&u);
        let defocus_radius =
            options.focus_dist * degrees_to_radians(options.defocus_angle / 2.0).tan();

        Camera {
            position: options.lookfrom,
            aspect_ratio: options.aspect_ratio,
            vfov: options.vfov,
            focus_dist: options.focus_dist,
            defocus_angle: options.defocus_angle,
            defocus_disk: Disk {
                u: u * defocus_radius,
                v: v * defocus_radius,
            },
            u,
            v,
            w,
        }
    }
}

#[derive(Clone, Copy)]
pub struct CameraOptions {
    pub aspect_ratio: f64,
    pub vfov: f64,
    pub lookfrom: Vec3,
    pub lookat: Vec3,
    pub vup: Vec3,
    pub defocus_angle: f64,
    pub focus_dist: f64,
}

impl CameraOptions {
    pub fn new() -> Self {
        Self {
            aspect_ratio: 16.0 / 9.0,
            vfov: 20.0,
            lookfrom: Vec3::new(0.0, 1.0, 0.0),
            lookat: Vec3::new(0.0, 0.0, 0.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_dist: 1.0,
        }
    }

    pub fn aspect_ratio(mut self, new_aspect_ratio: f64) -> Self {
        self.aspect_ratio = new_aspect_ratio;
        self
    }

    pub fn vfov(mut self, new_vfov: f64) -> Self {
        self.vfov = new_vfov;
        self
    }

    pub fn lookfrom(mut self, new_lookfrom: [f64; 3]) -> Self {
        self.lookfrom = Vec3::new_arr(new_lookfrom);
        self
    }

    pub fn lookat(mut self, new_lookat: [f64; 3]) -> Self {
        self.lookat = Vec3::new_arr(new_lookat);
        self
    }

    pub fn vup(mut self, new_vup: [f64; 3]) -> Self {
        self.vup = Vec3::new_arr(new_vup);
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
