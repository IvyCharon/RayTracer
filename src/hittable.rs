use crate::hit_record;
use crate::Object;
use crate::Ray;
use crate::Sphere;
use crate::Vec3;

pub struct Hittable_list {
    objects: Vec<Box<dyn Object>>,
}

impl Hittable_list {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn hit(self, r: Ray, t_min: f64, t_max: f64) -> hit_record {
        let mut temp_rec = hit_record::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for object in self.objects {
            temp_rec = (*object).hit(r, t_min, closest_so_far);
            if temp_rec.ifhit {
                hit_anything = true;
                closest_so_far = temp_rec.t;
            }
        }
        temp_rec.ifhit = hit_anything;
        return temp_rec;
    }
}
