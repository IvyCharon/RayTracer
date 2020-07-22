mod ray;
#[allow(clippy::float_cmp)]
mod vec3;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use ray::Ray;

pub use vec3::Vec3;

fn ray_color(r: Ray) -> Vec3 {
    let t = 0.5 * (r.dir.y + 1.0);
    Vec3 {
        x: 1.0 - t + t * 0.5,
        y: 1.0 - t + t * 0.7,
        z: 1.0,
    }
}

fn main() {
    let mut img: RgbImage = ImageBuffer::new(1024, 512);
    let bar = ProgressBar::new(1024);

    //let pixel = img.get_pixel_mut(x, y);
    //let color = 0 as u8;
    //*pixel = image::Rgb([color, color, color]);
    //bar.inc(1);

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(16.0 / 9.0 * 2.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let lower_left_corner = origin.clone()
        - horizontal.clone() / 2.0
        - vertical.clone() / 2.0
        - Vec3::new(0.0, 0.0, 1.0);
    for j in 0..512 {
        for i in 0..1024 {
            let pixel = img.get_pixel_mut(i, j);
            let u = i as f64 / (1024.0 - 1.0);
            let v = (511 - j) as f64 / (512.0 - 1.0);
            let r = Ray::new(
                origin.clone(),
                lower_left_corner.clone() + horizontal.clone() * u + vertical.clone() * v,
            );
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
