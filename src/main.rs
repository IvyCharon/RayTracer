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

const PI:f64 = 3.1415926535897932385;
const INFINITY: f64 = 1e15;

pub use vec3::Vec3;

fn ray_color(r: Ray, world: &Hittable_list) -> Vec3 {
    let rec = world.hit(r, 0.0, INFINITY);
    match rec {
        Option::Some(rec) => (rec.normal + Vec3::ones()) * 0.5,
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
    for j in 0..225 {
        for i in 0..400 {
            let pixel = img.get_pixel_mut(i, j);
            let u = i as f64 / (400.0 - 1.0);
            let v = (225 - j) as f64 / (225.0 - 1.0);
            let r = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);
            let color = ray_color(r, &world);
            *pixel = image::Rgb([
                (color.x * 255.999) as u8,
                (color.y * 255.999) as u8,
                (color.z * 255.999) as u8,
            ]);
        }
        bar.inc(1);
    }

    img.save("output/test.png").unwrap();
    bar.finish();
}
