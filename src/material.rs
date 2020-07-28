use crate::checker_texture;
use crate::solid_color;
use crate::Hit_record;
use crate::Ray;
use crate::Texture;
use crate::Vec3;
use std::sync::Arc;
extern crate rand;
use rand::Rng;

pub trait Material {
    fn scatter(&self, r_in: Ray, rec: &Hit_record) -> Sca_ret;
}

pub struct Sca_ret {
    pub scattered: Ray,
    pub attenustion: Vec3,
    pub jud: bool,
}

impl Sca_ret {
    pub fn new(r: Ray, v: Vec3, j: bool) -> Self {
        Self {
            scattered: r,
            attenustion: v,
            jud: j,
        }
    }
}

pub struct Lambertian {
    pub albedo: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(v: Vec3) -> Self {
        Lambertian {
            albedo: Arc::new(solid_color::new(v)),
        }
    }

    pub fn new_(v: Arc<dyn Texture>) -> Self {
        Lambertian { albedo: v }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: Ray, rec: &Hit_record) -> Sca_ret {
        let sca_dir = rec.normal + Vec3::random_unit_vec();
        return Sca_ret::new(
            Ray::new(rec.p, sca_dir.clone()),
            self.albedo.value(rec.u, rec.v, rec.p),
            true,
        );
    }
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(v: Vec3, f: f64) -> Self {
        Metal {
            albedo: v,
            fuzz: {
                if f <= 1.0 {
                    f
                } else {
                    1.0
                }
            },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: Ray, rec: &Hit_record) -> Sca_ret {
        let reflected = Vec3::reflect(r_in.dir.unit(), rec.normal);
        let sca = Ray::new(
            rec.p,
            reflected.clone() + Vec3::random_in_unit_sphere() * self.fuzz,
        );
        let at = self.albedo;
        return Sca_ret::new(sca.clone(), at, ((sca.clone()).dir * rec.normal) > 0.0);
    }
}

pub struct Dielectric {
    pub ref_idx: f64,
}

impl Dielectric {
    pub fn new(x: f64) -> Self {
        Self { ref_idx: x }
    }

    pub fn schlick(cos: f64, ri: f64) -> f64 {
        let mut r0 = (1.0 - ri.clone()) / (1.0 + ri.clone());
        r0 *= r0.clone();
        return r0.clone()
            + (1.0 - r0.clone())
                * (1.0 - cos.clone())
                * (1.0 - cos.clone())
                * (1.0 - cos.clone())
                * (1.0 - cos.clone())
                * (1.0 - cos.clone());
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: Ray, rec: &Hit_record) -> Sca_ret {
        let eta: f64 = {
            if rec.front_face {
                1.0 / self.ref_idx
            } else {
                self.ref_idx
            }
        };
        let cos_theta = {
            if -r_in.dir.unit() * rec.normal > 1.0 {
                1.0
            } else {
                -r_in.dir.unit() * rec.normal
            }
        };
        let sin_theta = (1.0 - cos_theta.clone() * cos_theta.clone()).sqrt();
        if eta.clone() * sin_theta > 1.0 {
            let refl = Vec3::reflect(r_in.dir.unit(), rec.normal);
            return Sca_ret::new(Ray::new(rec.p, refl), Vec3::new(1.0, 1.0, 1.0), true);
        }
        let rp = Dielectric::schlick(cos_theta.clone(), eta.clone());
        if rand::random::<f64>() < rp {
            let refl = Vec3::reflect(r_in.dir.unit(), rec.normal);
            return Sca_ret::new(Ray::new(rec.p, refl), Vec3::new(1.0, 1.0, 1.0), true);
        }
        let refr = Vec3::refract(r_in.dir.unit(), rec.normal, eta);
        return Sca_ret::new(Ray::new(rec.p, refr), Vec3::new(1.0, 1.0, 1.0), true);
    }
}
