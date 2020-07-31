use crate::aabb;
use crate::Material;
use crate::Ray;
use crate::Vec3;
use crate::Hittable_list;
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

pub struct uv {
    u: f64,
    v: f64,
}

impl uv {
    pub fn new(a: f64, b: f64) -> Self {
        Self { u: a, v: b }
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

pub struct xy_rect {
    mp: Arc<dyn Material>,
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
}

impl xy_rect {
    pub fn new(a: f64, b: f64, c: f64, d: f64, f: f64, e: Arc<dyn Material>) -> Self {
        Self {
            mp: e,
            x0: a,
            x1: b,
            y0: c,
            y1: d,
            k: f,
        }
    }
}

impl Object for xy_rect {
    fn hit(&self, r: Ray, t0: f64, t1: f64) -> Option<Hit_record> {
        let t = (self.k - r.beg.z) / r.dir.z;
        if t < t0 || t > t1 {
            return Option::None;
        }
        let x = r.beg.x + r.dir.x * t;
        let y = r.beg.y + r.dir.y * t;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return Option::None;
        }
        let outward_normal = Vec3::new(0.0, 0.0, 1.0);
        Option::Some(Hit_record {
            p: r.at(t),
            normal: {
                if (r.dir * outward_normal) < 0.0 {
                    outward_normal
                } else {
                    -outward_normal
                }
            },
            t: t,
            front_face: (r.dir * outward_normal) < 0.0,
            mat: Option::Some(self.mp.clone()),
            u: (x - self.x0) / (self.x1 - self.x0),
            v: (y - self.y0) / (self.y1 - self.y0),
        })
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<aabb> {
        return Option::Some(aabb::new(
            Vec3::new(self.x0, self.y0, self.k - 0.0001),
            Vec3::new(self.x1, self.y1, self.k + 0.0001),
        ));
    }
}

pub struct xz_rect {
    mp: Arc<dyn Material>,
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64,
}

impl xz_rect {
    pub fn new(a: f64, b: f64, c: f64, d: f64, f: f64, e: Arc<dyn Material>) -> Self {
        Self {
            mp: e,
            x0: a,
            x1: b,
            z0: c,
            z1: d,
            k: f,
        }
    }
}

impl Object for xz_rect {
    fn hit(&self, r: Ray, t0: f64, t1: f64) -> Option<Hit_record> {
        let t = (self.k - r.beg.y) / r.dir.y;
        if t < t0 || t > t1 {
            return Option::None;
        }
        let x = r.beg.x + r.dir.x * t;
        let z = r.beg.z + r.dir.z * t;
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return Option::None;
        }
        let outward_normal = Vec3::new(0.0, 1.0, 0.0);
        Option::Some(Hit_record {
            p: r.at(t),
            normal: {
                if (r.dir * outward_normal) < 0.0 {
                    outward_normal
                } else {
                    -outward_normal
                }
            },
            t: t,
            front_face: (r.dir * outward_normal) < 0.0,
            mat: Option::Some(self.mp.clone()),
            u: (x - self.x0) / (self.x1 - self.x0),
            v: (z - self.z0) / (self.z1 - self.z0),
        })
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<aabb> {
        return Option::Some(aabb::new(
            Vec3::new(self.x0, self.k - 0.0001, self.z0),
            Vec3::new(self.x1, self.k + 0.0001, self.z1),
        ));
    }
}

pub struct yz_rect {
    mp: Arc<dyn Material>,
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    k: f64,
}

impl yz_rect {
    pub fn new(a: f64, b: f64, c: f64, d: f64, f: f64, e: Arc<dyn Material>) -> Self {
        Self {
            mp: e,
            y0: a,
            y1: b,
            z0: c,
            z1: d,
            k: f,
        }
    }
}

impl Object for yz_rect {
    fn hit(&self, r: Ray, t0: f64, t1: f64) -> Option<Hit_record> {
        let t = (self.k - r.beg.x) / r.dir.x;
        if t < t0 || t > t1 {
            return Option::None;
        }
        let y = r.beg.y + r.dir.y * t;
        let z = r.beg.z + r.dir.z * t;
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return Option::None;
        }
        let outward_normal = Vec3::new(1.0, 0.0, 0.0);
        Option::Some(Hit_record {
            p: r.at(t),
            normal: {
                if (r.dir * outward_normal) < 0.0 {
                    outward_normal
                } else {
                    -outward_normal
                }
            },
            t: t,
            front_face: (r.dir * outward_normal) < 0.0,
            mat: Option::Some(self.mp.clone()),
            u: (y - self.y0) / (self.y1 - self.y0),
            v: (z - self.z0) / (self.z1 - self.z0),
        })
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<aabb> {
        return Option::Some(aabb::new(
            Vec3::new(self.k - 0.0001, self.y0, self.z0),
            Vec3::new(self.k + 0.0001, self.y1, self.z1),
        ));
    }
}

pub struct box_ {
    pub box_min: Vec3,
    pub box_max: Vec3,
    pub sides: Hittable_list,
}

impl box_ {
    pub fn new(mi: Vec3, ma: Vec3, p: Arc<dyn Material>) -> Self {
        let mut wor = Hittable_list::new();
        wor.add(Arc::new(xy_rect::new(mi.x, ma.x, mi.y, ma.y, mi.z, p.clone())));
        wor.add(Arc::new(xy_rect::new(mi.x, ma.x, mi.y, ma.y, ma.z, p.clone())));
        wor.add(Arc::new(xz_rect::new(mi.x, ma.x, mi.z, ma.z, mi.y, p.clone())));
        wor.add(Arc::new(xz_rect::new(mi.x, ma.x, mi.z, ma.z, ma.y, p.clone())));
        wor.add(Arc::new(yz_rect::new(mi.y, ma.y, mi.z, ma.z, mi.x, p.clone())));
        wor.add(Arc::new(yz_rect::new(mi.y, ma.y, mi.z, ma.z, ma.x, p.clone())));

        Self{
            box_min: mi,
            box_max: ma,
            sides: wor,
        }
    }
}

impl Object for box_ {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<Hit_record> {
        return self.sides.hit(r, t_min, t_max);
    }
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<aabb> {
        return Option::Some(aabb::new(self.box_min, self.box_max));
    }
}