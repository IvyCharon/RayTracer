use crate::Object;
use crate::Ray;
use crate::Vec3;
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct aabb {
    pub min: Vec3,
    pub max: Vec3,
}

impl aabb {
    pub fn new(a: Vec3, b: Vec3) -> Self {
        Self { min: a, max: b }
    }

    pub fn min(a: f64, b: f64) -> f64 {
        if a < b {
            return a;
        } else {
            return b;
        }
    }

    pub fn max(a: f64, b: f64) -> f64 {
        if a > b {
            return a;
        } else {
            return b;
        }
    }

    pub fn hit(self, r: Ray, tmin: f64, tmax: f64) -> bool {
        let mut tmi = tmin;
        let mut tma = tmax;
        for i in 0..3 {
            let invD = 1.0 / r.dir.get(i);
            let mut t0 = (self.min.get(i) - r.beg.get(i)) * invD;
            let mut t1 = (self.max.get(i) - r.beg.get(i)) * invD;
            if invD < 0.0 {
                let tmp = t0;
                t0 = t1;
                t1 = tmp;
            }
            let tmi = {
                if t0 > tmi {
                    t0
                } else {
                    tmi
                }
            };
            let tma = {
                if t1 < tma {
                    t1
                } else {
                    tma
                }
            };
            if tma <= tmi {
                return false;
            }
        }
        return true;
    }

    pub fn surrounding_box(box1: aabb, box2: aabb) -> aabb {
        let small = Vec3::new(
            aabb::min(box1.min.x, box2.min.x),
            aabb::min(box1.min.y, box2.min.y),
            aabb::min(box1.min.z, box2.min.z),
        );
        let big = Vec3::new(
            aabb::max(box1.max.x, box2.max.x),
            aabb::max(box1.max.y, box2.max.y),
            aabb::max(box1.max.z, box2.max.z),
        );
        return aabb::new(small, big);
    }
}
