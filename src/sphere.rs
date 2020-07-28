use crate::aabb;
use crate::Material;
use crate::Ray;
use crate::Vec3;
use std::sync::Arc;

const PI: f64 = 3.1415926535897932385;

pub trait Object {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<Hit_record>;
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<aabb>;
}
pub struct Hit_record {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat: Option<Arc<dyn Material>>,
    pub u: f64,
    pub v: f64,
}

pub struct uv {
    u: f64,
    v: f64,
}

impl uv {
    pub fn new(a: f64, b: f64) -> Self {
        Self { u: a, v: b }
    }
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
            u: 0.0,
            v: 0.0,
        }
    }

    pub fn get_sphere_uv(p: Vec3) -> uv {
        let phi = p.z.atan2(p.x);
        let theta = p.y.asin();
        let u = 1.0 - (phi + PI) / (2.0 * PI);
        let v = (theta + PI / 2.0) / PI;
        return uv::new(u, v);
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
                let uv_ =
                    Hit_record::get_sphere_uv((r.at(temp.clone()) - self.center) / self.radius);
                return Option::Some(Hit_record {
                    p: r.at(temp.clone()),
                    normal: tmpp,
                    t: temp.clone(),
                    front_face: k,
                    mat: Option::Some(self.mat.clone()),
                    u: uv_.u,
                    v: uv_.v,
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
                let uv_ =
                    Hit_record::get_sphere_uv((r.at(temp.clone()) - self.center) / self.radius);
                return Option::Some(Hit_record {
                    p: r.at(temp.clone()),
                    normal: tmpp,
                    t: temp.clone(),
                    front_face: k,
                    mat: Option::Some(self.mat.clone()),
                    u: uv_.u,
                    v: uv_.v,
                });
            }
        }
        return Option::None;
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<aabb> {
        return Option::Some(aabb::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius),
        ));
    }
}
