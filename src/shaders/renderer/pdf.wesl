import package::{
    bindings::{instances, primitives, lights},
    constants::PI,
    float,
    ggx,
    onb,
    primitive,
    random,
    transform,
    vector,
};

fn sphere_generate() -> vec3f {
    return random::rand_unit_vector();
}

fn sphere_pdf_value() -> f32 {
    return 1.0 / (4.0 * PI);
}

struct HemispherePdf {
    normal: vec3f,
};

fn new_hemisphere_pdf(normal: vec3f) -> HemispherePdf {
    return HemispherePdf(normal);
}

fn hemisphere_generate(pdf: HemispherePdf) -> vec3f {
    return random::rand_vector_on_hemisphere(pdf.normal);
}

fn hemisphere_pdf_value(pdf: HemispherePdf) -> f32 {
    return 1.0 / (2.0 * PI);
}

struct CosinePdf {
    uvw: onb::Onb,
};

fn new_cosine_pdf(w: vec3f) -> CosinePdf {
    return CosinePdf(onb::new_onb(w));
}

fn cosine_generate(pdf: CosinePdf) -> vec3f {
    return onb::transform(pdf.uvw, random::rand_cos_vector());
}

fn cosine_pdf_value(pdf: CosinePdf, direction: vec3f) -> f32 {
    let cos_theta = dot(normalize(direction), pdf.uvw.axes[2]);
    return max(0.0, cos_theta / PI);
}

// MixturePdf

fn mixture_pdf_generate(weight: f32, p0_dir: vec3f, p1_dir: vec3f) -> vec3f {
    if random::rand() < weight {
        return p0_dir;
    } else {
        return p1_dir;
    }
}

fn mixture_pdf_value(weight: f32, p0_value: f32, p1_value: f32) -> f32 {
    return weight * p0_value + (1.0 - weight) * p1_value;
}

struct GgxPdf {
    view_dir: vec3f,
    normal: vec3f,
    alpha: f32,
};

fn ggx_pdf_value(pdf: GgxPdf, direction: vec3f) -> f32 {
    let oi = vector::normalize_if_valid(direction);
    if !oi.is_some {
        return 0.0;
    }
    let incoming_dir = oi.vector;

    let oh = vector::half_vector(pdf.view_dir, incoming_dir);
    if !oh.is_some {
        return 0.0;
    }
    let half_vector = oh.vector;

    let n_dot_h = dot(pdf.normal, half_vector);
    let n_dot_i = dot(pdf.normal, incoming_dir);
    let n_dot_v = dot(pdf.normal, pdf.view_dir);

    if n_dot_h <= 0.0 || n_dot_i <= 0.0 || n_dot_v <= 0.0 {
        return 0.0;
    }

    return sanitize_pdf_value(visible_microfacet_pdf_value(pdf, n_dot_h, n_dot_v));
}

fn ggx_pdf_generate(pdf: GgxPdf) -> vec3f {
    let shading_basis = onb::new_onb(pdf.normal);
    let local_view_dir = onb::inv_transform(shading_basis, pdf.view_dir);
    let stretched_view_dir = stretch_view_dir(local_view_dir, pdf.alpha);
    let disk_sample = sample_visible_normal_disk(stretched_view_dir);
    let normal_frame = visible_normal_frame(stretched_view_dir);
    let stretched_half = stretched_half_vector(disk_sample, normal_frame, stretched_view_dir);
    let local_half = unstretch_half_vector(stretched_half, pdf.alpha);
    let world_half = onb::transform(shading_basis, local_half);

    return reflect(-pdf.view_dir, world_half);
}

fn sanitize_pdf_value(value: f32) -> f32 {
    if float::is_finite(value) && value > 0.0 {
        return value;
    } else {
        return 0.0;
    }
}

