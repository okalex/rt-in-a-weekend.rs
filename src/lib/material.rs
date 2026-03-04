use crate::lib::{ray, hittable, color, vec3, random};

pub struct Scattered {
  pub ray: ray::Ray,
  pub attenuation: color::Color,
}

pub trait Material: Send + Sync {
  fn scatter(&self, r_in: &ray::Ray, rec: &hittable::HitRecord) -> Scattered;
}

pub struct DefaultMaterial {}

impl Material for DefaultMaterial {
  fn scatter(&self, r_in: &ray::Ray, rec: &hittable::HitRecord) -> Scattered {
    return Scattered {
      ray: ray::new(vec3::zeroes(), vec3::zeroes()),
      attenuation: color::wrap_vec(vec3::zeroes()),
    }
  }
}

pub fn default_material() -> DefaultMaterial {
  return DefaultMaterial {};
}

pub struct Lambertian {
  albedo: color::Color,
}

impl Material for Lambertian {
  fn scatter(&self, r_in: &ray::Ray, rec: &hittable::HitRecord) -> Scattered {
    let mut scatter_dir = rec.normal + vec3::rand_unit();
    if scatter_dir.near_zero() {
      scatter_dir = rec.normal;
    }

    return Scattered {
      ray: ray::new(rec.point, scatter_dir),
      attenuation: self.albedo,
    };
  }
}

pub fn lambertian(albedo: [f64; 3]) -> Lambertian {
  return Lambertian {
    albedo: color::new_vec(albedo[0], albedo[1], albedo[2]),
  };
}

pub struct Metal {
  albedo: color::Color,
  fuzz: f64,
}

impl Material for Metal {
  fn scatter(&self, r_in: &ray::Ray, rec: &hittable::HitRecord) -> Scattered {
    let reflected = r_in.dir().reflect(&rec.normal).unit() + vec3::rand_unit().scale(self.fuzz);

    return Scattered {
      ray: ray::new(rec.point, reflected),
      attenuation: self.albedo,
    };
  }
}

pub fn metal(albedo: [f64; 3], fuzz: f64) -> Metal {
  return Metal {
    albedo: color::new_vec(albedo[0], albedo[1], albedo[2]),
    fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
  };
}

pub struct Dielectric {
  refraction_idx: f64,
}

impl Material for Dielectric {
  fn scatter(&self, r_in: &ray::Ray, rec: &hittable::HitRecord) -> Scattered {
    let ri = if rec.front_face { 1.0 / self.refraction_idx } else { self.refraction_idx };

    let unit_dir = r_in.dir().unit();
    let cos_theta = f64::min((-unit_dir).dot(&rec.normal), 1.0);
    let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
    
    let cannot_refract = (ri * sin_theta) > 1.0;
    let direction = if cannot_refract || reflectance(cos_theta, ri) > random::rand() { 
      unit_dir.reflect(&rec.normal) 
    } else { 
      unit_dir.refract(&rec.normal, ri) 
    };

    return Scattered {
      ray: ray::new(rec.point, direction),
      attenuation: color::new_vec(1.0, 1.0, 1.0),
    };
  }
}

pub fn dielectric(refraction_idx: f64) -> Dielectric {
  return Dielectric {
    refraction_idx: refraction_idx,
  };
}

fn reflectance(cosine: f64, refraction_idx: f64) -> f64 {
  let r0_tmp = (1.0 - refraction_idx) / (1.0 + refraction_idx);
  let r0 = r0_tmp * r0_tmp;
  return r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0);
}
