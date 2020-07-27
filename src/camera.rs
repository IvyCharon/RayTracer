use crate::Ray;
use crate::Vec3;
const PI: f64 = 3.1415926535897932385;
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f64, aspect_ratio: f64) -> Self {
        let theta = vfov * PI / 180.0;
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio.clone() * viewport_height.clone();
        let focal_length = 1.0;

        let w = (lookfrom - lookat).unit();
        let u = (Vec3::cross(vup, w.clone())).unit();
        let v = Vec3::cross(w.clone(), u.clone());

        Self {
            origin: lookfrom.clone(),
            lower_left_corner: lookfrom - u * viewport_width / 2.0 - v * viewport_height / 2.0 - w,
            horizontal: u * viewport_width,
            vertical: v * viewport_height,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
}
