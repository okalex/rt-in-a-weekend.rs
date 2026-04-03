import package::{
    ray,
    vector,
};

struct ScatterRecord {
    is_scattered: bool,
    attenuation: vec3f,
    scattered: ray::Ray,
    skip_pdf: bool,
}

fn none() -> ScatterRecord {
    return ScatterRecord(
        false,
        vec3f(0, 0, 0),
        ray::Ray(vector::ZEROS, vector::ZEROS),
        false,
    );
}

fn with_pdf(
    attenuation: vec3f,
    scattered: ray::Ray,
) -> ScatterRecord {
    return ScatterRecord(
        true,
        attenuation,
        scattered,
        false,
    );
}

fn skip_pdf(
    attenuation: vec3f,
    skip_pdf_ray: ray::Ray
) -> ScatterRecord {
    return ScatterRecord(
        true,
        attenuation,
        skip_pdf_ray,
        true,
    );
}
