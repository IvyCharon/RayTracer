use crate::aabb;
use std::sync::Arc;
use crate::Object;
use crate::Hit_record;
use crate::Hittable_list;
use crate::Ray;
use crate::Vec3;
extern crate rand;
use rand::Rng;

pub struct bvh_node{
    pub box_: aabb,
    pub left_leaf: Option<Arc<dyn Object>>,
    pub right_leaf: Option<Arc<dyn Object>>,
    pub left: Option<Arc<bvh_node>>,
    pub right: Option<Arc<bvh_node>>,
}

impl bvh_node{
    pub fn new_() -> Self{
        Self{
            box_: aabb::new(Vec3::zero(), Vec3::zero()),
            left_leaf: Option::None,
            right_leaf: Option::None,
            left: Option::None,
            right: Option::None,
        }
    }
    pub fn new(world: Hittable_list, t0: f64, t1: f64) -> Self{
        bvh_node::build(&world.objects, 0, world.objects.len() as u32, t0, t1)
    }

    pub fn random_int(min: u32, max: u32) -> u32{
        let mut rng = rand::thread_rng();
        return rng.gen_range(min, max + 1) as u32;
    }

    pub fn build(mut objects: &Vec<Arc<dyn Object>>, start: u32, end: u32, t0: f64, t1: f64) -> Self{
        let axis = bvh_node::random_int(0, 2);
        let object_span = end - start;
        let mut ret = bvh_node::new_();
        if object_span == 1 as u32 {
            ret.left_leaf = Option::Some(objects[start as usize].clone());
            ret.right_leaf = Option::Some(objects[start as usize].clone());
        }else if object_span == 2 as u32{
            if aabb::compare(objects[start as usize].clone(), objects[start as usize + 1].clone(), axis){
                ret.left_leaf = Option::Some(objects[start as usize].clone());
                ret.right_leaf = Option::Some(objects[start as usize + 1].clone());
            }else{
                ret.right_leaf = Option::Some(objects[start as usize].clone());
                ret.left_leaf = Option::Some(objects[start as usize + 1].clone());
            }
        }else{
            //sort(objects.begin() + start, objects.begin() + end, comparator);
            let compare = {
                if axis == 0 {
                    aabb::compare_x
                }else if axis == 1{
                    aabb::compare_y
                }else{
                    aabb::compare_z
                }
            };
            
            let mid = start + object_span / 2;
            ret.left = Option::Some(Arc::new(bvh_node::build(objects, start, mid, t0, t1)));
            ret.right = Option::Some(Arc::new(bvh_node::build(objects, mid, end, t0, t1)));
        }
        if let Some(x) = ret.left_leaf{
            if let Some(y) = ret.right_leaf{
                let box_left = x.bounding_box(t0, t1);
                let box_right = y.bounding_box(t0, t1);
                ret.box_ = aabb::surrounding_box(box_left.unwrap(), box_right.unwrap());
            }
        }
        if let Some(x) = ret.left{
            if let Some(y) = ret.right{
                 let box_left = x.bounding_box(t0, t1);
                let box_right = y.bounding_box(t0, t1);
                ret.box_ = aabb::surrounding_box(box_left.unwrap(), box_right.unwrap());
            }
        } 
        return ret;
    }

    pub fn bounding_box(&self, t0: f64, t1: f64) -> Option<aabb>{
        return Option::Some(self.box_);
    }

    pub fn hit(self, r: Ray, tmin: f64, tmax: f64) -> Option<Hit_record>{
        let tmp = self.box_.hit(r, tmin, tmax);
        if !tmp{
            return Option::None;
        }
        if let Some(x) = self.left_leaf {
            let hit_left = x.hit(r, tmin, tmax);
            let mut hit_right = Option::None;
            match hit_left {
                None => {
                    hit_right = self.right_leaf.unwrap().hit(r, tmin, tmax);
                    return hit_right;
                }
                Some(hit_left) => {
                    hit_right = self.right_leaf.unwrap().hit(r, tmin, hit_left.t);
                    match hit_right {
                        None => {
                            return Option::Some(hit_left);
                        }
                        Some(hit_right) => {
                            return Option::Some(hit_right);
                        }
                    }
                }
            }
        }
        if let Some(x) = self.left{
            let hit_left = x.hit(r, tmin, tmax);
            let mut hit_right = Option::None;
            match hit_left {
                None => {
                    if let Some(y) = self.right{
                        hit_right = y.hit(r, tmin, tmax);
                    return hit_right;
                    }
                }
                Some(hit_left) => {
                    if let Some(y) = self.right{
                        hit_right = y.hit(r, tmin, hit_left.t);
                        match hit_right {
                            None => {
                                return Option::Some(hit_left);
                            }
                            Some(hit_right) => {
                                return Option::Some(hit_right);
                            }
                        }
                    }
                    
                }
            }
        }
        return Option::None;
    }
}