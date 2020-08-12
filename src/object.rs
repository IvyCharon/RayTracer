use crate::HittableList;
use crate::Material;
use crate::Onb;
use crate::Ray;
use crate::Vec3;
use crate::AABB;
use std::sync::Arc;
extern crate rand;
use rand::Rng;

const INFINITY: f64 = 1e15;

pub trait Object {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self) -> Option<AABB>;

    fn pdf_value(&self, _o: Vec3, _d: Vec3) -> f64 {
        panic!("unimplemented!")
    }

    fn random(&self, _v: Vec3) -> Vec3 {
        panic!("unimplemented!")
    }
}
pub struct HitRecord<'a> {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat: Option<&'a dyn Material>,
    pub u: f64,
    pub v: f64,
}

impl<'a> HitRecord<'a> {
    pub fn get_sphere_uv(p: Vec3) -> UV {
        let phi = p.z.atan2(p.x);
        let theta = p.y.asin();
        let u = 1.0 - (phi + std::f64::consts::PI) / (2.0 * std::f64::consts::PI);
        let v = (theta + std::f64::consts::PI / 2.0) / std::f64::consts::PI;
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

pub struct Sphere<T: Material> {
    pub center: Vec3,
    pub radius: f64,
    pub mat: T,
}

impl<T: Material> Sphere<T> {
    pub fn new(v: Vec3, r: f64, m: T) -> Self {
        Self {
            center: v,
            radius: r,
            mat: m,
        }
    }
}

impl<T: Material> Object for Sphere<T> {
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
                    mat: Option::Some(&self.mat),
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
                    mat: Option::Some(&self.mat),
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

    fn pdf_value(&self, o: Vec3, d: Vec3) -> f64 {
        let rec = self.hit(Ray::new(o, d), 0.001, INFINITY);
        match rec {
            None => 0.0,
            Some(_r) => {
                let cotma =
                    (1.0 - self.radius * self.radius / ((self.center - o).length_squared())).sqrt();
                let solid_ang = 2.0 * std::f64::consts::PI * (1.0 - cotma);
                1.0 / solid_ang
            }
        }
    }

    fn random(&self, v: Vec3) -> Vec3 {
        let di = self.center - v;
        let dis_sq = di.length_squared();
        let uvw = Onb::build_from_w(di);
        uvw.local(Vec3::random_to_sphere(self.radius, dis_sq))
    }
}

#[derive(Clone, Debug, Copy)]
pub struct XYRect<T: Material> {
    mp: T,
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
}

impl<T: Material> XYRect<T> {
    pub fn new(a_: f64, b_: f64, c_: f64, d_: f64, f_: f64, e_: T) -> Self {
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

impl<T: Material> Object for XYRect<T> {
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
            mat: Option::Some(&self.mp),
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

    fn pdf_value(&self, o: Vec3, d: Vec3) -> f64 {
        let rec = self.hit(Ray::new(o, d), 0.001, INFINITY);
        match rec {
            None => 0.0,
            Some(rec) => {
                let area = (self.x1 - self.x0) * (self.y1 - self.y0);
                let dis = rec.t * rec.t * d.length_squared();
                let co = (d * rec.normal / d.length()).abs();
                dis / (co * area)
            }
        }
    }

    fn random(&self, v: Vec3) -> Vec3 {
        let mut rng = rand::thread_rng();
        let ran = Vec3::new(
            rng.gen_range(self.x0, self.x1),
            self.k,
            rng.gen_range(self.y0, self.y1),
        );
        ran - v
    }
}

#[derive(Clone, Debug, Copy)]
pub struct XZRect<T: Material> {
    mp: T,
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64,
}

impl<T: Material> XZRect<T> {
    pub fn new(a_: f64, b_: f64, c_: f64, d_: f64, f_: f64, e_: T) -> Self {
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

impl<T: Material> Object for XZRect<T> {
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
            mat: Option::Some(&self.mp),
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

    fn pdf_value(&self, o: Vec3, d: Vec3) -> f64 {
        let rec = self.hit(Ray::new(o, d), 0.001, INFINITY);
        match rec {
            None => 0.0,
            Some(rec) => {
                let area = (self.x1 - self.x0) * (self.z1 - self.z0);
                let dis = rec.t * rec.t * d.length_squared();
                let co = (d * rec.normal / d.length()).abs();
                dis / (co * area)
            }
        }
    }

    fn random(&self, v: Vec3) -> Vec3 {
        let mut rng = rand::thread_rng();
        let ran = Vec3::new(
            rng.gen_range(self.x0, self.x1),
            self.k,
            rng.gen_range(self.z0, self.z1),
        );
        ran - v
    }
}

#[derive(Clone, Debug, Copy)]
pub struct YZRrect<T: Material> {
    mp: T,
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    k: f64,
}

impl<T: Material> YZRrect<T> {
    pub fn new(a_: f64, b_: f64, c_: f64, d_: f64, f_: f64, e_: T) -> Self {
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

impl<T: Material> Object for YZRrect<T> {
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
            mat: Option::Some(&self.mp),
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

    fn pdf_value(&self, o: Vec3, d: Vec3) -> f64 {
        let rec = self.hit(Ray::new(o, d), 0.001, INFINITY);
        match rec {
            None => 0.0,
            Some(rec) => {
                let area = (self.y1 - self.y0) * (self.z1 - self.z0);
                let dis = rec.t * rec.t * d.length_squared();
                let co = (d * rec.normal / d.length()).abs();
                dis / (co * area)
            }
        }
    }

    fn random(&self, v: Vec3) -> Vec3 {
        let mut rng = rand::thread_rng();
        let ran = Vec3::new(
            rng.gen_range(self.y0, self.y1),
            self.k,
            rng.gen_range(self.z0, self.z1),
        );
        ran - v
    }
}

pub struct Box {
    pub box_min: Vec3,
    pub box_max: Vec3,
    pub sides: HittableList,
}

impl Box {
    pub fn new<U: Material + Clone + 'static>(mi: Vec3, ma: Vec3, p: U) -> Self {
        let mut wor = HittableList::new();
        wor.add(Arc::new(XYRect::new(
            mi.x,
            ma.x,
            mi.y,
            ma.y,
            ma.z,
            p.clone(),
        )));
        wor.add(Arc::new(XYRect::new(
            mi.x,
            ma.x,
            mi.y,
            ma.y,
            mi.z,
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
        wor.add(Arc::new(XZRect::new(
            mi.x,
            ma.x,
            mi.z,
            ma.z,
            mi.y,
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
        wor.add(Arc::new(YZRrect::new(
            mi.y,
            ma.y,
            mi.z,
            ma.z,
            mi.x,
            p,
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

pub struct Translate<T: Object> {
    pub ptr: T,
    pub offset: Vec3,
}

impl<T: Object> Translate<T> {
    pub fn new(p: T, v: Vec3) -> Self {
        Self { ptr: p, offset: v }
    }
}

impl<T: Object> Object for Translate<T> {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mor = Ray::new(r.beg - self.offset, r.dir);
        let wmm = self.ptr.hit(mor, t_min, t_max);
        match wmm {
            None => None,
            Some(k) => {
                let mut ret = k;
                ret.p += self.offset;
                ret.normal = {
                    if mor.dir * ret.normal < 0.0 {
                        ret.normal
                    } else {
                        -ret.normal
                    }
                };
                Some(ret)
            }
        }
    }

    fn bounding_box(&self) -> Option<AABB> {
        let tmp = self.ptr.bounding_box();
        match tmp {
            None => None,
            Some(k) => Some(AABB::new(k.min + self.offset, k.max + self.offset)),
        }
    }
}

pub struct RotateY<T: Object> {
    ptr: T,
    sin_theta: f64,
    cos_theta: f64,
    hasbox: bool,
    bbox: AABB,
}

impl<T: Object> RotateY<T> {
    pub fn new(p: T, angle: f64) -> Self {
        let radians = angle * std::f64::consts::PI / 180.0;

        let co = radians.cos();
        let si = radians.sin();

        let get = p.bounding_box();
        let mut tt = get.unwrap();

        let mi = Vec3::new(INFINITY, INFINITY, INFINITY);
        let ma = Vec3::new(-INFINITY, -INFINITY, -INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = tt.max.x * i as f64 + (1.0 - i as f64) * tt.min.x;
                    let y = tt.max.y * j as f64 + (1.0 - j as f64) * tt.min.y;
                    let z = tt.max.z * k as f64 + (1.0 - k as f64) * tt.min.z;

                    let newx = x * co + z * si;
                    let newz = -si * x + co * z;
                    let tes = Vec3::new(newx, y, newz);

                    mi.x.min(tes.x);
                    ma.x.max(tes.x);
                    mi.y.min(tes.y);
                    ma.y.max(tes.y);
                    mi.z.min(tes.z);
                    ma.z.max(tes.z);
                }
            }
        }
        tt = AABB::new(mi, ma);
        Self {
            ptr: p,
            sin_theta: si,
            cos_theta: co,
            hasbox: {
                match get {
                    None => false,
                    Some(_w) => true,
                }
            },
            bbox: tt,
        }
    }
}

impl<T: Object> Object for RotateY<T> {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut ori = r.beg;
        let mut di = r.dir;

        ori.x = self.cos_theta * r.beg.x - self.sin_theta * r.beg.z;
        ori.z = self.sin_theta * r.beg.x + self.cos_theta * r.beg.z;

        di.x = self.cos_theta * r.dir.x - self.sin_theta * r.dir.z;
        di.z = self.sin_theta * r.dir.x + self.cos_theta * r.dir.z;

        let ror = Ray::new(ori, di);

        let ww = self.ptr.hit(ror, t_min, t_max);

        match ww {
            None => None,
            Some(k) => {
                let mut p = k.p;
                let mut nor = k.normal;
                p.x = self.cos_theta * k.p.x + self.sin_theta * k.p.z;
                p.z = -self.sin_theta * k.p.x + self.cos_theta * k.p.z;
                nor.x = self.cos_theta * k.normal.x + self.sin_theta * k.normal.z;
                nor.z = -self.sin_theta * k.normal.x + self.cos_theta * k.normal.z;

                let mut ret = k;
                ret.p = p;
                ret.normal = {
                    if ror.dir * nor < 0.0 {
                        nor
                    } else {
                        -nor
                    }
                };
                Option::Some(ret)
            }
        }
    }

    fn bounding_box(&self) -> Option<AABB> {
        if self.hasbox {
            Some(self.bbox)
        } else {
            None
        }
    }
}

pub struct FlipFace<T: Object> {
    ptr: T,
}

impl<T: Object> FlipFace<T> {
    pub fn new(a: T) -> Self {
        Self { ptr: a }
    }
}

impl<T: Object> Object for FlipFace<T> {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let rec = self.ptr.hit(r, t_min, t_max);
        match rec {
            None => None,
            Some(k) => {
                let mut p = k;
                p.front_face = !p.front_face;
                Some(p)
            }
        }
    }
    fn bounding_box(&self) -> Option<AABB> {
        self.ptr.bounding_box()
    }
}
