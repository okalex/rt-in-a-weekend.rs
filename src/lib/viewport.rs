use nalgebra::{Point3, Vector3};

use crate::lib::camera::Camera;
use crate::lib::util::degrees_to_radians;

pub struct Disk {
    pub u: Vector3<f64>,
    pub v: Vector3<f64>,
}

pub struct Viewport {
    pub delta_u: Vector3<f64>,
    pub delta_v: Vector3<f64>,
    upper_left: Point3<f64>,
    pub defocus_disk: Disk,
}

impl Viewport {
    pub fn new(img_width: u32, img_height: u32, camera: &Camera) -> Viewport {
        let theta = degrees_to_radians(camera.vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * camera.focus_dist;
        let viewport_width = viewport_height * (img_width as f64) / (img_height as f64);

        let w = (camera.position - camera.target).normalize();
        let u = camera.vup.cross(&w).normalize();
        let v = w.cross(&u);
        let defocus_radius =
            camera.focus_dist * degrees_to_radians(camera.defocus_angle / 2.0).tan();

        let viewport_u = u * (viewport_width as f64);
        let viewport_v = -v * (viewport_height as f64);
        let upper_left = Point3::from(
            camera.position.coords - viewport_u / 2.0 - viewport_v / 2.0 - w * camera.focus_dist,
        );

        Viewport {
            delta_u: viewport_u / (img_width as f64),
            delta_v: viewport_v / (img_height as f64),
            upper_left,
            defocus_disk: Disk {
                u: u * defocus_radius,
                v: v * defocus_radius,
            },
        }
    }

    pub fn pixel00_loc(&self) -> Point3<f64> {
        return self.upper_left + (self.delta_u + self.delta_v) / 2.0;
    }

    pub fn pixel_loc(&self, x_idx: f64, y_idx: f64) -> Point3<f64> {
        return self.pixel00_loc() + self.delta_u * x_idx + self.delta_v * y_idx;
    }
}
