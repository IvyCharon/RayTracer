use crate::Hit_record;
use crate::Lambertian;
use crate::Object;
use crate::Ray;
use crate::Sphere;
use crate::Vec3;
use std::sync::Arc;

pub struct Hittable_list {
    objects: Vec<Arc<dyn Object>>,
}

impl Hittable_list {
    pub fn new() -> Self {
        Self {
            objects: vec![Arc::new(Sphere {
                center: Vec3::new(0.0, 0.0, 0.0),
                radius: 0.0,
                mat: Arc::new(Lambertian::new(Vec3::new(0.0, 0.0, 0.0))),
            })],
        }
    }

    pub fn add(&mut self, t: Arc<dyn Object>) {
        self.objects.push(t);
    }

    pub fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<Hit_record> {
        let mut temp_rec = Option::None;
        let mut closest_so_far = t_max;
        for object in self.objects.iter() {
            if let Option::Some(rec) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = rec.t;
                temp_rec = Option::Some(rec);
            }
        }
        return temp_rec;
    }
}
