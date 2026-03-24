import package::{
    bindings::{instances, primitives, lights},
    constants::PI,
    interval::Interval,
    onb::{Onb, new_onb, transform_onb},
    quad::{hit_quad, quad_pdf_value, quad_random},
    random::{rand_vector_on_hemisphere, rand_cos_vector, rand_range_int, rand_unit_vector},
    ray::Ray,
    sphere::{sphere_pdf_value, sphere_random},
    transform::{transform_point, transform_direction},
    types,
};

fn sphere_generate() -> vec3<f32> {
    return rand_unit_vector();
}

fn sphere_value() -> f32 {
    return 1.0 / (4.0 * PI);
}

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

struct LightsPdf {
    origin: vec3<f32>,
};

fn lights_pdf_value(pdf: LightsPdf, direction: vec3<f32>) -> f32 {
    let num_lights = arrayLength(&lights.lights);
    let weight = 1.0 / f32(num_lights);
    var sum = 0.0;

    for (var i = 0u; i < num_lights; i = i + 1) {
        let light_instance_id = lights.lights[i];
        let light_instance = instances.instances[light_instance_id];
        let light_primitive = primitives.primitives[light_instance.primitive_id];
        let local_origin = transform_point(pdf.origin, light_instance.inv_transform);
        let local_dir = transform_direction(direction, light_instance.inv_transform);
        sum += weight * primitive_pdf_value(light_primitive, local_origin, local_dir);
    }

    return sum;
}

fn lights_pdf_generate(pdf: LightsPdf) -> vec3<f32> {
    let num_lights = arrayLength(&lights.lights);
    let light_id = rand_range_int(i32(0), i32(num_lights - 1));
    let light_instance_id = lights.lights[light_id];
    let light_instance = instances.instances[light_instance_id];
    let light_primitive = primitives.primitives[light_instance.primitive_id];

    let local_origin = transform_point(pdf.origin, light_instance.inv_transform);
    let local_dir = primitive_random(light_primitive, local_origin);
    return normalize(transform_direction(local_dir, light_instance.transform));
}

fn primitive_pdf_value(primitive: types::GpuPrimitive, origin: vec3<f32>, direction: vec3<f32>) -> f32 {
    switch primitive._type {
        case types::GPUPRIMITIVE_SPHERE: {
            let sphere = types::unpack_GpuPrimitive_Sphere(primitive);
            return sphere_pdf_value(sphere, origin, direction);
        }

        // case types::GPUPRIMITIVE_TRIANGLE: {
        //     let triangle = types::unpack_GpuPrimitive_Triangle(primitive);
        // }

        case types::GPUPRIMITIVE_QUAD: {
            let quad = types::unpack_GpuPrimitive_Quad(primitive);
            return quad_pdf_value(quad, origin, direction);
        }

        // case types::GPUPRIMITIVE_MESH: {
        //     let mesh = types::unpack_GpuPrimitive_Mesh(primitive);
        // }

        default: {
            return 0.0;
        }
    }
}

fn primitive_random(primitive: types::GpuPrimitive, origin: vec3<f32>) -> vec3<f32> {
    switch primitive._type {
        case types::GPUPRIMITIVE_SPHERE: {
            let sphere = types::unpack_GpuPrimitive_Sphere(primitive);
            return sphere_random(sphere, origin);
        }

        // case types::GPUPRIMITIVE_TRIANGLE: {
        //     let triangle = types::unpack_GpuPrimitive_Triangle(primitive);
        // }

        case types::GPUPRIMITIVE_QUAD: {
            let quad = types::unpack_GpuPrimitive_Quad(primitive);
            return quad_random(quad, origin);
        }

        // case types::GPUPRIMITIVE_MESH: {
        //     let mesh = types::unpack_GpuPrimitive_Mesh(primitive);
        // }

        default: {
            return vec3<f32>(1.0, 0.0, 0.0);
        }
    }
}
