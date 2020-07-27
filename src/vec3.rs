use std::ops::{Add, AddAssign};

//use crate::rand;

use rand::prelude::*;

extern crate rand;
use rand::Rng;

const PI: f64 = 3.1415926535897932385;

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn ones() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn squared_length(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add<f64> for Vec3 {
    type Output = Self;

    fn add(self, other: f64) -> Self {
        Self {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl AddAssign<f64> for Vec3 {
    fn add_assign(&mut self, other: f64) {
        *self = Self {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        };
    }
}

use std::ops::{Sub, SubAssign};

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Sub<f64> for Vec3 {
    type Output = Self;

    fn sub(self, other: f64) -> Self {
        Self {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        };
    }
}

impl SubAssign<f64> for Vec3 {
    fn sub_assign(&mut self, other: f64) {
        *self = Self {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
        };
    }
}

use std::ops::{Mul, MulAssign};

impl Mul for Vec3 {
    type Output = f64;

    fn mul(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        *self = Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        };
    }
}

use std::ops::Div;

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl Vec3 {
    pub fn elemul(a: Vec3, b: Vec3) -> Vec3 {
        Vec3 {
            x: a.x * b.x,
            y: a.y * b.y,
            z: a.z * b.z,
        }
    }

    pub fn cross(a: Vec3, b: Vec3) -> Vec3 {
        Vec3 {
            x: a.y * b.z - a.z * b.y,
            y: a.z * b.x - a.x * b.z,
            z: a.x * b.y - a.y * b.x,
        }
    }
}

use std::ops::Neg;

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Vec3 {
    pub fn length(self) -> f64 {
        ((self.x * self.x + self.y * self.y + self.z * self.z) as f64).sqrt()
    }

    pub fn length_squared(self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z) as f64
    }

    pub fn unit(self) -> Vec3 {
        let a: f64 = ((self.x * self.x + self.y * self.y + self.z * self.z) as f64).sqrt();
        if a == 0.0 {
            panic!("Divide by 0!");
        }
        Vec3 {
            x: self.x / a,
            y: self.y / a,
            z: self.z / a,
        }
    }
}

impl Vec3 {
    pub fn Random() -> Vec3 {
        Vec3::new(
            rand::random::<f64>(),
            rand::random::<f64>(),
            rand::random::<f64>(),
        )
    }

    pub fn Random_(mi: f64, ma: f64) -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3::new(
            rng.gen_range(mi, ma),
            rng.gen_range(mi, ma),
            rng.gen_range(mi, ma),
        )
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::Random_(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_unit_vec() -> Vec3 {
        let mut rng = rand::thread_rng();
        let a = rng.gen_range(0.0, PI);
        let z = rng.gen_range(-1.0, 1.0);
        let r: f64 = ((1.0 - z.clone() * z.clone()) as f64).sqrt();
        return Vec3::new(r * a.cos(), r * a.sin(), z);
    }

    pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
        let in_unit_sphere = Vec3::random_in_unit_sphere();
        if in_unit_sphere.clone() * normal.clone() > 0.0 {
            return in_unit_sphere;
        }
        return -in_unit_sphere;
    }

    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        return v - n * (v * n) * 2.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0), Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_add() {
        assert_eq!(
            Vec3::new(1.0, 0.0, -1.0) + Vec3::new(2.0, 4.0, 6.0),
            Vec3::new(3.0, 4.0, 5.0)
        )
    }

    #[test]
    fn test_add_assign() {
        let mut x = Vec3::new(1.0, 0.0, -1.0);
        x += Vec3::new(2.0, 4.0, 6.0);
        assert_eq!(x, Vec3::new(3.0, 4.0, 5.0))
    }

    #[test]
    fn test_add_f64() {
        assert_eq!(
            Vec3::new(1.0, 0.0, -1.0) + 233.0,
            Vec3::new(234.0, 233.0, 232.0)
        )
    }

    #[test]
    fn test_add_assign_f64() {
        let mut x = Vec3::new(1.0, 0.0, -1.0);
        x += 233.0;
        assert_eq!(x, Vec3::new(234.0, 233.0, 232.0))
    }

    #[test]
    fn test_sub() {
        assert_eq!(
            Vec3::new(1.0, 0.0, -1.0) - Vec3::new(2.0, 4.0, 6.0),
            Vec3::new(-1.0, -4.0, -7.0)
        )
    }

    #[test]
    fn test_sub_assign() {
        let mut x = Vec3::new(1.0, 0.0, -1.0);
        x -= Vec3::new(2.0, 4.0, 6.0);
        assert_eq!(x, Vec3::new(-1.0, -4.0, -7.0))
    }

    #[test]
    fn test_sub_f64() {
        assert_eq!(Vec3::new(1.0, 0.0, -1.0) - 1.0, Vec3::new(0.0, -1.0, -2.0))
    }

    #[test]
    fn test_sub_assign_f64() {
        let mut x = Vec3::new(1.0, 0.0, -1.0);
        x -= 1.0;
        assert_eq!(x, Vec3::new(0.0, -1.0, -2.0))
    }

    #[test]
    fn test_mul() {
        assert_eq!(Vec3::new(1.0, 0.0, -1.0) * Vec3::ones(), 0.0);
    }

    #[test]
    fn test_mul_assign() {
        let mut x = Vec3::new(1.0, 0.0, -1.0);
        x *= 2.0;
        assert_eq!(x, Vec3::new(2.0, 0.0, -2.0));
    }

    #[test]
    fn test_mul_f64() {
        assert_eq!(Vec3::new(1.0, 0.0, -1.0) * 1.0, Vec3::new(1.0, 0.0, -1.0));
    }

    #[test]
    fn test_div() {
        assert_eq!(Vec3::new(1.0, -2.0, 0.0) / 2.0, Vec3::new(0.5, -1.0, 0.0));
    }

    #[test]
    fn test_elemul() {
        assert_eq!(
            Vec3::elemul(Vec3::new(1.0, 2.0, 3.0), Vec3::new(1.0, 2.0, 3.0)),
            Vec3::new(1.0, 4.0, 9.0)
        );
    }

    #[test]
    fn test_cross() {
        assert_eq!(
            Vec3::cross(Vec3::new(1.0, 2.0, 3.0), Vec3::new(2.0, 3.0, 4.0)),
            Vec3::new(8.0 - 9.0, 6.0 - 4.0, 3.0 - 4.0)
        );
    }
    #[test]
    fn test_neg() {
        assert_eq!(-Vec3::new(1.0, -2.0, 3.0), Vec3::new(-1.0, 2.0, -3.0));
    }

    #[test]
    fn test_squared_length() {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0).squared_length(), 14.0 as f64);
    }

    #[test]
    fn test_length() {
        assert_eq!(
            Vec3::new(3.0, 4.0, 5.0).length(),
            ((3.0 * 3.0 + 4.0 * 4.0 + 5.0 * 5.0) as f64).sqrt()
        );
    }

    #[test]
    fn test_unit() {
        assert_eq!(Vec3::new(233.0, 0.0, 0.0).unit(), Vec3::new(1.0, 0.0, 0.0));
        assert_eq!(
            Vec3::new(-233.0, 0.0, 0.0).unit(),
            Vec3::new(-1.0, 0.0, 0.0)
        );
    }

    #[test]
    #[should_panic]
    fn test_unit_panic() {
        Vec3::new(0.0, 0.0, 0.0).unit();
    }
}
