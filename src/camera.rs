use crate::Ray;
use crate::Vec3;
const PI: f64 = 3.1415926535897932385;
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub lens_radius: f64,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let theta = vfov * PI / 180.0;
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio.clone() * viewport_height.clone();

        let ww = (lookfrom - lookat).unit();
        let uu = (Vec3::cross(vup, ww.clone())).unit();
        let vv = Vec3::cross(ww.clone(), uu.clone());

        Self {
            origin: lookfrom.clone(),
            lower_left_corner: lookfrom
                - uu * viewport_width * focus_dist / 2.0
                - vv * viewport_height * focus_dist / 2.0
                - ww * focus_dist,
            horizontal: uu * viewport_width * focus_dist,
            vertical: vv * viewport_height * focus_dist,
            w: ww,
            u: uu,
            v: vv,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let rd = Vec3::random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin - offset,
        )
    }
}
