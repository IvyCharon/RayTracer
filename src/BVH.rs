use crate::aabb;
use crate::Hit_record;
use crate::Hittable_list;
use crate::Object;
use crate::Ray;
use crate::Vec3;
use std::sync::Arc;
extern crate rand;
use rand::Rng;

pub struct bvh_node {
    pub box_: aabb,
    pub left: Arc<dyn Object>,
    pub right: Arc<dyn Object>,
}

impl bvh_node {
    pub fn new_(b: aabb, l: Arc<dyn Object>, r: Arc<dyn Object>) -> Self {
        Self {
            box_: b,
            left: l,
            right: r,
        }
    }
    pub fn new(mut world: Hittable_list, t0: f64, t1: f64) -> Self {
        bvh_node::build(world.objects, world.num as u32, t0, t1)
    }

    pub fn random_int(min: u32, max: u32) -> u32 {
        let mut rng = rand::thread_rng();
        return rng.gen_range(min, max + 1) as u32;
    }

    pub fn build(mut objects: Vec<Arc<dyn Object>>, object_span: u32, t0: f64, t1: f64) -> Self {
        let axis = bvh_node::random_int(0, 2);
        let left: Arc<dyn Object>;
        let right: Arc<dyn Object>;
        if object_span == 1 as u32 {
            left = objects.remove(0);
            right = left.clone();
        } else if object_span == 2 as u32 {
            right = objects.remove(1);
            left = objects.remove(0);
        } else {
            objects.sort_by(|a, b| {
                let bo_a = a.bounding_box(t0, t1).unwrap().min.get(axis);
                let bo_b = b.bounding_box(t0, t1).unwrap().min.get(axis);
                bo_a.partial_cmp(&bo_b).unwrap()
            });
            let mid = object_span / 2;
            let (object1, object2) = objects.split_at_mut(mid as usize);

            left = Arc::new(bvh_node::build(object1.to_vec(), mid, t0, t1));
            right = Arc::new(bvh_node::build(object2.to_vec(), object_span - mid, t0, t1));
        }
        let box_left = left.bounding_box(t0, t1);
        let box_right = right.bounding_box(t0, t1);
        bvh_node::new_(
            aabb::surrounding_box(box_left.unwrap(), box_right.unwrap()),
            left,
            right,
        )
    }
}

impl Object for bvh_node {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<Hit_record> {
        let tmp = self.box_.hit(r, t_min, t_max);
        if !tmp {
            return Option::None;
        } else {
            let hit_left = self.left.hit(r, t_min, t_max);
            match hit_left {
                None => {
                    let hit_right = self.right.hit(r, t_min, t_max);
                    if let Some(z) = hit_right {
                        return Option::Some(z);
                    } else {
                        return Option::None;
                    }
                }
                Some(y) => {
                    let hit_right = self.right.hit(r, t_min, y.t);
                    if let Some(z) = hit_right {
                        return Option::Some(z);
                    } else {
                        return Option::Some(y);
                    }
                }
            }
        }
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<aabb> {
        return Option::Some(self.box_);
    }
}
