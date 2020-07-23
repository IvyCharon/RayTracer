mod ray;
mod sphere;
#[allow(clippy::float_cmp)]
mod vec3;
use sphere::hit_record;
use sphere::Object;
use sphere::Sphere;
mod hittable;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use ray::Ray;

pub use vec3::Vec3;

fn hit_sphere(center: Vec3, radius: f64, r: Ray) -> f64 {
    let oc = r.beg - center;
    let a = r.dir.length_squared();
    let b = 2.0 * (oc * r.dir);
    let c = oc.length_squared() - radius * radius;
    let ans = b * b - 4.0 * a * c;
    if ans < 0.0 {
        return -1.0;
    } else {
        return (-b - ans.sqrt()) / (2.0 * a);
    }
}

fn ray_color(r: Ray) -> Vec3 {
    let tmp = hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, r.clone());
    if tmp > 0.0 {
        let N = (r.at(tmp) - Vec3::new(0.0, 0.0, -1.0)).unit();
        return Vec3::new(N.x + 1.0, N.y + 1.0, N.z + 1.0) * 0.5;
    }
    let t = 0.5 * (r.dir.unit().y + 1.0);
    Vec3 {
        x: 1.0 - t + t * 0.5,
        y: 1.0 - t + t * 0.7,
        z: 1.0,
    }
}

fn main() {
    let mut img: RgbImage = ImageBuffer::new(400, 225);
    let bar = ProgressBar::new(225);

    //let pixel = img.get_pixel_mut(x, y);
    //let color = 0 as u8;
    //*pixel = image::Rgb([color, color, color]);
    //bar.inc(1);

    //draw sky
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
            let color = ray_color(r);
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
