mod ray;
mod sphere;
#[allow(clippy::float_cmp)]
mod vec3;
use sphere::hit_record;
use sphere::Object;
use sphere::Sphere;
mod hittable;
use hittable::Hittable_list;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use ray::Ray;
extern crate rand;

const PI:f64 = 3.1415926535897932385;
const INFINITY: f64 = 1e15;

pub use vec3::Vec3;

fn ray_color(r: Ray, world: &Hittable_list, depth: i32) -> Vec3 {
    let rec = world.hit(r, 0.0, INFINITY);
    if depth <= 0{
        return Vec3::new(0.0,0.0,0.0);
    }
    match rec {
        Option::Some(rec) =>{
            //(rec.normal + Vec3::ones()) * 0.5
            let target:Vec3 = rec.p + rec.normal + Vec3::random_in_unit_sphere();
            return ray_color(Ray::new(rec.p, target - rec.p), world, depth - 1) * 0.5;
            }
        Option::None => {
            let t = 0.5 * (r.dir.unit().y + 1.0);
            Vec3 {
            x: 1.0 - t + t * 0.5,
            y: 1.0 - t + t * 0.7,
            z: 1.0,
            }
        }
    }
}

fn main() {
    let mut img: RgbImage = ImageBuffer::new(400, 225);
    let bar = ProgressBar::new(225);

    let mut world = Hittable_list::new();
    
    world.add(Box::new(Sphere::new(Vec3::new(0.0,0.0,-1.0),0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0,-100.5,-1.0),100.0)));
    
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(16.0 / 9.0 * 2.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, 1.0);
    let samples_per_pixel = 100;
    let max_depth = 50;

    for j in 0..225 {
        for i in 0..400 {
            let mut col = Vec3::new(0.0,0.0,0.0);
            for s in 0..samples_per_pixel.clone(){
                let u = (i as f64 + rand::random::<f64>()) / (400.0 - 1.0);
                let v = (225.0 - j as f64 + rand::random::<f64>()) / (225.0 - 1.0);
                let r = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);
                col += ray_color(r, &world, max_depth.clone());
            }
            let pixel = img.get_pixel_mut(i, j);
            
            *pixel = image::Rgb([
                (col.x * 255.999/(samples_per_pixel.clone()as f64)) as u8,
                (col.y * 255.999/(samples_per_pixel.clone()as f64)) as u8,
                (col.z * 255.999/(samples_per_pixel.clone()as f64)) as u8,
            ]);
        }
        bar.inc(1);
    }

    img.save("output/test.png").unwrap();
    bar.finish();
}
