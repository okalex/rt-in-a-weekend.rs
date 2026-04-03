struct Onb {
    axes: mat3x3f,
};

fn new_onb(n: vec3f) -> Onb {
    let axis2 = normalize(n);
    var a = vec3f(1, 0, 0);
    if abs(axis2[0]) > 0.9 {
        a = vec3f(0, 1, 0);
    }
    let axis1 = normalize(cross(axis2, a));
    let axis0 = cross(axis2, axis1);

    return Onb(mat3x3f(axis0, axis1, axis2));
}

// Transform from basis coordinates to local space.
fn transform(onb: Onb, v: vec3f) -> vec3f {
    return v[0] * onb.axes[0] + v[1] * onb.axes[1] + v[2] * onb.axes[2];
}

// Transform from local space to basis coordinates.
fn inv_transform(onb: Onb, v: vec3f) -> vec3f {
    return vec3f(dot(v, onb.axes[0]), dot(v, onb.axes[1]), dot(v, onb.axes[2]));
}
