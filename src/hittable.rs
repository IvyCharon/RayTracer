use crate::Hit_record;
use crate::Lambertian;
use crate::Metal;
use crate::Dielectric;
use crate::Object;
use crate::Ray;
use crate::Sphere;
use crate::Vec3;
use std::sync::Arc;
extern crate rand;
use rand::Rng;

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

    pub fn random_scene() -> Self{
        let mut world = Hittable_list::new();

        let ground_mat = Lambertian::new(Vec3::new(0.5,0.5,0.5));
        world.add(Arc::new(Sphere::new(Vec3::new(0.0,-1000.0,0.0), 1000.0, Arc::new(ground_mat))));

        for a in -11 .. 11{
            for b in -11..11{
                let choose_mat = rand::random::<f64>();
                let center = Vec3::new(a as f64 + 0.9 * rand::random::<f64>(), 0.2, b as f64 + 0.9 * rand::random::<f64>());
                
                if (center - Vec3::new(4.0,0.2,0.0)).length() > 0.9{
                    if choose_mat < 0.8{
                        //difuse
                        let albedo = Vec3::elemul(Vec3::Random(),Vec3::Random());
                        let sphere_mat = Lambertian::new(albedo);
                        world.add(Arc::new(Sphere::new(center, 0.2, Arc::new(sphere_mat))));
                    } else if choose_mat < 0.95 {
                        //metal
                        let albedo = Vec3::Random_(0.5,1.0);
                        let mut rng = rand::thread_rng();
                        let fuzz = rng.gen_range(0.0,0.5);
                        let sphere_mat = Metal::new(albedo, fuzz);
                        world.add(Arc::new(Sphere::new(center, 0.2, Arc::new(sphere_mat))));
                    }else {
                        //glass
                        let sphere_mat = Dielectric::new(1.5);
                        world.add(Arc::new(Sphere::new(center, 0.2, Arc::new(sphere_mat))));
                    }
                }
            }
        }
        let mat1 = Dielectric::new(1.5);
        world.add(Arc::new(Sphere::new(Vec3::new(0.0,1.0,0.0), 1.0, Arc::new(mat1))));

        let mat2 = Lambertian::new(Vec3::new(0.4,0.2,0.1));
        world.add(Arc::new(Sphere::new(Vec3::new(-4.0,1.0,0.0), 1.0, Arc::new(mat2))));

        let mat3 = Metal::new(Vec3::new(0.7,0.6,0.5),0.0);
        world.add(Arc::new(Sphere::new(Vec3::new(4.0,1.0,0.0), 1.0, Arc::new(mat3))));

        return world;
    }
}
