use crate::Ray;
use crate::Vec3;

pub trait Object {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<hit_record>;
}
#[derive(Clone, Debug, PartialEq, Copy)]
pub struct hit_record {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl hit_record {
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
        }
    }
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(v: Vec3, r: f64) -> Self {
        Self {
            center: v,
            radius: r,
        }
    }
}

impl Object for Sphere {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<hit_record> {
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
                return Option::Some(hit_record {
                    p: r.at(temp.clone()),
                    normal: tmpp,
                    t: temp.clone(),
                    front_face: k,
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
                return Option::Some(hit_record {
                    p: r.at(temp.clone()),
                    normal: tmpp,
                    t: temp.clone(),
                    front_face: k,
                });
            }
        }
        return Option::None;
    }
}
