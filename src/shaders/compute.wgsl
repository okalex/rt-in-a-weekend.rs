#import types;

struct Rgba {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
};

struct Ray {
    orig: vec3<f32>,
    dir: vec3<f32>,
};

struct Interval {
    min: f32,
    max: f32,
};

struct HitRecord {
    is_hit: bool,
    point: vec3<f32>,
    normal: vec3<f32>,
    t: f32,
    mat_idx: u32,
};

@group(0) @binding(0) var<storage, read_write> output: array<Rgba>;
@group(0) @binding(1) var<uniform> gpu_meta: types::GpuMeta;
@group(0) @binding(2) var<storage, read> objects: types::GpuObjects;
@group(0) @binding(3) var<storage, read> materials: types::GpuMaterials;

@compute
@workgroup_size(16, 16, 1)
fn main(
    @builtin(global_invocation_id) global_id: vec3<u32>
) {
    let x = global_id.x;
    let y = global_id.y;

    if (x >= gpu_meta.width || y >= gpu_meta.height) {
        return;
    }

    let idx = gpu_meta.width * y + x;

    let camera_ray = get_ray(x, y);
    let ray_color = get_ray_color(camera_ray);

    // let r = f32(x) / f32(gpu_meta.width);
    // let g = f32(y) / f32(gpu_meta.height);
    // let b = sqrt(r * r + g * g);
    // let grad_color = Rgba(r, g, b, 1.0);
    
    output[idx] = ray_color;
}

fn to_rgba(color: vec3<f32>) -> Rgba {
    return Rgba(color[0], color[1], color[2], 1.0);
}

fn get_ray(x: u32, y: u32) -> Ray {
    let pixel_sample = pixel_loc(gpu_meta.viewport, x, y);
    let origin = gpu_meta.camera.position;
    let direction = pixel_sample - origin;
    return Ray(origin, direction);
}

fn get_ray_color(ray: Ray) -> Rgba {
    let hit_record = hit(ray, Interval(0.001, 1e30));
    if (hit_record.is_hit) {
        let material = get_material(hit_record.mat_idx);
        return get_material_color(material);
    } else {
        return to_rgba(gpu_meta.background);
    }
}

fn get_material(mat_idx: u32) -> types::GpuMaterial {
    return materials.materials[mat_idx];
}

// TODO: uv mapping
fn get_material_color(material: types::GpuMaterial) -> Rgba {
    switch material._type {
        case types::GPUMATERIAL_LAMBERTIAN: { 
            let lambertian = types::unpack_GpuMaterial_Lambertian(material);
            return get_texture_color(lambertian.texture);
        }

        default: {
            return Rgba(0, 0, 0, 0);
        }
    }
}

// TODO: uv mapping
fn get_texture_color(texture: types::GpuTexture) -> Rgba {
    switch texture._type {
        case types::GPUTEXTURE_SOLIDCOLOR: {
            let solid_color = types::unpack_GpuTexture_SolidColor(texture);
            return to_rgba(solid_color.albedo);
        }

        default: {
            return Rgba(0, 0, 0, 0);
        }
    }
}

fn hit(ray: Ray, ray_t: Interval) -> HitRecord {
    var hit_record = no_hit();
    var closest = ray_t.max;
    for (var i: u32 = 0; i < arrayLength(&objects.objects); i = i + 1) {
        let object = objects.objects[i];
        let temp_record = hit_object(ray, Interval(ray_t.min, closest), object);
        if (temp_record.is_hit) {
            hit_record = temp_record;
            closest = hit_record.t;
        }
    }
    return hit_record;
}

fn hit_object(ray: Ray, ray_t: Interval, object: types::GpuShape) -> HitRecord {
    switch object._type {
        case types::GPUSHAPE_SPHERE: {
            let sphere = types::unpack_GpuShape_Sphere(object);
            return hit_sphere(ray, ray_t, sphere);
        }

        default: {
            return no_hit();
        }
    }
}

fn hit_sphere(ray: Ray, ray_t: Interval, sphere: types::GpuShape_Sphere) -> HitRecord {
    let oc = sphere.center - ray.orig;
    let a = dot(ray.dir, ray.dir);
    let b = -2.0 * dot(ray.dir, oc);
    let c = dot(oc, oc) - sphere.radius * sphere.radius;
    let discriminant = b * b - 4 * a * c;

    if (discriminant < 0) {
        return no_hit();
    }

    let sqrtd = sqrt(discriminant);

    // Try the nearer root first, then the farther one
    var root = (-b - sqrtd) / (2.0 * a);
    if (root < ray_t.min || root > ray_t.max) {
        root = (-b + sqrtd) / (2.0 * a);
        if (root < ray_t.min || root > ray_t.max) {
            return no_hit();
        }
    }

    let hit_point = ray_at(ray, root);
    let normal = (hit_point - sphere.center) / sphere.radius;

    return HitRecord(
        true,
        hit_point,
        normal,
        root,
        sphere.mat_idx,
    );
}

fn no_hit() -> HitRecord {
    return HitRecord(false, vec3(0,0,0), vec3(0,0,0), 0, 0);
}

fn pixel_loc(viewport: types::GpuViewport, x: u32, y: u32) -> vec3<f32> {
    return viewport.pixel00_loc + viewport.delta_u * f32(x) + viewport.delta_v * f32(y);
}

fn ray_at(ray: Ray, t: f32) -> vec3<f32> {
    return ray.orig + ray.dir * t;
}
