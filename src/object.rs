use crate::HittableList;
use crate::Material;
use crate::Ray;
use crate::Vec3;
use crate::AABB;
use std::sync::Arc;

const PI: f64 = 3.141_592_653_589_793;

pub trait Object {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self) -> Option<AABB>;
}
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat: Option<Arc<dyn Material>>,
    pub u: f64,
    pub v: f64,
}

impl HitRecord {
    pub fn get_sphere_uv(p: Vec3) -> UV {
        let phi = p.z.atan2(p.x);
        let theta = p.y.asin();
        let u = 1.0 - (phi + PI) / (2.0 * PI);
        let v = (theta + PI / 2.0) / PI;
        UV::new(u, v)
    }
}

pub struct UV {
    u: f64,
    v: f64,
}

impl UV {
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
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.beg - self.center;
        let a = r.dir.length_squared();
        let half_b: f64 = oc * r.dir;
        let c = oc.length_squared() - self.radius * self.radius;
        let ans = half_b * half_b - a * c;

        if ans > 0.0 {
            let root = ans.sqrt();

            let mut temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                let outward_normal: Vec3 = (r.at(temp) - self.center) / self.radius;
                let k = (outward_normal * r.dir) < 0.0;
                let mut tmpp = outward_normal;
                if !k {
                    tmpp = -outward_normal;
                }
                let uv_ = HitRecord::get_sphere_uv((r.at(temp) - self.center) / self.radius);
                return Option::Some(HitRecord {
                    p: r.at(temp),
                    normal: tmpp,
                    t: temp,
                    front_face: k,
                    mat: Option::Some(self.mat.clone()),
                    u: uv_.u,
                    v: uv_.v,
                });
            }

            temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                let outward_normal: Vec3 = (r.at(temp) - self.center) / self.radius;
                let k = (outward_normal * r.dir) < 0.0;
                let mut tmpp = outward_normal;
                if !k {
                    tmpp = -outward_normal;
                }
                let uv_ = HitRecord::get_sphere_uv((r.at(temp) - self.center) / self.radius);
                return Option::Some(HitRecord {
                    p: r.at(temp),
                    normal: tmpp,
                    t: temp,
                    front_face: k,
                    mat: Option::Some(self.mat.clone()),
                    u: uv_.u,
                    v: uv_.v,
                });
            }
        }
        Option::None
    }

    fn bounding_box(&self) -> Option<AABB> {
        Option::Some(AABB::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius),
        ))
    }
}

pub struct XYRect {
    mp: Arc<dyn Material>,
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
}

impl XYRect {
    pub fn new(a_: f64, b_: f64, c_: f64, d_: f64, f_: f64, e_: Arc<dyn Material>) -> Self {
        Self {
            mp: e_,
            x0: a_,
            x1: b_,
            y0: c_,
            y1: d_,
            k: f_,
        }
    }
}

impl Object for XYRect {
    fn hit(&self, r: Ray, t0: f64, t1: f64) -> Option<HitRecord> {
        let t_ = (self.k - r.beg.z) / r.dir.z;
        if t_ < t0 || t_ > t1 {
            return Option::None;
        }
        let x = r.beg.x + r.dir.x * t_;
        let y = r.beg.y + r.dir.y * t_;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return Option::None;
        }
        let outward_normal = Vec3::new(0.0, 0.0, 1.0);
        Option::Some(HitRecord {
            p: r.at(t_),
            normal: {
                if (r.dir * outward_normal) < 0.0 {
                    outward_normal
                } else {
                    -outward_normal
                }
            },
            t: t_,
            front_face: (r.dir * outward_normal) < 0.0,
            mat: Option::Some(self.mp.clone()),
            u: (x - self.x0) / (self.x1 - self.x0),
            v: (y - self.y0) / (self.y1 - self.y0),
        })
    }

    fn bounding_box(&self) -> Option<AABB> {
        Option::Some(AABB::new(
            Vec3::new(self.x0, self.y0, self.k - 0.0001),
            Vec3::new(self.x1, self.y1, self.k + 0.0001),
        ))
    }
}

pub struct XZRect {
    mp: Arc<dyn Material>,
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64,
}

impl XZRect {
    pub fn new(a_: f64, b_: f64, c_: f64, d_: f64, f_: f64, e_: Arc<dyn Material>) -> Self {
        Self {
            mp: e_,
            x0: a_,
            x1: b_,
            z0: c_,
            z1: d_,
            k: f_,
        }
    }
}

