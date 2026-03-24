import package::{
    types
};

// TODO: uv mapping
fn get_texture_color(texture: types::GpuTexture, u: f32, v: f32, point: vec3f) -> vec3f {
    switch texture._type {
        case types::GPUTEXTURE_SOLIDCOLOR: {
            let solid_color = types::unpack_GpuTexture_SolidColor(texture);
            return solid_color.albedo;
        }

        case types::GPUTEXTURE_CHECKERED {
            let checkered = types::unpack_GpuTexture_Checkered(texture);
            let x_int = i32(floor(checkered.inv_scale * point.x));
            let y_int = i32(floor(checkered.inv_scale * point.y));
            let z_int = i32(floor(checkered.inv_scale * point.z));
            let is_even = (x_int + y_int + z_int) % 2 == 0;
            if is_even {
                return checkered.even;
            } else {
                return checkered.odd;
            }
        }

        default: {
            return vec3f(0, 0, 0);
        }
    }
}
