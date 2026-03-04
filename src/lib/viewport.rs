use crate::lib::vec3;
use crate::lib::camera;

pub struct Viewport {
    width: f64,
    height: f64,
    pub viewport_u: vec3::Vec3,
    pub viewport_v: vec3::Vec3,
    pub w: vec3::Vec3,
    img_width: u32,
    img_height: u32,
}

impl Viewport {
    pub fn delta_u(&self) -> vec3::Vec3 {
        return self.viewport_u.scale(1.0 / self.img_width as f64);
    }

    pub fn delta_v(&self) -> vec3::Vec3 {
        return self.viewport_v.scale(1.0 / self.img_height as f64);
    }

    pub fn upper_left(&self, camera: &camera::Camera) -> vec3::Vec3 {
        return camera.center - self.w.scale(camera.options.focus_dist) - self.viewport_u.scale(0.5) - self.viewport_v.scale(0.5);
    }

    pub fn pixel00_loc(&self, camera: &camera::Camera) -> vec3::Vec3 {
        return self.upper_left(camera) + (self.delta_u() + self.delta_v()).scale(0.5);
    }

    pub fn pixel_loc(&self, camera: &camera::Camera, x_idx: u32, y_idx: u32) -> vec3::Vec3 {
        return self.pixel00_loc(camera) + self.delta_u().scale(x_idx as f64) + self.delta_v().scale(y_idx as f64);
    }
}

pub fn new(img_width: u32, img_height: u32, viewport_width: f64, viewport_height: f64, u: vec3::Vec3, v: vec3::Vec3, w: vec3::Vec3) -> Viewport {
    return Viewport {
        width: viewport_width,
        height: viewport_height,
        viewport_u: u.scale(viewport_width as f64),
        viewport_v: -v.scale(viewport_height as f64),
        w: w,
        img_width: img_width,
        img_height: img_height,
    };
}
