use crate::Vec3;
use std::sync::Arc;

pub trait Texture {
    fn color(&self, u: f64, v: f64, p: Vec3) -> Vec3;
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3;
}

pub struct SolidColor {
    color: Vec3,
}

impl SolidColor {
    pub fn new(a: Vec3) -> Self {
        Self { color: a }
    }
}

impl Texture for SolidColor {
    fn color(&self, _u: f64, _v: f64, _p: Vec3) -> Vec3 {
        self.color
    }

    fn value(&self, _u: f64, _v: f64, _p: Vec3) -> Vec3 {
        self.color
    }
}

pub struct CheckerTexture {
    pub odd: Arc<dyn Texture>,
    pub even: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(a: Vec3, b: Vec3) -> Self {
        Self {
            odd: Arc::new(SolidColor::new(a)),
            even: Arc::new(SolidColor::new(b)),
        }
    }
}

impl Texture for CheckerTexture {
    fn color(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }

    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        self.color(u, v, p)
    }
}
