use crate::lib::camera::{Camera, CameraOptions};
use crate::lib::util::degrees_to_radians;
use crate::lib::vec3::Vec3;

pub struct Viewport {
    width: f64,
    height: f64,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub delta_u: Vec3,
    pub delta_v: Vec3,
    focus_dist: f64,
}

impl Viewport {
    pub fn new(img_width: u32, img_height: u32, camera: &Camera) -> Viewport {
        let theta = degrees_to_radians(camera.vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * camera.focus_dist;
        let viewport_width = viewport_height * (img_width as f64) / (img_height as f64);

        let viewport_u = camera.u * (viewport_width as f64);
        let viewport_v = -camera.v * (viewport_height as f64);
        Viewport {
            width: viewport_width,
            height: viewport_height,
            u: viewport_u,
            v: viewport_v,
            w: camera.w,
            delta_u: viewport_u / (img_width as f64),
            delta_v: viewport_v / (img_height as f64),
            focus_dist: camera.focus_dist,
        }
    }

    pub fn upper_left(&self, camera: &Camera) -> Vec3 {
        return camera.position - self.w * camera.focus_dist - self.u / 2.0 - self.v / 2.0;
    }

    pub fn pixel00_loc(&self, camera: &Camera) -> Vec3 {
        return self.upper_left(camera) + (self.delta_u + self.delta_v) / 2.0;
    }

    pub fn pixel_loc(&self, camera: &Camera, x_idx: u32, y_idx: u32) -> Vec3 {
        return self.pixel00_loc(camera)
            + self.delta_u * (x_idx as f64)
            + self.delta_v * (y_idx as f64);
    }
}
