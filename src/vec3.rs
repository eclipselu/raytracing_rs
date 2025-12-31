use std::ops::{Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Neg, Sub};

use crate::utils::{random_double, random_double_range};

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub type Color = Vec3;
pub type Point3 = Vec3;

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    pub fn length_squared(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn unit_vector(self) -> Vec3 {
        self / self.length()
    }

    pub fn near_zero(self) -> bool {
        let eps = 1e-8;
        self.x.abs() < eps && self.y.abs() < eps && self.z.abs() < eps
    }

    pub fn random() -> Vec3 {
        Vec3 {
            x: random_double(),
            y: random_double(),
            z: random_double(),
        }
    }

    pub fn random_range(min: f64, max: f64) -> Vec3 {
        Vec3 {
            x: random_double_range(min, max),
            y: random_double_range(min, max),
            z: random_double_range(min, max),
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        loop {
            let v = Vec3::random_range(-1.0, 1.0);
            let lensq = v.length_squared();
            if lensq > 1e-160 && lensq <= 1.0 {
                return v / v.length();
            }
        }
    }

    pub fn random_on_hemisphere(normal: Vec3) -> Vec3 {
        let v = Vec3::random_unit_vector();
        if dot(v, normal) > 0.0 { v } else { -v }
    }
}
impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!(
                "index out of bounds: the len is 3 but the index is {}",
                index
            ),
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, t: f64) -> Self::Output {
        Vec3 {
            x: t * self.x,
            y: t * self.y,
            z: t * self.z,
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Self::Output {
        Vec3 {
            x: v.x * self.x,
            y: v.y * self.y,
            z: v.z * self.z,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t: f64) {
        self.x *= t;
        self.y *= t;
        self.z *= t;
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, t: f64) -> Self::Output {
        Vec3 {
            x: self.x / t,
            y: self.y / t,
            z: self.z / t,
        }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t: f64) {
        self.x /= t;
        self.y /= t;
        self.z /= t;
    }
}

pub fn dot(v1: Vec3, v2: Vec3) -> f64 {
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
}

pub fn reflect(v: Vec3, normal: Vec3) -> Vec3 {
    v - 2.0 * dot(v, normal) * normal
}

pub fn refract(uv: Vec3, normal: Vec3, eta_over_etap: f64) -> Vec3 {
    let cos_theta = dot(-uv, normal).min(1.0);
    let r_out_perp = eta_over_etap * (uv + cos_theta * normal);
    let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * normal;

    r_out_perp + r_out_parallel
}
