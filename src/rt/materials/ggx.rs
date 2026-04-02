use crate::util::{
    color::Color,
    types::{Float, PI},
};

pub fn ggx_d(n_dot_h: Float, alpha_sqrd: Float) -> Float {
    let n_dot_h_sqrd = n_dot_h * n_dot_h;
    let denom = n_dot_h_sqrd * (alpha_sqrd - 1.0) + 1.0;
    alpha_sqrd / (PI * denom * denom)
}

pub fn smith_g1(n_dot_o: Float, alpha_sqrd: Float) -> Float {
    let n_dot_o_sqrd = n_dot_o * n_dot_o;
    let denom = n_dot_o + (alpha_sqrd + (1.0 - alpha_sqrd) * n_dot_o_sqrd).sqrt();
    2.0 * n_dot_o / denom
}

pub fn smith_g2(n_dot_i: Float, n_dot_o: Float, alpha_sqrd: Float) -> Float {
    let n_dot_i_sqrd = n_dot_i * n_dot_i;
    let n_dot_o_sqrd = n_dot_o * n_dot_o;
    let a = alpha_sqrd + (1.0 - alpha_sqrd) * n_dot_i_sqrd;
    let b = alpha_sqrd + (1.0 - alpha_sqrd) * n_dot_o_sqrd;
    let denom = n_dot_o * a.sqrt() + n_dot_i * b.sqrt();
    2.0 * n_dot_i * n_dot_o / denom
}

pub fn schlick_fresnel(f0: Color, v_dot_h: Float) -> Color {
    let cos_theta = v_dot_h.clamp(0.0, 1.0);
    f0 + (1.0 - f0) * (1.0 - cos_theta).powf(5.0)
}
