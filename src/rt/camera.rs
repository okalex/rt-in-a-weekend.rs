use crate::rt::{
    random::{rand, rand_in_unit_disk},
    ray::Ray,
    sampler::Sampler,
    types::{Float, Point, Uint, Vector},
    util::degrees_to_radians,
    viewport::Viewport,
};

struct Disk {
    pub u: Vector,
    pub v: Vector,
}

pub struct Camera {
    options: CameraOptions,
    viewport: Viewport,
    defocus_disk: Disk,
    pub sampler: Sampler,
}

impl Camera {
    pub fn new(options: CameraOptions, viewport: Viewport, sampler: Sampler) -> Self {
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
            sampler,
        }
    }

    pub fn foreach_ray<F>(&self, i: Uint, j: Uint, mut f: F)
    where
        F: FnMut(Ray) -> (),
    {
        self.sampler.foreach_sample(|offset| {
            let ray = self.get_ray(i, j, offset);
            f(ray);
        });
    }

    fn get_ray(&self, i: Uint, j: Uint, offset: Vector) -> Ray {
        let pixel_sample = self
            .viewport
            .pixel_loc(i as Float + offset.x, j as Float + offset.y);

        let ray_origin = if self.options.defocus_angle <= 0.0 {
            self.options.position
        } else {
            self.defocus_disk_sample()
        };
        let ray_dir = pixel_sample - ray_origin;
        let ray_time = rand();

        Ray::new(ray_origin, ray_dir, ray_time)
    }

    fn defocus_disk_sample(&self) -> Point {
        let p = rand_in_unit_disk();
        return Point::from(
            self.options.position + (self.defocus_disk.u * p.x) + (self.defocus_disk.v * p.y),
        );
    }
}

pub struct CameraOptions {
    pub position: Point,
    pub target: Point,
    pub vup: Vector,
    pub vfov: Float,
    pub focus_dist: Float,
    pub defocus_angle: Float,
}

impl CameraOptions {
    pub fn new() -> CameraOptions {
        CameraOptions {
            position: Point::new(0.0, 1.0, 0.0),
            target: Point::new(0.0, 0.0, 0.0),
            vup: Vector::new(0.0, 1.0, 0.0),
            vfov: 20.0,
            defocus_angle: 0.0,
            focus_dist: 1.0,
        }
    }

    #[allow(dead_code)]
    pub fn vfov(mut self, new_vfov: Float) -> Self {
        self.vfov = new_vfov;
        self
    }

    #[allow(dead_code)]
    pub fn position(mut self, new_position: [Float; 3]) -> Self {
        self.position = Point::from(new_position);
        self
    }

    #[allow(dead_code)]
    pub fn target(mut self, new_target: [Float; 3]) -> Self {
        self.target = Point::from(new_target);
        self
    }

    #[allow(dead_code)]
    pub fn vup(mut self, new_vup: [Float; 3]) -> Self {
        self.vup = Vector::from(new_vup);
        self
    }

    #[allow(dead_code)]
    pub fn defocus_angle(mut self, new_defocus_angle: Float) -> Self {
        self.defocus_angle = new_defocus_angle;
        self
    }

    #[allow(dead_code)]
    pub fn focus_dist(mut self, new_focus_dist: Float) -> Self {
        self.focus_dist = new_focus_dist;
        self
    }
}
