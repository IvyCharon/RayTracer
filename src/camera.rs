use crate::Ray;
use crate::Vec3;
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio.clone() * viewport_height.clone();
        let focal_length = 1.0;
        Self {
            origin: Vec3::new(0.0, 0.0, 0.0),
            lower_left_corner: Vec3::new(0.0, 0.0, 0.0)
                - Vec3::new(viewport_width.clone(), 0.0, 0.0) / 2.0
                - Vec3::new(0.0, viewport_height.clone(), 0.0) / 2.0
                - Vec3::new(0.0, 0.0, focal_length.clone()),
            horizontal: Vec3::new(viewport_width.clone(), 0.0, 0.0),
            vertical: Vec3::new(0.0, viewport_height.clone(), 0.0),
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
}
