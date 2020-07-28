use crate::Vec3;
use std::sync::Arc;

pub trait Texture {
    fn color(&self, u: f64, v: f64, p: Vec3) -> Vec3;
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3;
}

pub struct solid_color {
    color: Vec3,
}

impl solid_color {
    pub fn new(a: Vec3) -> Self {
        Self { color: a }
    }
}

impl Texture for solid_color {
    fn color(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        self.color
    }

    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        return self.color;
    }
}

pub struct checker_texture {
    pub odd: Arc<dyn Texture>,
    pub even: Arc<dyn Texture>,
}

impl checker_texture {
    pub fn new(a: Vec3, b: Vec3) -> Self {
        Self {
            odd: Arc::new(solid_color::new(a)),
            even: Arc::new(solid_color::new(b)),
        }
    }
}

impl Texture for checker_texture {
    fn color(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines < 0.0 {
            return self.odd.value(u, v, p);
        } else {
            return self.even.value(u, v, p);
        }
    }

    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        return self.color(u, v, p);
    }
}
