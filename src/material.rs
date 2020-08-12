use crate::CosPdf;
use crate::HitRecord;
use crate::Pdf;
use crate::Ray;
use crate::Texture;
use crate::Vec3;
extern crate rand;

pub trait Material {
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> ScaRet;
    fn scatter_(&self, r_in: Ray, rec: &HitRecord) -> ScaRet_;
    fn emitted(&self, _rec: &HitRecord, _u: f64, _v: f64, _p: Vec3) -> Vec3 {
        Vec3::zero()
    }
    fn scattering_pdf(&self, _r_in: Ray, _rec: &HitRecord, _scattered: Ray) -> f64 {
        0.0
    }
}

pub struct ScaRet {
    pub scattered: Ray,
    pub attenustion: Vec3,
    pub pdf_ptr: Option<Box<dyn Pdf>>,
    pub is_specular: bool,
    pub jud: bool,
}

impl ScaRet {
    pub fn new(r: Ray, v: Vec3, ip: bool, j: bool) -> Self {
        Self {
            scattered: r,
            attenustion: v,
            pdf_ptr: Option::None,
            is_specular: ip,
            jud: j,
        }
    }

    pub fn _new_(r: Ray, v: Vec3, ip: bool, j: bool) -> Self {
        Self {
            scattered: r,
            attenustion: v,
            pdf_ptr: Option::None,
            is_specular: ip,
            jud: j,
        }
    }
}

pub struct ScaRet_ {
    pub scattered: Ray,
    pub attenustion: Vec3,
    pub jud: bool,
}

impl ScaRet_ {
    pub fn new(r: Ray, v: Vec3, j: bool) -> Self {
        Self {
            scattered: r,
            attenustion: v,
            jud: j,
        }
    }
}

#[derive(Clone, Debug, Copy)]
pub struct NoMaterial;

impl Material for NoMaterial {
    fn scatter(&self, _r_in: Ray, _rec: &HitRecord) -> ScaRet {
        panic!("no material");
    }

    fn emitted(&self, _rec: &HitRecord, _u: f64, _v: f64, _p: Vec3) -> Vec3 {
        panic!("no material");
    }

    fn scatter_(&self, _r_in: Ray, _rec: &HitRecord) -> ScaRet_ {
        panic!("no material!");
    }
}

#[derive(Clone, Debug, Copy)]
pub struct Lambertian<T: Texture> {
    pub albedo: T,
}

impl<T: Texture> Lambertian<T> {
    pub fn new(v: T) -> Self {
        Lambertian { albedo: v }
    }
}

impl<T: Texture> Material for Lambertian<T> {
    fn scatter(&self, _r_in: Ray, rec: &HitRecord) -> ScaRet {
        ScaRet {
            scattered: Ray::new(Vec3::zero(), Vec3::zero()),
            attenustion: self.albedo.value(rec.u, rec.v, rec.p),
            pdf_ptr: Option::Some(Box::new(CosPdf::new(rec.normal))),
            is_specular: false,
            jud: true,
        }
    }

    fn scattering_pdf(&self, _r_in: Ray, rec: &HitRecord, scattered: Ray) -> f64 {
        let co = rec.normal * (scattered.dir.unit());
        if co < 0.0 {
            0.0
        } else {
            co / std::f64::consts::PI
        }
    }

    fn scatter_(&self, _r_in: Ray, rec: &HitRecord) -> ScaRet_ {
        let sca_dir = rec.normal + Vec3::random_unit_vec();
        ScaRet_::new(
            Ray::new(rec.p, sca_dir),
            self.albedo.value(rec.u, rec.v, rec.p),
            true,
        )
    }
}

#[derive(Clone, Debug)]
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
        ScaRet {
            scattered: Ray::new(rec.p, reflected + Vec3::random_in_unit_sphere() * self.fuzz),
            attenustion: self.albedo,
            pdf_ptr: Option::None,
            is_specular: true,
            jud: true,
        }
    }

    fn scatter_(&self, r_in: Ray, rec: &HitRecord) -> ScaRet_ {
        let reflected = Vec3::reflect(r_in.dir.unit(), rec.normal);
        let sca = Ray::new(rec.p, reflected + Vec3::random_in_unit_sphere() * self.fuzz);
        let at = self.albedo;
        ScaRet_::new(sca, at, (sca.dir * rec.normal) > 0.0)
    }
}

#[derive(Clone, Debug)]
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
            return ScaRet::new(Ray::new(rec.p, refl), Vec3::new(1.0, 1.0, 1.0), true, true);
        }
        let rp = Dielectric::schlick(cos_theta, eta);
        if rand::random::<f64>() < rp {
            let refl = Vec3::reflect(r_in.dir.unit(), rec.normal);
            return ScaRet::new(Ray::new(rec.p, refl), Vec3::new(1.0, 1.0, 1.0), true, true);
        }
        let refr = Vec3::refract(r_in.dir.unit(), rec.normal, eta);
        ScaRet::new(Ray::new(rec.p, refr), Vec3::new(1.0, 1.0, 1.0), true, true)
    }

    fn scatter_(&self, r_in: Ray, rec: &HitRecord) -> ScaRet_ {
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
            return ScaRet_::new(Ray::new(rec.p, refl), Vec3::new(1.0, 1.0, 1.0), true);
        }
        let rp = Dielectric::schlick(cos_theta, eta);
        if rand::random::<f64>() < rp {
            let refl = Vec3::reflect(r_in.dir.unit(), rec.normal);
            return ScaRet_::new(Ray::new(rec.p, refl), Vec3::new(1.0, 1.0, 1.0), true);
        }
        let refr = Vec3::refract(r_in.dir.unit(), rec.normal, eta);
        ScaRet_::new(Ray::new(rec.p, refr), Vec3::new(1.0, 1.0, 1.0), true)
    }
}

#[derive(Clone, Debug)]
pub struct DiffuseLight<T: Texture> {
    pub emit: T,
}

impl<T: Texture> DiffuseLight<T> {
    pub fn new(m: T) -> Self {
        Self { emit: m }
    }
}

impl<T: Texture> Material for DiffuseLight<T> {
    fn scatter(&self, _r_in: Ray, _rec: &HitRecord) -> ScaRet {
        ScaRet {
            scattered: Ray::new(Vec3::zero(), Vec3::zero()),
            attenustion: Vec3::zero(),
            pdf_ptr: Option::None,
            is_specular: false,
            jud: false,
        }
    }

    fn emitted(&self, _rec: &HitRecord, u: f64, v: f64, p: Vec3) -> Vec3 {
        self.emit.value(u, v, p)
    }

    fn scatter_(&self, _r_in: Ray, _rec: &HitRecord) -> ScaRet_ {
        ScaRet_ {
            scattered: Ray::new(Vec3::zero(), Vec3::zero()),
            attenustion: Vec3::zero(),
            jud: false,
        }
    }
}
