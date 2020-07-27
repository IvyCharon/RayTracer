use crate::Material;
use crate::Ray;
use crate::Vec3;
use std::sync::Arc;

pub trait Object {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<Hit_record>;
}
pub struct Hit_record {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat: Option<Arc<dyn Material>>,
}

impl Hit_record {
    pub fn set_face_normal(mut self, r: Ray, outward_normal: Vec3) {
        self.front_face = (r.dir * outward_normal) < 0.0;
        if self.front_face {
            self.normal = outward_normal;
        } else {
            self.normal = -outward_normal;
        }
    }

    pub fn new() -> Self {
        Self {
            p: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false,
            mat: Option::None,
        }
    }
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub mat: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(v: Vec3, r: f64, m: Arc<dyn Material>) -> Self {
        Self {
            center: v,
            radius: r,
            mat: m,
        }
    }
}

impl Object for Sphere {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<Hit_record> {
        let oc = r.beg - self.center;
        let a = r.dir.length_squared();
        let half_b: f64 = oc * r.dir;
        let c = oc.length_squared() - self.radius * self.radius;
        let ans = half_b * half_b - a * c;

        if ans > 0.0 {
            let root = ans.sqrt();

            let mut temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                let outward_normal: Vec3 = (r.at(temp.clone()) - self.center) / self.radius;
                let k = (outward_normal * r.dir) < 0.0;
                let mut tmpp = outward_normal;
                if !k {
                    tmpp = -outward_normal;
                }
                return Option::Some(Hit_record {
                    p: r.at(temp.clone()),
                    normal: tmpp,
                    t: temp.clone(),
                    front_face: k,
                    mat: Option::Some(self.mat.clone()),
                });
            }

            temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                let outward_normal: Vec3 = (r.at(temp.clone()) - self.center) / self.radius;
                let k = (outward_normal * r.dir) < 0.0;
                let mut tmpp = outward_normal;
                if !k {
                    tmpp = -outward_normal;
                }
                return Option::Some(Hit_record {
                    p: r.at(temp.clone()),
                    normal: tmpp,
                    t: temp.clone(),
                    front_face: k,
                    mat: Option::Some(self.mat.clone()),
                });
            }
        }
        return Option::None;
    }
}

pub struct Moving_Sphere{
    pub center0: Vec3,
    pub center1: Vec3,
    pub radius: f64,
    pub time0: f64,
    pub time1: f64,
    pub mat: Arc<dyn Material>,
}

impl Moving_Sphere{
    pub fn new(c0: Vec3, c1: Vec3, r: f64, t0: f64, t1: f64, m: Arc<dyn Material>) -> Self{
        Self{
            center0: c0,
            center1: c1,
            radius: r,
            time0: t0,
            time1: t1,
            mat: m,
        }
    }

    pub fn center(&self, t: f64) -> Vec3{
        return self.center0 + (self.center1 - self.center0) * ((t - self.time0) / (self.time1 - self.time0));
    }
}

impl Object for Moving_Sphere{
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<Hit_record>{
        let oc = r.beg - self.center(r.tm);
        let a = r.dir.length_squared();
        let half_b: f64 = oc * r.dir;
        let c = oc.length_squared() - self.radius * self.radius;
        let ans = half_b * half_b - a * c;

        if ans > 0.0 {
            let root = ans.sqrt();

            let mut temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                let outward_normal: Vec3 = (r.at(temp.clone()) - self.center(r.tm)) / self.radius;
                let k = (outward_normal * r.dir) < 0.0;
                let mut tmpp = outward_normal;
                if !k {
                    tmpp = -outward_normal;
                }
                return Option::Some(Hit_record {
                    p: r.at(temp.clone()),
                    normal: tmpp,
                    t: temp.clone(),
                    front_face: k,
                    mat: Option::Some(self.mat.clone()),
                });
            }

            temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                let outward_normal: Vec3 = (r.at(temp.clone()) - self.center(r.tm)) / self.radius;
                let k = (outward_normal * r.dir) < 0.0;
                let mut tmpp = outward_normal;
                if !k {
                    tmpp = -outward_normal;
                }
                return Option::Some(Hit_record {
                    p: r.at(temp.clone()),
                    normal: tmpp,
                    t: temp.clone(),
                    front_face: k,
                    mat: Option::Some(self.mat.clone()),
                });
            }
        }
        return Option::None;
    }
}