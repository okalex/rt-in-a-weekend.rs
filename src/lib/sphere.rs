use std::sync::Arc;
use crate::lib::{hittable, ray, vec3, interval, material};

pub struct Sphere {
  pub center: vec3::Vec3,
  pub radius: f64,
  pub mat: Arc<dyn material::Material>,
}

impl hittable::Hittable for Sphere {

  fn hit(&self, ray: &ray::Ray, ray_t: interval::Interval) -> hittable::HitRecord {
    let oc = self.center - *ray.orig();
    let a = ray.dir().length_squared();
    let h = ray.dir().dot(&oc);
    let c = oc.length_squared() - self.radius * self.radius;
    let discriminant = (h * h) - (a * c);

    if discriminant < 0.0 {
      return hittable::no_hit();
    }

    let sqrtd = discriminant.sqrt();
    let mut root = (h - sqrtd) / a;
    if !ray_t.surrounds(root) {
      root = (h + sqrtd) / a;
      if !ray_t.surrounds(root) {
        return hittable::no_hit();
      }
    }

    let point = ray.at(root);
    let normal = (point - self.center).scale(1.0 / self.radius);
    let front_face = ray.dir().dot(&normal) < 0.0;
    return hittable::hit_record(&point, &normal, front_face, root, Arc::clone(&self.mat));
  }

}

pub fn new(center: vec3::Vec3, radius: f64, mat: Arc<dyn material::Material>) -> Sphere {
  return Sphere {
    center: center,
    radius: radius,
    mat: mat,
  };
}

pub fn new_arr(center: [f64; 3], radius: f64, mat: Arc<dyn material::Material>) -> Sphere {
  return new(vec3::new_arr(center), radius, mat);
}
