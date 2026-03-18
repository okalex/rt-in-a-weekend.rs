struct Onb {
    axes: mat3x3<f32>,
};

fn new_onb(n: vec3<f32>) -> Onb {
    let axis2 = normalize(n);
    var a = vec3<f32>(1, 0, 0);
    if (abs(axis2[0]) > 0.9) {
        a = vec3<f32>(0, 1, 0);
    }
    let axis1 = normalize(cross(axis2, a));
    let axis0 = cross(axis2, axis1);

    return Onb(mat3x3<f32>(axis0, axis1, axis2));
}

fn transform_onb(onb: Onb, v: vec3<f32>) -> vec3<f32> {
    return v[0] * onb.axes[0] + v[1] * onb.axes[1] + v[2] * onb.axes[2];
}
