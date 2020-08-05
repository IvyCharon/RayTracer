use crate::Object;
use crate::Onb;
use crate::Vec3;
use std::sync::Arc;

pub trait Pdf {
    fn value(&self, dir: Vec3) -> f64;
    fn generate(&self) -> Vec3;
}

pub struct CosPdf {
    uvw: Onb,
}

impl CosPdf {
    pub fn _new(s: Vec3) -> Self {
        Self {
            uvw: Onb::build_from_w(s),
        }
    }
}

impl Pdf for CosPdf {
    fn value(&self, dir: Vec3) -> f64 {
        let co = dir.unit() * self.uvw.w();
        if co <= 0.0 {
            0.0
        } else {
            co / std::f64::consts::PI
        }
    }

    fn generate(&self) -> Vec3 {
        self.uvw.local(Vec3::random_cosine_direction())
    }
}

pub struct HittablePdf {
    o: Vec3,
    ptr: Arc<dyn Object>,
}

impl HittablePdf {
    pub fn new(oo: Vec3, p: Arc<dyn Object>) -> Self {
        Self { o: oo, ptr: p }
    }
}

impl Pdf for HittablePdf {
    fn value(&self, dir: Vec3) -> f64 {
        self.ptr.pdf_value(self.o, dir)
    }

    fn generate(&self) -> Vec3 {
        self.ptr.random(self.o)
    }
}