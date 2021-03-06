use crate::HitRecord;
use crate::HittableList;
use crate::Object;
use crate::Ray;
use crate::AABB;
use std::sync::Arc;
extern crate rand;
use rand::Rng;

pub struct BvhNode {
    pub box_: AABB,
    pub left: Arc<dyn Object>,
    pub right: Arc<dyn Object>,
}

impl BvhNode {
    pub fn new_(b: AABB, l: Arc<dyn Object>, r: Arc<dyn Object>) -> Self {
        Self {
            box_: b,
            left: l,
            right: r,
        }
    }
    pub fn new(world: HittableList, t0: f64, t1: f64) -> Self {
        BvhNode::build(world.objects, world.num as u32, t0, t1)
    }

    pub fn random_int(min: u32, max: u32) -> u32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(min, max + 1) as u32
    }

    pub fn build(mut objects: Vec<Arc<dyn Object>>, object_span: u32, t0: f64, t1: f64) -> Self {
        let axis = BvhNode::random_int(0, 2);
        let left: Arc<dyn Object>;
        let right: Arc<dyn Object>;
        if object_span == 1 as u32 {
            left = objects.remove(0);
            right = left.clone();
        } else if object_span == 2 as u32 {
            objects.sort_by(|a, b| {
                let bo_a = a.bounding_box().unwrap().min.get(axis);
                let bo_b = b.bounding_box().unwrap().min.get(axis);
                bo_a.partial_cmp(&bo_b).unwrap()
            });
            right = objects.remove(1);
            left = objects.remove(0);
        } else {
            objects.sort_by(|a, b| {
                let bo_a = a.bounding_box().unwrap().min.get(axis);
                let bo_b = b.bounding_box().unwrap().min.get(axis);
                bo_a.partial_cmp(&bo_b).unwrap()
            });
            let mid = object_span / 2;
            let (object1, object2) = objects.split_at_mut(mid as usize);

            left = Arc::new(BvhNode::build(object1.to_vec(), mid, t0, t1));
            right = Arc::new(BvhNode::build(object2.to_vec(), object_span - mid, t0, t1));
        }
        let box_left = left.bounding_box();
        let box_right = right.bounding_box();
        BvhNode::new_(
            AABB::surrounding_box(box_left.unwrap(), box_right.unwrap()),
            left,
            right,
        )
    }
}

impl Object for BvhNode {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let tmp = self.box_.hit(r, t_min, t_max);
        if !tmp {
            Option::None
        } else {
            let hit_left = self.left.hit(r, t_min, t_max);
            match hit_left {
                None => {
                    let hit_right = self.right.hit(r, t_min, t_max);
                    if let Some(z) = hit_right {
                        Option::Some(z)
                    } else {
                        Option::None
                    }
                }
                Some(y) => {
                    let hit_right = self.right.hit(r, t_min, y.t);
                    if let Some(z) = hit_right {
                        if z.t < y.t {
                            Option::Some(z)
                        } else {
                            Option::Some(y)
                        }
                    } else {
                        Option::Some(y)
                    }
                }
            }
        }
    }

    fn bounding_box(&self) -> Option<AABB> {
        Option::Some(self.box_)
    }
}
