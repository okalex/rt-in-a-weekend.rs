use crate::lib::camera::Camera;
use crate::lib::vec3::Vec3;

pub struct Viewport {
    width: f64,
    height: f64,
    pub viewport_u: Vec3,
    pub viewport_v: Vec3,
    pub w: Vec3,
    img_width: u32,
    img_height: u32,
}

impl Viewport {
    pub fn new(
        img_width: u32,
        img_height: u32,
        viewport_width: f64,
        viewport_height: f64,
        u: Vec3,
        v: Vec3,
        w: Vec3,
    ) -> Viewport {
        Viewport {
            width: viewport_width,
            height: viewport_height,
            viewport_u: u * (viewport_width as f64),
            viewport_v: -v * (viewport_height as f64),
            w: w,
            img_width: img_width,
            img_height: img_height,
        }
    }

    pub fn delta_u(&self) -> Vec3 {
        return self.viewport_u / (self.img_width as f64);
    }

    pub fn delta_v(&self) -> Vec3 {
        return self.viewport_v / (self.img_height as f64);
    }

    pub fn upper_left(&self, camera: &Camera) -> Vec3 {
        return camera.center
            - self.w * camera.focus_dist()
            - self.viewport_u / 2.0
            - self.viewport_v / 2.0;
    }

    pub fn pixel00_loc(&self, camera: &Camera) -> Vec3 {
        return self.upper_left(camera) + (self.delta_u() + self.delta_v()) / 2.0;
    }

    pub fn pixel_loc(&self, camera: &Camera, x_idx: u32, y_idx: u32) -> Vec3 {
        return self.pixel00_loc(camera)
            + self.delta_u() * (x_idx as f64)
            + self.delta_v() * (y_idx as f64);
    }
}
