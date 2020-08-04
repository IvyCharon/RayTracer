use crate::HitRecord;
use crate::Ray;
use crate::SolidColor;
use crate::Texture;
use crate::Vec3;
use std::sync::Arc;
extern crate rand;

pub trait Material {
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> ScaRet;
    fn emitted(&self, _u: f64, _v: f64, _p: Vec3) -> Vec3 {
        Vec3::zero()
    }
    fn scattering_pdf(&self, _r_in: Ray, _rec: &HitRecord, _scattered: Ray) -> f64 {
        0.0
    }
}

pub struct ScaRet {
    pub scattered: Ray,
    pub attenustion: Vec3,
    pub pdf: f64,
    pub jud: bool,
}

impl ScaRet {
    pub fn new(r: Ray, v: Vec3, j: bool) -> Self {
        Self {
            scattered: r,
            attenustion: v,
            pdf: 0.0,
            jud: j,
        }
    }

    pub fn new_(r: Ray, v: Vec3, p: f64, j: bool) -> Self {
        Self {
            scattered: r,
            attenustion: v,
            pdf: p,
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
            albedo: Arc::new(SolidColor::new(v)),
        }
    }

    pub fn new_(v: Arc<dyn Texture>) -> Self {
        Lambertian { albedo: v }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: Ray, rec: &HitRecord) -> ScaRet {
        //let sca_dir = rec.normal + Vec3::random_unit_vec();
        let dd = Vec3::random_in_hemisphere(&rec.normal);
        ScaRet::new_(
            Ray::new(rec.p, dd.unit()),
            self.albedo.value(rec.u, rec.v, rec.p),
            0.5 / std::f64::consts::PI,
            true,
        )
    }

    fn scattering_pdf(&self, _r_in: Ray, rec: &HitRecord, scattered: Ray) -> f64 {
        let co = rec.normal * (scattered.dir.unit());
        if co < 0.0 {
            0.0
        } else {
            co / std::f64::consts::PI
        }
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
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> ScaRet {
        let reflected = Vec3::reflect(r_in.dir.unit(), rec.normal);
        let sca = Ray::new(rec.p, reflected + Vec3::random_in_unit_sphere() * self.fuzz);
        let at = self.albedo;
        ScaRet::new(sca, at, (sca.dir * rec.normal) > 0.0)
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
        let mut r0 = (1.0 - ri) / (1.0 + ri);
        r0 *= r0;
        r0 + (1.0 - r0) * (1.0 - cos).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> ScaRet {
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
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        if eta * sin_theta > 1.0 {
            let refl = Vec3::reflect(r_in.dir.unit(), rec.normal);
            return ScaRet::new(Ray::new(rec.p, refl), Vec3::new(1.0, 1.0, 1.0), true);
        }
        let rp = Dielectric::schlick(cos_theta, eta);
        if rand::random::<f64>() < rp {
            let refl = Vec3::reflect(r_in.dir.unit(), rec.normal);
            return ScaRet::new(Ray::new(rec.p, refl), Vec3::new(1.0, 1.0, 1.0), true);
        }
        let refr = Vec3::refract(r_in.dir.unit(), rec.normal, eta);
        ScaRet::new(Ray::new(rec.p, refr), Vec3::new(1.0, 1.0, 1.0), true)
    }
}

pub struct DiffuseLight {
    pub emit: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(m: Arc<dyn Texture>) -> Self {
        Self { emit: m }
    }

    pub fn new_(p: Vec3) -> Self {
        Self {
            emit: Arc::new(SolidColor::new(p)),
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _r_in: Ray, _rec: &HitRecord) -> ScaRet {
        ScaRet {
            scattered: Ray::new(Vec3::zero(), Vec3::zero()),
            attenustion: Vec3::zero(),
            pdf: 0.0,
            jud: false,
        }
    }

    fn emitted(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        self.emit.value(u, v, p)
    }
}
