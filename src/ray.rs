use crate::Vec3;

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Ray {
    pub beg: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(p: Vec3, d: Vec3) -> Self {
        Self { beg: p, dir: d }
    }

    pub fn at(self, t: f64) -> Vec3 {
        self.dir * t + self.beg
    }
}
