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
            objects: vec![Box::new(Sphere {
                center: Vec3::new(0.0,0.0,0.0),
                radius:0.0,
            })],
        }
    }

    pub fn add(&mut self, t: Box<dyn Object>){
        self.objects.push(t);
    }

    pub fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<hit_record> {
        let mut temp_rec = Option::None;
        let mut closest_so_far = t_max;
        for object in self.objects.iter() {
            if let Option::Some(rec) = object.hit(r, t_min, closest_so_far){
                closest_so_far = rec.t;
                temp_rec = Option::Some(rec);
            }
        }
        return temp_rec;
    }
}