impl Object for XZRect {
    fn hit(&self, r: Ray, t0: f64, t1: f64) -> Option<HitRecord> {
        let t_ = (self.k - r.beg.y) / r.dir.y;
        if t_ < t0 || t_ > t1 {
            return Option::None;
        }
        let x = r.beg.x + r.dir.x * t_;
        let z = r.beg.z + r.dir.z * t_;
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return Option::None;
        }
        let outward_normal = Vec3::new(0.0, 1.0, 0.0);
        Option::Some(HitRecord {
            p: r.at(t_),
            normal: {
                if (r.dir * outward_normal) < 0.0 {
                    outward_normal
                } else {
                    -outward_normal
                }
            },
            t: t_,
            front_face: (r.dir * outward_normal) < 0.0,
            mat: Option::Some(self.mp.clone()),
            u: (x - self.x0) / (self.x1 - self.x0),
            v: (z - self.z0) / (self.z1 - self.z0),
        })
    }

    fn bounding_box(&self) -> Option<AABB> {
        Option::Some(AABB::new(
            Vec3::new(self.x0, self.k - 0.0001, self.z0),
            Vec3::new(self.x1, self.k + 0.0001, self.z1),
        ))
    }
}

pub struct YZRrect {
    mp: Arc<dyn Material>,
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    k: f64,
}

impl YZRrect {
    pub fn new(a_: f64, b_: f64, c_: f64, d_: f64, f_: f64, e_: Arc<dyn Material>) -> Self {
        Self {
            mp: e_,
            y0: a_,
            y1: b_,
            z0: c_,
            z1: d_,
            k: f_,
        }
    }
}

impl Object for YZRrect {
    fn hit(&self, r: Ray, t0: f64, t1: f64) -> Option<HitRecord> {
        let t_ = (self.k - r.beg.x) / r.dir.x;
        if t_ < t0 || t_ > t1 {
            return Option::None;
        }
        let y = r.beg.y + r.dir.y * t_;
        let z = r.beg.z + r.dir.z * t_;
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return Option::None;
        }
        let outward_normal = Vec3::new(1.0, 0.0, 0.0);
        Option::Some(HitRecord {
            p: r.at(t_),
            normal: {
                if (r.dir * outward_normal) < 0.0 {
                    outward_normal
                } else {
                    -outward_normal
                }
            },
            t: t_,
            front_face: (r.dir * outward_normal) < 0.0,
            mat: Option::Some(self.mp.clone()),
            u: (y - self.y0) / (self.y1 - self.y0),
            v: (z - self.z0) / (self.z1 - self.z0),
        })
    }

    fn bounding_box(&self) -> Option<AABB> {
        Option::Some(AABB::new(
            Vec3::new(self.k - 0.0001, self.y0, self.z0),
            Vec3::new(self.k + 0.0001, self.y1, self.z1),
        ))
    }
}

pub struct Box {
    pub box_min: Vec3,
    pub box_max: Vec3,
    pub sides: HittableList,
}

impl Box {
    pub fn new(mi: Vec3, ma: Vec3, p: Arc<dyn Material>) -> Self {
        let mut wor = HittableList::new();
        wor.add(Arc::new(XYRect::new(
            mi.x,
            ma.x,
            mi.y,
            ma.y,
            mi.z,
            p.clone(),
        )));
        wor.add(Arc::new(XYRect::new(
            mi.x,
            ma.x,
            mi.y,
            ma.y,
            ma.z,
            p.clone(),
        )));
        wor.add(Arc::new(XZRect::new(
            mi.x,
            ma.x,
            mi.z,
            ma.z,
            mi.y,
            p.clone(),
        )));
        wor.add(Arc::new(XZRect::new(
            mi.x,
            ma.x,
            mi.z,
            ma.z,
            ma.y,
            p.clone(),
        )));
        wor.add(Arc::new(YZRrect::new(
            mi.y,
            ma.y,
            mi.z,
            ma.z,
            mi.x,
            p.clone(),
        )));
        wor.add(Arc::new(YZRrect::new(
            mi.y,
            ma.y,
            mi.z,
            ma.z,
            ma.x,
            p.clone(),
        )));

        Self {
            box_min: mi,
            box_max: ma,
            sides: wor,
        }
    }
}

impl Object for Box {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(r, t_min, t_max)
    }
    fn bounding_box(&self) -> Option<AABB> {
        Option::Some(AABB::new(self.box_min, self.box_max))
    }
}
