use crate::Hit_record;
use crate::Ray;
use crate::Vec3;

pub trait Material{
    fn scatter(&self, r_in: Ray, rec: Hit_record) -> Sca_ret;
}

pub struct Sca_ret{
    pub scattered: Ray,
    pub attenustion: Vec3,
    pub jud: bool,
}

impl Sca_ret{
    pub fn new(r: Ray, v: Vec3, j: bool) -> Self{
        Self{
            scattered: r,
            attenustion: v,
            jud: j,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Lambertian{
    pub albedo: Vec3,
}

impl Lambertian{
    pub fn new(v: Vec3) -> Self{
        Lambertian{
            albedo: v,
        }
    }
}

impl Material for Lambertian{
    fn scatter(&self, r_in: Ray, rec: Hit_record) -> Sca_ret{
        let sca_dir = rec.normal + Vec3::random_unit_vec();
        return Sca_ret::new(Ray::new(rec.p, sca_dir.clone()), self.albedo, true);
    }
}

pub struct Metal{
    pub albedo: Vec3,
}

impl Metal{
    pub fn new(v: Vec3) -> Self{
        Metal{
            albedo: v,
        }
    }
}

impl Material for Metal{
    fn scatter(&self, r_in: Ray, rec: Hit_record) -> Sca_ret{
        let reflected = Vec3::reflect(r_in.dir.unit(), rec.normal);
        let sca = Ray::new(rec.p, reflected.clone());
        let at = self.albedo;
        return Sca_ret::new(sca.clone(), at, ((sca.clone()).dir * rec.normal) > 0.0);
    }
}
