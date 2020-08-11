use crate::Vec3;

pub trait Texture {
    fn color(&self, u: f64, v: f64, p: Vec3) -> Vec3;
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3;
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub struct CheckerTexture<T: Texture> {
    pub odd: T,
    pub even: T,
}

impl<T: Texture> CheckerTexture<T> {
    pub fn new(a: T, b: T) -> Self {
        Self { odd: a, even: b }
    }
}

impl<T: Texture> Texture for CheckerTexture<T> {
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
