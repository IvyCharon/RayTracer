use crate::Object;
use crate::Onb;
use crate::Vec3;

pub trait Pdf {
    fn value(&self, dir: Vec3) -> f64;
    fn generate(&self) -> Vec3;
}

pub struct CosPdf {
    uvw: Onb,
}

impl CosPdf {
    pub fn new(s: Vec3) -> Self {
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

pub struct HittablePdf<T: Object> {
    o: Vec3,
    ptr: T,
}

impl<'a, T: Object> HittablePdf<T> {
    pub fn new(oo: Vec3, p: T) -> Self {
        Self { o: oo, ptr: p }
    }
}

impl<T: Object> Pdf for HittablePdf<T> {
    fn value(&self, dir: Vec3) -> f64 {
        self.ptr.pdf_value(self.o, dir)
    }

    fn generate(&self) -> Vec3 {
        self.ptr.random(self.o)
    }
}

pub struct MixturePdf<'a, T: Pdf> {
    p0: T,
    p1: &'a dyn Pdf,
}

impl<'a, T: Pdf> MixturePdf<'a, T> {
    pub fn new(pp0: T, pp1: &'a dyn Pdf) -> Self {
        Self { p0: pp0, p1: pp1 }
    }
}

impl<'a, T: Pdf> Pdf for MixturePdf<'a, T> {
    fn value(&self, dir: Vec3) -> f64 {
        0.5 * self.p0.value(dir) + 0.5 * self.p1.value(dir)
    }
    fn generate(&self) -> Vec3 {
        let tmp = rand::random::<f64>();
        if tmp < 0.5 {
            self.p0.generate()
        } else {
            self.p1.generate()
        }
    }
}
