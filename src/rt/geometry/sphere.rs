use crate::{
    rt::{
        geometry::{aabb::Aabb, hit_record::HitRecord},
        onb::Onb,
        ray::Ray,
    },
    util::{
        interval::Interval,
        random::rand,
        types::{Float, INFINITY, PI, Point, Vector},
    },
};

#[derive(Clone)]
pub struct Sphere {
    pub center: Ray,
    pub radius: Float,
    pub aabb: Aabb,
}

impl Sphere {
    pub fn new(center: Ray, radius: Float, aabb: Aabb) -> Sphere {
        Sphere { center, radius, aabb }
    }

    pub fn stationary(center: Point, radius: Float) -> Sphere {
        let ray = Ray::new(center, Vector::ZERO, 0.0);
        let rvec = Vector::splat(radius);
        let aabb = Aabb::new(center - rvec, center + rvec);
        Self::new(ray, radius, aabb)
    }

    pub fn get_uv(normal: &Vector) -> (Float, Float) {
        let u = 0.5 + (-normal.z).atan2(normal.x) / (2.0 * PI);
        let v = 0.5 + normal.y.asin() / PI;
        (u, v)
    }

    fn rand_to_sphere(radius: Float, dist_sqrd: Float) -> Vector {
        let r1 = rand();
        let r2 = rand();
        let z = 1.0 + r2 * ((1.0 - radius * radius / dist_sqrd).sqrt() - 1.0);

        let phi = 2.0 * PI * r1;
        let tmp = (1.0 - z * z).sqrt();
        let x = phi.cos() * tmp;
        let y = phi.sin() * tmp;

        Vector::new(x, y, z)
    }

    pub fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let curr_center = self.center.at(ray.time);
        let oc = curr_center - ray.orig;
        let a = ray.dir.length_squared();
        let h = ray.dir.dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = (h * h) - (a * c);

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (h - sqrtd) / a;

        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let point = ray.at(root);
        let outward_normal = (point - curr_center) / self.radius;
        let (front_face, face_normal) = HitRecord::get_front_face(ray, outward_normal);
        let (u, v) = Sphere::get_uv(&face_normal); // Why is this not &point?

        Some(HitRecord::new(point, face_normal, front_face, root, u, v))
    }

    pub fn pdf_value(&self, origin: &Point, direction: &Vector) -> Float {
        // This method only works for stationary spheres.
        let ray = Ray::new(*origin, *direction, 0.0);
        let interval = Interval::new(0.001, INFINITY);
        match self.hit(&ray, interval) {
            None => 0.0,
            Some(_) => {
                let dist_sqrd = (self.center.at(0.0) - origin).length_squared();
                let cos_theta_max = (1.0 - self.radius * self.radius / dist_sqrd).sqrt();
                let solid_angle = 2.0 * PI * (1.0 - cos_theta_max);
                1.0 / solid_angle
            }
        }
    }

    pub fn random(&self, origin: &Point) -> Vector {
        let direction = self.center.at(0.0) - origin;
        let dist_sqrd = direction.length_squared();
        let uvw = Onb::new(&direction);
        uvw.transform(Self::rand_to_sphere(self.radius, dist_sqrd))
    }
}
