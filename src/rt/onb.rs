use crate::rt::types::Vector;

pub struct Onb {
    axis: [Vector; 3],
}

impl Onb {
    pub fn new(n: &Vector) -> Self {
        let axis2 = n.normalize();
        let a = if axis2.x.abs() > 0.9 {
            Vector::new(0.0, 1.0, 0.0)
        } else {
            Vector::new(1.0, 0.0, 0.0)
        };
        let axis1 = axis2.cross(a).normalize();
        let axis0 = axis2.cross(axis1);
        Self {
            axis: [axis0, axis1, axis2],
        }
    }

    pub fn u(&self) -> Vector {
        self.axis[0]
    }

    pub fn v(&self) -> Vector {
        self.axis[1]
    }

    pub fn w(&self) -> Vector {
        self.axis[2]
    }

    // Transform from basis coordinates to local space.
    pub fn transform(&self, v: Vector) -> Vector {
        v[0] * self.u() + v[1] * self.v() + v[2] * self.w()
    }
}
