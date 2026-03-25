import package::{
    bindings::primitives,
    hit_record,
    interval,
    medium,
    mesh,
    quad,
    ray,
    sphere,
    triangle,
    types,
};

fn hit(
    primitive: types::GpuPrimitive,
    ray: ray::Ray,
    ray_t: interval::Interval
) -> hit_record::HitRecord {

    switch primitive._type {
        case types::GPUPRIMITIVE_SPHERE: {
            let sphere = types::unpack_GpuPrimitive_Sphere(primitive);
            return sphere::hit(sphere, ray, ray_t);
        }

        case types::GPUPRIMITIVE_TRIANGLE: {
            let triangle = types::unpack_GpuPrimitive_Triangle(primitive);
            return triangle::hit(triangle, ray, ray_t);
        }

        case types::GPUPRIMITIVE_QUAD: {
            let quad = types::unpack_GpuPrimitive_Quad(primitive);
            return quad::hit(quad, ray, ray_t);
        }

        case types::GPUPRIMITIVE_MESH: {
            let mesh = types::unpack_GpuPrimitive_Mesh(primitive);
            return mesh::hit(mesh, ray, ray_t);
        }

        case types::GPUPRIMITIVE_MEDIUM: {
            let medium = types::unpack_GpuPrimitive_Medium(primitive);
            let boundary_id = medium.boundary_primitive_id;
            let boundary = primitives.primitives[boundary_id];
            return medium::hit(medium, boundary, ray, ray_t);
        }

        default: {
            return hit_record::no_hit();
        }
    }
}

fn pdf_value(primitive: types::GpuPrimitive, origin: vec3f, direction: vec3f) -> f32 {
    switch primitive._type {
        case types::GPUPRIMITIVE_SPHERE: {
            let sphere = types::unpack_GpuPrimitive_Sphere(primitive);
            return sphere::pdf_value(sphere, origin, direction);
        }

        // case types::GPUPRIMITIVE_TRIANGLE: {
        //     let triangle = types::unpack_GpuPrimitive_Triangle(primitive);
        // }

        case types::GPUPRIMITIVE_QUAD: {
            let quad = types::unpack_GpuPrimitive_Quad(primitive);
            return quad::pdf_value(quad, origin, direction);
        }

        // case types::GPUPRIMITIVE_MESH: {
        //     let mesh = types::unpack_GpuPrimitive_Mesh(primitive);
        // }

        default: {
            return 0.0;
        }
    }
}

fn rand(primitive: types::GpuPrimitive, origin: vec3f) -> vec3f {
    switch primitive._type {
        case types::GPUPRIMITIVE_SPHERE: {
            let sphere = types::unpack_GpuPrimitive_Sphere(primitive);
            return sphere::rand(sphere, origin);
        }

        // case types::GPUPRIMITIVE_TRIANGLE: {
        //     let triangle = types::unpack_GpuPrimitive_Triangle(primitive);
        // }

        case types::GPUPRIMITIVE_QUAD: {
            let quad = types::unpack_GpuPrimitive_Quad(primitive);
            return quad::rand(quad, origin);
        }

        // case types::GPUPRIMITIVE_MESH: {
        //     let mesh = types::unpack_GpuPrimitive_Mesh(primitive);
        // }

        default: {
            return vec3f(1.0, 0.0, 0.0);
        }
    }
}
