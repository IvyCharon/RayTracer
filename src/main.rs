#[allow(clippy::float_cmp)]
mod aabb;
use aabb::AABB;
mod camera;
use camera::Camera;
mod object;
use object::Box;
use object::HitRecord;
use object::Object;
use object::RotateY;
use object::Sphere;
use object::Translate;
use object::XYRect;
use object::XZRect;
use object::YZRrect;
mod ray;
use ray::Ray;
mod bvh;
use bvh::BvhNode;
mod texture;
use texture::CheckerTexture;
use texture::SolidColor;
use texture::Texture;
mod vec3;
use vec3::Vec3;
mod hittable_list;
use hittable_list::HittableList;
mod material;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use material::Dielectric;
use material::DiffuseLight;
use material::Lambertian;
use material::Material;
use material::Metal;
use std::sync::Arc;
extern crate rand;

const INFINITY: f64 = 1e15;

fn ray_color(r: Ray, back_ground: Vec3, world: Arc<dyn Object>, depth: i32) -> Vec3 {
    let rec = world.hit(r, 0.001, INFINITY);
    if depth <= 0 {
        return Vec3::zero();
    }
    match rec {
        Option::Some(rec) => {
            let s = rec.mat.as_ref().unwrap().scatter(r, &rec);
            let emitted = rec.mat.as_ref().unwrap().emitted(rec.u, rec.v, rec.p);
            if s.jud {
                let pdf = s.pdf;
                let albedo = s.attenustion;
                return emitted + Vec3::elemul(
                    albedo * rec.mat.as_ref().unwrap().scattering_pdf(r, &rec, s.scattered),
                    ray_color(s.scattered, back_ground, world, depth - 1) / s.pdf);
            }
            emitted
        }
        Option::None => back_ground,
    }
}

fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}

fn main() {
    let samples_per_pixel = 100;
    let max_depth = 50;

    let choose = 2;
    let world: Arc<dyn Object>;
    let aspect_ratio: f64;
    let image_width: u32;
    let image_height: u32;
    let lookfrom: Vec3;
    let lookat: Vec3;
    let vup: Vec3;
    let dist_to_focus: f64;
    let aperture: f64;
    let background: Vec3;
    let cam: Camera;
    match choose {
        1 => {
            //night
            world = HittableList::night();
            aspect_ratio = 3.0 / 2.0;
            image_width = 400;
            image_height = ((image_width as f64) / aspect_ratio) as u32;

            lookfrom = Vec3::new(13.0, 2.0, 3.0);
            lookat = Vec3::new(0.0, 0.0, 0.0);
            vup = Vec3::new(0.0, 1.0, 0.0);
            dist_to_focus = 10.0;
            aperture = 0.1;
            background = Vec3::new(0.0, 0.0, 0.0);

            cam = Camera::new(
                lookfrom,
                lookat,
                vup,
                20.0,
                aspect_ratio,
                aperture,
                dist_to_focus,
            );
        }
        2 => {
            //cornell box
            world = HittableList::cornell_box();
            aspect_ratio = 1.0;
            image_width = 600;
            image_height = ((image_width as f64) / aspect_ratio) as u32;

            lookfrom = Vec3::new(278.0, 278.0, -800.0);
            lookat = Vec3::new(278.0, 278.0, 0.0);
            vup = Vec3::new(0.0, 1.0, 0.0);
            dist_to_focus = 10.0;
            aperture = 0.0;
            background = Vec3::new(0.0, 0.0, 0.0);

            cam = Camera::new(
                lookfrom,
                lookat,
                vup,
                40.0,
                aspect_ratio,
                aperture,
                dist_to_focus,
            );
        }
        _ => {
            //day
            world = HittableList::random_scene();
            aspect_ratio = 3.0 / 2.0;
            image_width = 400;
            image_height = ((image_width as f64) / aspect_ratio) as u32;

            lookfrom = Vec3::new(13.0, 2.0, 3.0);
            lookat = Vec3::new(0.0, 0.0, 0.0);
            vup = Vec3::new(0.0, 1.0, 0.0);
            dist_to_focus = 10.0;
            aperture = 0.1;
            background = Vec3::new(0.7, 0.8, 1.0);

            cam = Camera::new(
                lookfrom,
                lookat,
                vup,
                20.0,
                aspect_ratio,
                aperture,
                dist_to_focus,
            );
        }
    }

    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);
    let bar = ProgressBar::new(image_height as u64);

    for j in 0..image_height {
        for i in 0..image_width {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for _s in 0..samples_per_pixel {
                let u = (i as f64 + rand::random::<f64>()) / (image_width as f64 - 1.0);
                let v = (image_height as f64 - j as f64 + rand::random::<f64>())
                    / (image_height as f64 - 1.0);
                let r = cam.get_ray(u, v);
                col += ray_color(r, background, world.clone(), max_depth);
            }
            let pixel = img.get_pixel_mut(i, j);
            *pixel = image::Rgb([
                (clamp((col.x / (samples_per_pixel as f64)).sqrt(), 0.0, 0.999) * 255.999) as u8,
                (clamp((col.y / (samples_per_pixel as f64)).sqrt(), 0.0, 255.999) * 255.999) as u8,
                (clamp((col.z / (samples_per_pixel as f64)).sqrt(), 0.0, 255.999) * 255.999) as u8,
            ]);
        }
        bar.inc(1);
    }

    img.save("output/test.png").unwrap();
    bar.finish();
}
