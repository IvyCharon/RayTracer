use crate::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Onb {
    pub axis0: Vec3,
    pub axis1: Vec3,
    pub axis2: Vec3,
}

impl Onb {
    pub fn _u(self) -> Vec3 {
        self.axis0
    }

    pub fn _v(self) -> Vec3 {
        self.axis1
    }

    pub fn w(self) -> Vec3 {
        self.axis2
    }

    pub fn local(&self, a: Vec3) -> Vec3 {
        self.axis0 * a.x + self.axis1 * a.y + self.axis2 * a.z
    }

    pub fn build_from_w(n: Vec3) -> Self {
        let a2t = n.unit();
        let a = {
            if a2t.x > 0.9 || a2t.x < -0.9 {
                Vec3::new(0.0, 1.0, 0.0)
            } else {
                Vec3::new(1.0, 0.0, 0.0)
            }
        };
        let a1t = Vec3::cross(a2t, a).unit();
        let a0t = Vec3::cross(a2t, a1t);
        Self {
            axis0: a0t,
            axis1: a1t,
            axis2: a2t,
        }
    }
}
