#[allow(clippy::float_cmp)]
mod vec3;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;

pub use vec3::Vec3;

fn main() {
    let mut img: RgbImage = ImageBuffer::new(1024, 512);
    let bar = ProgressBar::new(1024);

    for x in 0..1024 {
        for y in 0..512 {
            let pixel = img.get_pixel_mut(x, y);
            let color = 0 as u8;
            *pixel = image::Rgb([color, color, color]);
        }
        bar.inc(1);
    }

    let color: u8 = 125 as u8;

    for i in 1..16 {
        let theta: f64 = (i as f64 / 16.0) * 2.0 * std::f64::consts::PI as f64;
        let dir = Vec3::new(theta.cos(), theta.sin(), 0.0);
        let cen = Vec3::new(512.0, 256.0, 0.0);
        let mut begin: Vec3 = dir.clone() * 25.0 + cen;
        let mut j = 1;
        while j < 201 {
            begin += dir.clone();
            let pixel = img.get_pixel_mut(begin.x as u32, begin.y as u32);
            *pixel = image::Rgb([color, color, color]);
            j += 1;
        }
    }

    img.save("output/test.png").unwrap();
    bar.finish();
}
