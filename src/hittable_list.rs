use crate::Box;
use crate::BvhNode;
use crate::CheckerTexture;
use crate::Dielectric;
use crate::DiffuseLight;
use crate::FlipFace;
use crate::HitRecord;
use crate::Lambertian;
use crate::Metal;
use crate::Object;
use crate::Ray;
use crate::RotateY;
use crate::SolidColor;
use crate::Sphere;
use crate::Translate;
use crate::Vec3;
use crate::XYRect;
use crate::XZRect;
use crate::YZRrect;
use crate::AABB;
use std::sync::Arc;
extern crate rand;
use rand::Rng;

const INFINITY: f64 = 1e15;

#[derive(Clone)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Object>>,
    pub num: usize,
}

impl Object for HittableList {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut temp_rec = Option::None;
        let mut closest_so_far = t_max;
        for object in self.objects.iter() {
            if let Option::Some(rec) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = rec.t;
                temp_rec = Option::Some(rec);
            }
        }
        temp_rec
    }

    fn bounding_box(&self) -> Option<AABB> {
        if self.objects.is_empty() {
            return Option::None;
        }
        let mut first_box = true;
        let mut output_box = AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
        for object in self.objects.iter() {
            let tmp = object.bounding_box();
            match tmp {
                None => {
                    return Option::None;
                }
                Some(tmp) => {
                    if first_box {
                        output_box = tmp;
                    } else {
                        output_box = AABB::surrounding_box(output_box, tmp);
                    }
                    first_box = false;
                }
            }
        }
        Option::Some(output_box)
    }

    fn pdf_value(&self, o: Vec3, d: Vec3) -> f64 {
        let weight = 1.0 / self.objects.len() as f64;
        let mut sum = 0.0;
        for object in self.objects.iter() {
            sum += weight * object.pdf_value(o, d);
        }
        sum
    }

    fn random(&self, o: Vec3) -> Vec3 {
        let mut rng = rand::thread_rng();
        self.objects[rng.gen_range(0, self.objects.len())].random(o)
    }
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: vec![],
            num: 0,
        }
    }

    pub fn add(&mut self, t: Arc<dyn Object>) {
        self.objects.push(t);
        self.num += 1;
    }

    pub fn random_scene() -> BvhNode {
        let mut world = HittableList::new();

        let checker = CheckerTexture::new(
            SolidColor::new(Vec3::new(0.2, 0.3, 0.1)),
            SolidColor::new(Vec3::new(0.9, 0.9, 0.9)),
        );
        world.add(Arc::new(Sphere::new(
            Vec3::new(0.0, -1000.0, 0.0),
            1000.0,
            Lambertian::new(checker),
        )));

        for a in -11..11 {
            for b in -11..11 {
                let choose_mat = rand::random::<f64>();
                let center = Vec3::new(
                    a as f64 + 0.9 * rand::random::<f64>(),
                    0.2,
                    b as f64 + 0.9 * rand::random::<f64>(),
                );
                if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                    if choose_mat < 0.8 {
                        //difuse
                        let albedo = Vec3::elemul(Vec3::random1(), Vec3::random1());
                        let sphere_mat = Lambertian::new(SolidColor::new(albedo));
                        world.add(Arc::new(Sphere::new(center, 0.2, sphere_mat)));
                    } else if choose_mat < 0.95 {
                        //metal
                        let albedo = Vec3::random2(0.5, 1.0);
                        let mut rng = rand::thread_rng();
                        let fuzz = rng.gen_range(0.0, 0.5);
                        let sphere_mat = Metal::new(albedo, fuzz);
                        world.add(Arc::new(Sphere::new(center, 0.2, sphere_mat)));
                    } else {
                        //glass
                        let sphere_mat = Dielectric::new(1.5);
                        world.add(Arc::new(Sphere::new(center, 0.2, sphere_mat)));
                    }
                }
            }
        }

        let mat1 = Dielectric::new(1.5);
        world.add(Arc::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, mat1)));

        let mat2 = Lambertian::new(SolidColor::new(Vec3::new(0.4, 0.2, 0.1)));
        world.add(Arc::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, mat2)));

        let mat3 = Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0);
        world.add(Arc::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, mat3)));

        //world
        BvhNode::new(world, 0.001, INFINITY)
    }

    pub fn night() -> BvhNode {
        let mut world = HittableList::new();

        let checker = CheckerTexture::new(
            SolidColor::new(Vec3::new(0.2, 0.3, 0.1)),
            SolidColor::new(Vec3::new(0.9, 0.9, 0.9)),
        );
        world.add(Arc::new(Sphere::new(
            Vec3::new(0.0, -1000.0, 0.0),
            1000.0,
            Lambertian::new(checker),
        )));
        for a in -13..13 {
            for b in -13..13 {
                let choose_mat = rand::random::<f64>();
                let mut rng = rand::thread_rng();
                let r = rng.gen_range(0.09, 0.29);
                let center = Vec3::new(
                    a as f64 + 0.9 * rand::random::<f64>(),
                    r,
                    b as f64 + 0.9 * rand::random::<f64>(),
                );

                if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                    if choose_mat < 0.6 {
                        let dl = DiffuseLight::new(SolidColor::new(Vec3::elemul(
                            Vec3::random1(),
                            Vec3::random1(),
                        )));
                        world.add(Arc::new(Sphere::new(center, r * 0.99999, dl)));
                        let ke = Dielectric::new(4.0);
                        world.add(Arc::new(Sphere::new(center, r, ke)));
                    } else if choose_mat < 0.65 {
                        //difuse
                        let albedo = Vec3::elemul(Vec3::random1(), Vec3::random1());
                        let sphere_mat = Lambertian::new(SolidColor::new(albedo));
                        world.add(Arc::new(Sphere::new(center, r, sphere_mat)));
                    } else if choose_mat < 0.8 {
                        //metal
                        let albedo = Vec3::random2(0.5, 1.0);
                        let mut rng = rand::thread_rng();
                        let fuzz = rng.gen_range(0.0, 0.5);
                        let sphere_mat = Metal::new(albedo, fuzz);
                        world.add(Arc::new(Sphere::new(center, r, sphere_mat)));
                    } else {
                        //glass
                        let sphere_mat = Dielectric::new(1.5);
                        world.add(Arc::new(Sphere::new(center, r, sphere_mat)));
                    }
                }
            }
        }

        let mat = DiffuseLight::new(CheckerTexture::new(
            SolidColor::new(Vec3::new(1.0, 1.0, 1.0)),
            SolidColor::new(Vec3::new(
                (12.0 * 16.0 + 7.8) / 255.0,
                (9.0 * 16.0 + 4.0) / 255.0,
                (160.0 + 4.0) / 255.0,
            )),
        ));
        world.add(Arc::new(Sphere::new(
            Vec3::new(3.0, 0.45, 0.0),
            0.45 * 0.99999,
            mat,
        )));
        let ke = Dielectric::new(4.0);
        world.add(Arc::new(Sphere::new(Vec3::new(3.0, 0.45, 0.0), 0.45, ke)));

        //world
        BvhNode::new(world, 0.001, INFINITY)
    }

    pub fn cornell_box() -> HittableList {
        let red = Lambertian::new(SolidColor::new(Vec3::new(0.65, 0.05, 0.05)));
        let white = Lambertian::new(SolidColor::new(Vec3::new(0.73, 0.73, 0.73)));
        let green = Lambertian::new(SolidColor::new(Vec3::new(0.12, 0.45, 0.15)));
        let light = DiffuseLight::new(SolidColor::new(Vec3::new(15.0, 15.0, 15.0)));

        let mut world = HittableList::new();
        world.add(Arc::new(YZRrect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
        world.add(Arc::new(YZRrect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
        world.add(Arc::new(FlipFace::new(XZRect::new(
            213.0, 343.0, 227.0, 332.0, 554.0, light,
        ))));
        world.add(Arc::new(XZRect::new(
            0.0,
            555.0,
            0.0,
            555.0,
            555.0,
            white.clone(),
        )));
        world.add(Arc::new(XZRect::new(
            0.0,
            555.0,
            0.0,
            555.0,
            0.0,
            white.clone(),
        )));
        world.add(Arc::new(XYRect::new(
            0.0,
            555.0,
            0.0,
            555.0,
            555.0,
            white.clone(),
        )));

        let aluminum = Metal::new(Vec3::new(0.8, 0.85, 0.88), 0.0);
        let box1 = Box::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(165.0, 330.0, 165.0),
            aluminum,
            //white,
        );
        let rot1 = RotateY::new(box1, 15.0);
        let ww1 = Arc::new(Translate::new(rot1, Vec3::new(265.0, 0.0, 295.0)));
        world.add(ww1);

        /*let glass_sphere = Arc::new(Sphere::new(
            Vec3::new(190.0, 90.0, 190.0),
            90.0,
            Dielectric::new(1.5),
        ));
        world.add(glass_sphere);*/

        let box2 = Box::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(165.0, 165.0, 165.0),
            white,
        );
        let rot2 = RotateY::new(box2, -18.0);
        let ww2 = Arc::new(Translate::new(rot2, Vec3::new(130.0, 0.0, 65.0)));
        world.add(ww2);

        world
        //BvhNode::new(world, 0.001, INFINITY)
    }
}