fn visible_microfacet_pdf_value(pdf: GgxPdf, n_dot_h: f32, n_dot_v: f32) -> f32 {
    let alpha_sqrd = pdf.alpha * pdf.alpha;
    let normal_distribution = ggx::ggx_d(n_dot_h, alpha_sqrd);
    let masking = ggx::smith_g1(n_dot_v, alpha_sqrd);
    return normal_distribution * masking / (4.0 * n_dot_v);
}

fn stretch_view_dir(local_view_dir: vec3f, alpha: f32) -> vec3f {
    let ov = vector::normalize_if_valid(vec3f(
        alpha * local_view_dir.x,
        alpha * local_view_dir.y,
        local_view_dir.z
    ));
    if !ov.is_some {
        return vec3f(0.0, 0.0, 1.0);
    }
    return ov.vector;
}

fn sample_visible_normal_disk(stretched_view_dir: vec3f) -> vec3f {
    var disk_sample = random::rand_on_unit_disk();
    let blend = 0.5 * (1.0 + stretched_view_dir.z);
    disk_sample.y = (1.0 - blend) * sqrt(1.0 - disk_sample.x * disk_sample.x) + blend * disk_sample.y;
    disk_sample.z = sqrt(max(0.0, 1.0 - disk_sample.x * disk_sample.x - disk_sample.y * disk_sample.y));
    return disk_sample;
}

struct VisibleNormalFrame {
    tangent: vec3f,
    bitangent: vec3f,
};

fn visible_normal_frame(stretched_view_dir: vec3f) -> VisibleNormalFrame {
    var tangent: vec3f;
    if abs(stretched_view_dir.z) < 0.999 {
        tangent = normalize(cross(vec3f(0.0, 0.0, 1.0), stretched_view_dir));
    } else {
        tangent = normalize(cross(vec3f(1.0, 0.0, 0.0), stretched_view_dir));
    }
    let bitangent = cross(stretched_view_dir, tangent);

    return VisibleNormalFrame(tangent, bitangent);
}

fn stretched_half_vector(
    disk_sample: vec3f,
    normal_frame: VisibleNormalFrame,
    stretched_view_dir: vec3f,
) -> vec3f {
    return disk_sample.x * normal_frame.tangent + disk_sample.y * normal_frame.bitangent + disk_sample.z * stretched_view_dir;
}

fn unstretch_half_vector(stretched_half_vector: vec3f, alpha: f32) -> vec3f {
    let new_vec = vec3f(
        alpha * stretched_half_vector.x,
        alpha * stretched_half_vector.y,
        max(stretched_half_vector.z, 0.0)
    );
    let ov = vector::normalize_if_valid(new_vec);
    if !ov.is_some {
        return vec3f(0.0, 0.0, 1.0);
    }
    return ov.vector;
}

struct LightsPdf {
    origin: vec3f,
};

fn lights_pdf_value(pdf: LightsPdf, direction: vec3f) -> f32 {
    let num_lights = lights.count;
    let weight = 1.0 / f32(num_lights);
    var sum = 0.0;

    for (var i = 0u; i < num_lights; i = i + 1) {
        let light_instance_id = lights.lights[i];
        let light_instance = instances.instances[light_instance_id];
        let light_primitive = primitives.primitives[light_instance.primitive_id];
        let local_origin = transform::transform_point(pdf.origin, light_instance.inv_transform);
        let local_dir = transform::transform_direction(direction, light_instance.inv_transform);
        sum += weight * primitive::pdf_value(light_primitive, local_origin, local_dir);
    }

    return sum;
}

fn lights_pdf_generate(pdf: LightsPdf) -> vec3f {
    let num_lights = lights.count;
    let light_id = random::rand_range_int(i32(0), i32(num_lights - 1));
    let light_instance_id = lights.lights[light_id];
    let light_instance = instances.instances[light_instance_id];
    let light_primitive = primitives.primitives[light_instance.primitive_id];

    let local_origin = transform::transform_point(pdf.origin, light_instance.inv_transform);
    let local_dir = primitive::rand(light_primitive, local_origin);
    return normalize(transform::transform_direction(local_dir, light_instance.transform));
}
