import package::{
    color::{Rgba, to_rgba},
    types
};

// TODO: uv mapping
fn get_texture_color(texture: types::GpuTexture) -> vec3<f32> {
    switch texture._type {
        case types::GPUTEXTURE_SOLIDCOLOR: {
            let solid_color = types::unpack_GpuTexture_SolidColor(texture);
            return solid_color.albedo;
        }

        default: {
            return vec3<f32>(0, 0, 0);
        }
    }
}
