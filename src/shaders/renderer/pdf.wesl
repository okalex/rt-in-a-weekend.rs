import package::{
    constants::PI,
    onb::{Onb, new_onb, transform_onb},
    random::{rand_vector_on_hemisphere, rand_cos_vector},
};

struct HemispherePdf {
    normal: vec3<f32>,
};

fn new_hemisphere_pdf(normal: vec3<f32>) -> HemispherePdf {
    return HemispherePdf(normal);
}

fn hemisphere_generate(pdf: HemispherePdf) -> vec3<f32> {
    return rand_vector_on_hemisphere(pdf.normal);
}

fn hemisphere_pdf_value(pdf: HemispherePdf) -> f32 {
    return 1.0 / (2.0 * PI);
}

struct CosinePdf {
    uvw: Onb,
};

fn new_cosine_pdf(w: vec3<f32>) -> CosinePdf {
    return CosinePdf(new_onb(w));
}

fn cosine_generate(pdf: CosinePdf) -> vec3<f32> {
    return transform_onb(pdf.uvw, rand_cos_vector());
}

fn cosine_pdf_value(pdf: CosinePdf, direction: vec3<f32>) -> f32 {
    let cos_theta = dot(normalize(direction), pdf.uvw.axes[2]);
    return max(0.0, cos_theta / PI);
}
