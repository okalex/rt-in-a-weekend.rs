import package::{
    constants::PI,
};

fn ggx_d(n_dot_h: f32, alpha_sqrd: f32) -> f32 {
    let n_dot_h_sqrd = n_dot_h * n_dot_h;
    let denom = n_dot_h_sqrd * (alpha_sqrd - 1.0) + 1.0;
    return alpha_sqrd / (PI * denom * denom);
}

fn smith_g1(n_dot_o: f32, alpha_sqrd: f32) -> f32 {
    let n_dot_o_sqrd = n_dot_o * n_dot_o;
    let denom = n_dot_o + sqrt(alpha_sqrd + (1.0 - alpha_sqrd) * n_dot_o_sqrd);
    return 2.0 * n_dot_o / denom;
}

fn smith_g2(n_dot_i: f32, n_dot_o: f32, alpha_sqrd: f32) -> f32 {
    let n_dot_i_sqrd = n_dot_i * n_dot_i;
    let n_dot_o_sqrd = n_dot_o * n_dot_o;
    let a = alpha_sqrd + (1.0 - alpha_sqrd) * n_dot_i_sqrd;
    let b = alpha_sqrd + (1.0 - alpha_sqrd) * n_dot_o_sqrd;
    let denom = n_dot_o * sqrt(a) + n_dot_i * sqrt(b);
    return 2.0 * n_dot_i * n_dot_o / denom;
}

fn schlick_fresnel(f0: vec3f, v_dot_h: f32) -> vec3f {
    let cos_theta = clamp(v_dot_h, 0.0, 1.0);
    return f0 + (1.0 - f0) * pow(1.0 - cos_theta, 5.0);
}
