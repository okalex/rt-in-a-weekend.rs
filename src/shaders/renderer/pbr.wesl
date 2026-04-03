import package::{
    color,
    constants,
    ggx,
    hit_record,
    pdf,
    random,
    ray,
    scatter_record,
    types,
    vector,
};

const MIRROR_ROUGHNESS_EPSILON = 1e-6;

fn scatter(pbr_mat: types::GpuMaterial_Pbr, ray_in: ray::Ray, hit_record: hit_record::HitRecord) -> scatter_record::ScatterRecord {
    let wo = -normalize(ray_in.dir);
    if is_mirror_limit(pbr_mat) {
        return scatter_mirror_limit(pbr_mat, ray_in, hit_record, wo);
    }

    let pdf = ggx_pdf(pbr_mat, wo, hit_record.normal);
    let scattered_dir = pdf::mixture_pdf_generate(
        pbr_mat.p_spec,
        pdf::ggx_pdf_generate(pdf),
        pdf::cosine_generate(pdf::new_cosine_pdf(hit_record.normal)),
    );

    return scatter_record::with_pdf(
        vec3f(1.0, 1.0, 1.0),
        ray::Ray(hit_record.point, scattered_dir),
    );
}

fn ggx_pdf(pbr_mat: types::GpuMaterial_Pbr, wo: vec3f, normal: vec3f) -> pdf::GgxPdf {
    return pdf::GgxPdf(wo, normal, pbr_mat.alpha);
}

fn brdf(
    pbr_mat: types::GpuMaterial_Pbr,
    ray_in: ray::Ray,
    hit_record: hit_record::HitRecord,
    scattered_dir: vec3f
) -> vec3f {
    let wo = -normalize(ray_in.dir);
    let wi = normalize(scattered_dir);

    if is_mirror_limit(pbr_mat) {
        return mirror_limit_diffuse_brdf(pbr_mat, hit_record, wo);
    }

    let oh = vector::half_vector(wi, wo);
    if !oh.is_some {
        return vec3f(0.0, 0.0, 0.0);
    }
    let h = oh.vector;

    let v_dot_h = dot(wi, h);
    let n_dot_h = dot(hit_record.normal, h);
    let n_dot_i = dot(hit_record.normal, wi);
    let n_dot_o = dot(hit_record.normal, wo);

    if n_dot_i <= 0.0 || n_dot_o <= 0.0 || n_dot_h <= 0.0 || v_dot_h <= 0.0 {
        return vec3f(0.0, 0.0, 0.0);
    }

    let alpha_sqrd = pbr_mat.alpha * pbr_mat.alpha;
    let f = ggx::schlick_fresnel(pbr_mat.f0, v_dot_h);
    let d = ggx::ggx_d(n_dot_h, alpha_sqrd);
    let g = ggx::smith_g2(n_dot_i, n_dot_o, alpha_sqrd);

    let diffuse = (vec3f(1.0, 1.0, 1.0) - f) * (1.0 - pbr_mat.metallic) * pbr_mat.albedo / constants::PI;
    let specular = d * f * g / (4.0 * n_dot_i * n_dot_o);

    return diffuse + specular;
}

fn is_mirror_limit(pbr_mat: types::GpuMaterial_Pbr) -> bool {
    return pbr_mat.roughness <= MIRROR_ROUGHNESS_EPSILON;
}

fn scatter_mirror_limit(
    pbr_mat: types::GpuMaterial_Pbr,
    ray_in: ray::Ray,
    hit_record: hit_record::HitRecord,
    wo: vec3f,
) -> scatter_record::ScatterRecord {
    let weights = mirror_limit_weights(pbr_mat, hit_record.normal, wo);
    if !weights.is_valid {
        return scatter_record::none();
    }

    if weights.specular_probability >= 1.0 || (weights.specular_probability > 0.0 && random::rand() < weights.specular_probability) {
        let reflected = reflect(normalize(ray_in.dir), hit_record.normal);
        return scatter_record::skip_pdf(
            weights.specular_color / max(weights.specular_probability, constants::HALF_VECTOR_EPSILON),
            ray::Ray(hit_record.point, reflected),
        );
    }

    if weights.diffuse_probability <= 0.0 {
        return scatter_record::none();
    }

    return scatter_record::with_pdf(
        vec3f(1.0, 1.0, 1.0),
        ray::Ray(hit_record.point, pdf::cosine_generate(pdf::new_cosine_pdf(hit_record.normal))),
    );
}

fn diffuse_color(pbr_mat: types::GpuMaterial_Pbr, fresnel: vec3f) -> vec3f {
    return (vec3f(1.0, 1.0, 1.0) - fresnel) * (1.0 - pbr_mat.metallic) * pbr_mat.albedo;
}

struct MirrorLimitWeights {
    is_valid: bool,
    specular_color: vec3f,
    diffuse_color: vec3f,
    specular_probability: f32,
    diffuse_probability: f32,
};

fn no_mirror_limits() -> MirrorLimitWeights {
    return MirrorLimitWeights(false, vec3f(0.0, 0.0, 0.0), vec3f(0.0, 0.0, 0.0), 0.0, 0.0);
}

fn mirror_limit_weights(
    pbr_mat: types::GpuMaterial_Pbr,
    normal: vec3f,
    wo: vec3f,
) -> MirrorLimitWeights {
    let n_dot_o = clamp(dot(normal, wo), 0.0, 1.0);
    let specular_color = ggx::schlick_fresnel(pbr_mat.f0, n_dot_o);
    let diff_color = diffuse_color(pbr_mat, specular_color);
    let specular_luma = color::luminance(specular_color);
    let diff_luma = color::luminance(diff_color);
    let total_luma = specular_luma + diff_luma;

    if total_luma <= 0.0 {
        return no_mirror_limits();
    }

    let specular_probability = clamp(specular_luma / total_luma, 0.0, 1.0);

    return MirrorLimitWeights(
        true,
        specular_color,
        diff_color,
        specular_probability,
        1.0 - specular_probability,
    );
}

fn mirror_limit_diffuse_brdf(
    pbr_mat: types::GpuMaterial_Pbr,
    hit_record: hit_record::HitRecord,
    wo: vec3f
) -> vec3f {
    let weights = mirror_limit_weights(pbr_mat, hit_record.normal, wo);
    if !weights.is_valid {
        return vec3f(0.0, 0.0, 0.0);
    }

    if weights.diffuse_probability <= 0.0 {
        return vec3f(0.0, 0.0, 0.0);
    }

    return weights.diffuse_color / (constants::PI * clamp(weights.diffuse_probability, constants::HALF_VECTOR_EPSILON, 1.0));
}
