use crate::Vec3;
use crate::Ray;
use std::sync::Arc;
use crate::Object;

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct aabb {
    pub min: Vec3,
    pub max: Vec3,
}

impl aabb {
    pub fn new(a: Vec3, b: Vec3) -> Self{
        Self{
            min: a,
            max: b,
        }
    }

    pub fn min(a: f64, b: f64) -> f64{
        if a < b{
            return a;
        }else{
            return b;
        }
    }

    pub fn max(a: f64, b: f64) -> f64{
        if a > b{
            return a;
        }else{
            return b;
        }
    }

    pub fn hit(self,r: Ray, tmin: f64, tmax: f64) -> bool{
        let mut tmi = tmin;
        let mut tma = tmax;
        for i in 0..3{
            let invD = 1.0 / r.dir.get(i);
            let mut t0 = (self.min.get(i) - r.beg.get(i)) * invD;
            let mut t1 = (self.max.get(i) - r.beg.get(i)) * invD;
            if invD < 0.0{
                let tmp = t0;
                t0 = t1;
                t1 = tmp;
            }
            let tmi = {
                if t0 > tmi{
                    t0
                }else{
                    tmi
                }
            };
            let tma = {
                if t1 < tma{
                    t1
                }else{
                    tma
                }
            };
            if tma <= tmi {
                return false;
            }
        }
        return true;
    }

    pub fn surrounding_box(box1: aabb, box2: aabb) -> aabb{
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

    pub fn compare(a: Arc<dyn Object>, b: Arc<dyn Object>, ax: u32) -> bool{
        let bo_a = a.bounding_box(0.0, 0.0);
        let bo_b = b.bounding_box(0.0, 0.0);
        match bo_a {
            None => {
                println!("wrong!");
                return  false;
            }
            Some(bo_a) => {
                match bo_b {
                    None => {
                        println!("wrong!");
                        return false;
                    }
                    Some(bo_b) => {
                        return bo_a.min.get(ax) < bo_b.min.get(ax);
                    }
                }
            }
        }
    }

    pub fn compare_x(a: Arc<dyn Object>, b: Arc<dyn Object>) -> bool{
        return aabb::compare(a, b, 0);
    }

    pub fn compare_y(a: Arc<dyn Object>, b: Arc<dyn Object>) -> bool{
        return aabb::compare(a, b, 1);
    }

    pub fn compare_z(a: Arc<dyn Object>, b: Arc<dyn Object>) -> bool{
        return aabb::compare(a, b, 2);
    }
}