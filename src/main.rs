mod camera;
mod ray;
mod sphere;
#[allow(clippy::float_cmp)]
mod vec3;
use camera::Camera;
use sphere::Hit_record;
use sphere::Object;
use sphere::Sphere;
mod hittable;
use hittable::Hittable_list;
mod material;
use material::Material;
use material::Lambertian;
use material::Metal;
use material::Sca_ret;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use ray::Ray;
extern crate rand;

const INFINITY: f64 = 1e15;

pub use vec3::Vec3;

fn ray_color(r: Ray, world: &Hittable_list, depth: i32) -> Vec3 {
    let rec = world.hit(r, 0.001, INFINITY);
    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }
    match rec {
        Option::Some(rec) => {
            let target: Vec3 = rec.p + Vec3::random_in_hemisphere(&rec.normal);
            return ray_color(Ray::new(rec.p, target - rec.p), world, depth - 1) * 0.5;
            let s = (rec.mat).unwrap().scatter(r, rec);
            if s.jud {
                return Vec3::elemul(ray_color(r, world, depth - 1), s.attenustion);
            }
            return Vec3::new(0.0,0.0,0.0);
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

fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    return x;
}

fn main() {
    let mut img: RgbImage = ImageBuffer::new(400, 225);
    let bar = ProgressBar::new(225);

    let mut world = Hittable_list::new();

    let mat_ground = Lambertian::new(Vec3::new(0.8,0.8,0.8));
    let mat_center = Lambertian::new(Vec3::new(0.7,0.3,0.3));
    let mat_left = Metal::new(Vec3::new(0.8,0.8,0.8));
    let mat_right = Metal::new(Vec3::new(0.8,0.6,0.2));

    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, Box::new(mat_ground))));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, Box::new(mat_center))));
    world.add(Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, Box::new(mat_left))));
    world.add(Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, Box::new(mat_right))));
    

    let cam = Camera::new();
    let samples_per_pixel = 100;
    let max_depth = 50;

    for j in 0..225 {
        for i in 0..400 {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for s in 0..samples_per_pixel.clone() {
                let u = (i as f64 + rand::random::<f64>()) / (400.0 - 1.0);
                let v = (225.0 - j as f64 + rand::random::<f64>()) / (225.0 - 1.0);
                let r = cam.get_ray(u, v);
                col += ray_color(r, &world, max_depth.clone());
            }
            let pixel = img.get_pixel_mut(i, j);
            *pixel = image::Rgb([
                (clamp(
                    (col.x / (samples_per_pixel.clone() as f64)).sqrt(),
                    0.0,
                    0.999,
                ) * 255.999) as u8,
                (clamp(
                    (col.y / (samples_per_pixel.clone() as f64)).sqrt(),
                    0.0,
                    255.999,
                ) * 255.999) as u8,
                (clamp(
                    (col.z / (samples_per_pixel.clone() as f64)).sqrt(),
                    0.0,
                    255.999,
                ) * 255.999) as u8,
            ]);
        }
        bar.inc(1);
    }

    img.save("output/test.png").unwrap();
    bar.finish();
}
