use crate::aabb;
use crate::bvh_node;
use crate::checker_texture;
use crate::diffuse_light;
use crate::solid_color;
use crate::xy_rect;
use crate::yz_rect;
use crate::xz_rect;
use crate::Dielectric;
use crate::Hit_record;
use crate::Lambertian;
use crate::Metal;
use crate::Object;
use crate::Ray;
use crate::Sphere;
use crate::Vec3;
use std::sync::Arc;
extern crate rand;
use rand::Rng;

const INFINITY: f64 = 1e15;

pub struct Hittable_list {
    pub objects: Vec<Arc<dyn Object>>,
    pub num: usize,
}

impl Object for Hittable_list{
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<Hit_record> {
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
    
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<aabb> {
        if self.objects.is_empty() {
            return Option::None;
        }
        let mut first_box = true;
        let mut output_box = aabb::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
        for object in self.objects.iter() {
            let tmp = object.bounding_box(t0, t1);
            match tmp {
                None => {
                    return Option::None;
                }
                Some(tmp) => {
                    if first_box {
                        output_box = tmp;
                    } else {
                        output_box = aabb::surrounding_box(output_box, tmp);
                    }
                    first_box = false;
                }
            }
        }
        return Option::Some(output_box);
    }
}

impl Hittable_list {
    pub fn new() -> Self {
        Self {
            objects: vec![Arc::new(Sphere {
                center: Vec3::new(0.0, 0.0, 0.0),
                radius: 0.0,
                mat: Arc::new(Lambertian::new(Vec3::new(0.0, 0.0, 0.0))),
            })],
            num: 0,
        }
    }

    pub fn add(&mut self, t: Arc<dyn Object>) {
        self.objects.push(t);
        self.num += 1;
    }

    pub fn random_scene() -> Arc<bvh_node> {
        let mut world = Hittable_list::new();

        let checker = Arc::new(checker_texture::new(
            Vec3::new(0.2, 0.3, 0.1),
            Vec3::new(0.9, 0.9, 0.9),
        ));
        world.add(Arc::new(Sphere::new(
            Vec3::new(0.0, -1000.0, 0.0),
            1000.0,
            Arc::new(Lambertian::new_(checker)),
        )));

        for a in -6..6 {
            for b in -6..6 {
                let choose_mat = rand::random::<f64>();
                let center = Vec3::new(
                    a as f64 + 0.9 * rand::random::<f64>(),
                    0.2,
                    b as f64 + 0.9 * rand::random::<f64>(),
                );

                if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                    if choose_mat < 0.6 {
                        let dl = Arc::new(diffuse_light::new(Arc::new(solid_color::new(
                            Vec3::elemul(Vec3::Random(), Vec3::Random()),
                        ))));
                        world.add(Arc::new(Sphere::new(center, 0.2, dl)));
                    } else if choose_mat < 0.7 {
                        //difuse
                        let albedo = Vec3::elemul(Vec3::Random(), Vec3::Random());
                        let sphere_mat = Lambertian::new(albedo);
                        world.add(Arc::new(Sphere::new(center, 0.2, Arc::new(sphere_mat))));
                    } else if choose_mat < 0.85 {
                        //metal
                        let albedo = Vec3::Random_(0.5, 1.0);
                        let mut rng = rand::thread_rng();
                        let fuzz = rng.gen_range(0.0, 0.5);
                        let sphere_mat = Metal::new(albedo, fuzz);
                        world.add(Arc::new(Sphere::new(center, 0.2, Arc::new(sphere_mat))));
                    } else {
                        //glass
                        let sphere_mat = Dielectric::new(1.5);
                        world.add(Arc::new(Sphere::new(center, 0.2, Arc::new(sphere_mat))));
                    }
                }
            }
        }
        let mat1 = Dielectric::new(1.5);
        world.add(Arc::new(Sphere::new(
            Vec3::new(0.0, 1.0, 0.0),
            1.0,
            Arc::new(mat1),
        )));

        let mat2 = Lambertian::new(Vec3::new(0.4, 0.2, 0.1));
        world.add(Arc::new(Sphere::new(
            Vec3::new(-4.0, 1.0, 0.0),
            1.0,
            Arc::new(mat2),
        )));

        let mat3 = Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0);
        world.add(Arc::new(Sphere::new(
            Vec3::new(4.0, 1.0, 0.0),
            1.0,
            Arc::new(mat3),
        )));
        Arc::new(bvh_node::new(world, 0.001, INFINITY))
    }

    pub fn test_xy() -> Arc<dyn Object> {
        let mut world = Hittable_list::new();
        /*let checker = Arc::new(checker_texture::new(
            Vec3::new(0.2, 0.3, 0.1),
            Vec3::new(0.9, 0.9, 0.9),
        ));
        world.add(Arc::new(Sphere::new(
            Vec3::new(0.0, -1000.0, 0.0),
            1000.0,
            Arc::new(Lambertian::new_(checker)),
        )));*/

        //let tmp = Arc::new(Lambertian::new(Vec3::new(0.5,0.5,0.5)));
        //world.add(Arc::new(Sphere::new(Vec3::new(0.0,2.0,0.0),2.0,tmp)));

        let ttmp = Arc::new(Lambertian::new(Vec3::new(0.2, 0.4, 0.3)));
        world.add(Arc::new(xy_rect::new(3.0, 5.0, 1.0, 3.0, -2.0, ttmp)));

        Arc::new(bvh_node::new(world, 0.001, INFINITY))
        //Arc::new(world)
    }

    pub fn cornell_box() -> Arc<dyn Object> {
        let red = Arc::new(Lambertian::new(Vec3::new(0.65,0.05,0.05)));
        let white = Arc::new(Lambertian::new(Vec3::new(0.73,0.73,0.73)));
        let green = Arc::new(Lambertian::new(Vec3::new(0.12,0.45,0.15)));
        let light = Arc::new(diffuse_light::new_(Vec3::new(15.0,15.0,15.0)));

        let mut world = Hittable_list::new();
        world.add(Arc::new(yz_rect::new(0.0,555.0,0.0,555.0,555.0,green)));
        world.add(Arc::new(yz_rect::new(0.0,555.0,0.0,555.0,0.0,red)));
        world.add(Arc::new(xz_rect::new(213.0,343.0,227.0,332.0,554.0,light)));
        world.add(Arc::new(xz_rect::new(0.0,555.0,0.0,555.0,0.0,white.clone())));
        world.add(Arc::new(xz_rect::new(0.0,555.0,0.0,555.0,555.0,white.clone())));
        world.add(Arc::new(xy_rect::new(0.0,555.0,0.0,555.0,555.0,white.clone())));
        
        Arc::new(world)
    }
}
