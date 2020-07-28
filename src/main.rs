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
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use material::Dielectric;
use material::Lambertian;
use material::Material;
use material::Metal;
use ray::Ray;
use std::sync::Arc;
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
            let s = rec.mat.as_ref().unwrap().scatter(r, &rec);
            if s.jud {
                return Vec3::elemul(ray_color(s.scattered, world, depth - 1), s.attenustion);
            }
            return Vec3::new(0.0, 0.0, 0.0);
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
    const aspect_ratio: f64 = 16.0 / 9.0;
    const image_width: u32 = 400;
    const image_height: u32 = ((image_width as f64) / aspect_ratio) as u32;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let mut img: RgbImage = ImageBuffer::new(image_width.clone(), image_height.clone());
    let bar = ProgressBar::new(image_height as u64);

    let world = Hittable_list::random_scene();

    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    for j in 0..image_height {
        for i in 0..image_width {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for s in 0..samples_per_pixel.clone() {
                let u = (i as f64 + rand::random::<f64>()) / (image_width as f64 - 1.0);
                let v = (image_height as f64 - j as f64 + rand::random::<f64>())
                    / (image_height as f64 - 1.0);
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
