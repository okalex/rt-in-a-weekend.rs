use crate::{
    rt::camera::CameraOptions,
    util::{
        trig::degrees_to_radians,
        types::{Float, Point, Uint, Vector},
    },
};

pub struct Viewport {
    pub delta_u: Vector,
    pub delta_v: Vector,
    pub u: Vector,
    pub v: Vector,
    pub pixel00_loc: Point,
}

impl Viewport {
    pub fn new(img_width: Uint, img_height: Uint, camera_options: &CameraOptions) -> Viewport {
        let theta = degrees_to_radians(camera_options.vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * camera_options.focus_dist;
        let viewport_width = viewport_height * (img_width as Float) / (img_height as Float);

        let w = (camera_options.position - camera_options.target).normalize();
        let u = camera_options.vup.cross(w).normalize();
        let v = w.cross(u);

        let viewport_u = u * (viewport_width as Float);
        let viewport_v = -v * (viewport_height as Float);
        let delta_u = viewport_u / (img_width as Float);
        let delta_v = viewport_v / (img_height as Float);
        let upper_left = Point::from(camera_options.position - viewport_u / 2.0 - viewport_v / 2.0 - w * camera_options.focus_dist);
        let pixel00_loc = upper_left + (delta_u + delta_v) / 2.0;

        Viewport {
            delta_u,
            delta_v,
            u,
            v,
            pixel00_loc,
        }
    }

    pub fn pixel_loc(&self, x_idx: Float, y_idx: Float) -> Point {
        self.pixel00_loc + self.delta_u * x_idx + self.delta_v * y_idx
    }
}
