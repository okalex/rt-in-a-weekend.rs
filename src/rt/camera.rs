use core::f64;

use nalgebra::{Point3, Vector3};

use crate::rt::{
    random::{rand, rand_in_unit_disk},
    ray::Ray,
    util::degrees_to_radians,
    viewport::Viewport,
};

struct Disk {
    pub u: Vector3<f64>,
    pub v: Vector3<f64>,
}

pub struct Camera {
    options: CameraOptions,
    viewport: Viewport,
    defocus_disk: Disk,
    samples_per_pixel: u32,
}

impl Camera {
    pub fn new(options: CameraOptions, viewport: Viewport, samples_per_pixel: u32) -> Self {
        let defocus_radius =
            options.focus_dist * degrees_to_radians(options.defocus_angle / 2.0).tan();
        let defocus_disk = Disk {
            u: viewport.u * defocus_radius,
            v: viewport.v * defocus_radius,
        };
        Self {
            options,
            viewport,
            defocus_disk,
            samples_per_pixel,
        }
    }

    pub fn foreach_ray<F>(&self, i: u32, j: u32, mut f: F)
    where
        F: FnMut(Ray) -> (),
    {
        self.foreach_sample_stratified(|offset| {
            let ray = self.get_ray(i, j, offset);
            f(ray);
        })
    }

    fn get_ray(&self, i: u32, j: u32, offset: Vector3<f64>) -> Ray {
        let pixel_sample = self
            .viewport
            .pixel_loc(i as f64 + offset.x, j as f64 + offset.y);

        let ray_origin = if self.options.defocus_angle <= 0.0 {
            self.options.position
        } else {
            self.defocus_disk_sample()
        };
        let ray_dir = pixel_sample - ray_origin;
        let ray_time = rand();

        Ray::new(ray_origin, ray_dir, ray_time)
    }

    fn defocus_disk_sample(&self) -> Point3<f64> {
        let p = rand_in_unit_disk();
        return Point3::from(
            self.options.position.coords
                + (self.defocus_disk.u * p.x)
                + (self.defocus_disk.v * p.y),
        );
    }

    #[allow(unused)]
    fn foreach_sample<F>(&self, mut f: F)
    where
        F: FnMut(Vector3<f64>) -> (),
    {
        for _ in 0..self.samples_per_pixel {
            let offset = self.sample_square();
            f(offset)
        }
    }

    #[allow(unused)]
    fn sample_square(&self) -> Vector3<f64> {
        Vector3::new(rand() - 0.5, rand() - 0.5, 0.0)
    }

    #[allow(unused)]
    fn foreach_sample_stratified<F>(&self, mut f: F)
    where
        F: FnMut(Vector3<f64>) -> (),
    {
        let sqrt_spp = (self.samples_per_pixel as f64).sqrt();
        let recip_sqrt_spp = 1.0 / sqrt_spp;

        for s_j in 0..sqrt_spp as u32 {
            for s_i in 0..sqrt_spp as u32 {
                let offset = self.sample_square_stratified(s_i, s_j, recip_sqrt_spp);
                f(offset)
            }
        }
    }

    #[allow(unused)]
    fn sample_square_stratified(&self, s_i: u32, s_j: u32, recip_sqrt_spp: f64) -> Vector3<f64> {
        let px = ((s_i as f64 + rand()) * recip_sqrt_spp) - 0.5;
        let py = ((s_j as f64 + rand()) * recip_sqrt_spp) - 0.5;
        Vector3::new(px, py, 0.0)
    }
}

#[derive(Clone, Copy)]
pub struct CameraOptions {
    pub position: Point3<f64>,
    pub target: Point3<f64>,
    pub vup: Vector3<f64>,
    pub vfov: f64,
    pub focus_dist: f64,
    pub defocus_angle: f64,
}

impl CameraOptions {
    pub fn new() -> CameraOptions {
        CameraOptions {
            position: Point3::new(0.0, 1.0, 0.0),
            target: Point3::new(0.0, 0.0, 0.0),
            vup: Vector3::new(0.0, 1.0, 0.0),
            vfov: 20.0,
            defocus_angle: 0.0,
            focus_dist: 1.0,
        }
    }

    #[allow(dead_code)]
    pub fn vfov(mut self, new_vfov: f64) -> Self {
        self.vfov = new_vfov;
        self
    }

    #[allow(dead_code)]
    pub fn position(mut self, new_position: [f64; 3]) -> Self {
        self.position = Point3::from(new_position);
        self
    }

    #[allow(dead_code)]
    pub fn target(mut self, new_target: [f64; 3]) -> Self {
        self.target = Point3::from(new_target);
        self
    }

    #[allow(dead_code)]
    pub fn vup(mut self, new_vup: [f64; 3]) -> Self {
        self.vup = Vector3::from(new_vup);
        self
    }

    #[allow(dead_code)]
    pub fn defocus_angle(mut self, new_defocus_angle: f64) -> Self {
        self.defocus_angle = new_defocus_angle;
        self
    }

    #[allow(dead_code)]
    pub fn focus_dist(mut self, new_focus_dist: f64) -> Self {
        self.focus_dist = new_focus_dist;
        self
    }
}
