mod AABB_;
mod camera;
mod object;
mod ray;
#[allow(clippy::float_cmp)]
mod vec3;
use AABB_::aabb;
mod BVH;
mod texture;
use camera::Camera;
use object::Hit_record;
use object::Object;
use object::Sphere;
use BVH::bvh_node;
mod hittable_list;
use hittable_list::Hittable_list;
mod material;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use material::diffuse_light;
use material::Dielectric;
use material::Lambertian;
use material::Material;
use material::Metal;
use object::xy_rect;
use object::yz_rect;
use object::xz_rect;
use object::box_;
use ray::Ray;
use std::sync::Arc;
use texture::checker_texture;
use texture::solid_color;
use texture::Texture;
extern crate rand;

const INFINITY: f64 = 1e15;

pub use vec3::Vec3;

fn ray_color(r: Ray, back_ground: Vec3, world: Arc<dyn Object>, depth: i32) -> Vec3 {
    let rec = world.hit(r, 0.001, INFINITY);
    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }
    match rec {
        Option::Some(rec) => {
            let s = rec.mat.as_ref().unwrap().scatter(r, &rec);
            let emitted = rec.mat.unwrap().emitted(rec.u, rec.v, rec.p);
            if s.jud {
                return emitted
                    + Vec3::elemul(
                        ray_color(s.scattered, back_ground, world, depth - 1),
                        s.attenustion,
                    );
            }
            return emitted;
        }
        Option::None => {
            return back_ground;
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
    const aspect_ratio: f64 = 1.0;
    const image_width: u32 = 600;
    const image_height: u32 = ((image_width as f64) / aspect_ratio) as u32;
    let samples_per_pixel = 200;
    let max_depth = 50;
    let mut img: RgbImage = ImageBuffer::new(image_width.clone(), image_height.clone());
    let bar = ProgressBar::new(image_height as u64);

    let world = Hittable_list::cornell_box();

    let lookfrom = Vec3::new(278.0, 278.0, -800.0);
    let lookat = Vec3::new(278.0, 278.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;
    let background = Vec3::new(0.0, 0.0, 0.0);
    //let background = Vec3::new(0.7, 0.8, 1.0);

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        40.0,
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
                col += ray_color(r, background, world.clone(), max_depth.clone());
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
