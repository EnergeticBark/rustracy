use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};
use crate::{random_f64, random_f64_range};

#[derive(Debug, Copy, Clone)]
pub struct Vec3(f64, f64, f64);

impl Vec3 {
    pub fn new() -> Self {
        Self(0.0, 0.0, 0.0)
    }

    pub fn from(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z)
    }

    pub fn random() -> Self {
        Self(random_f64(), random_f64(), random_f64())
    }

    pub fn random_range(min: f64, max :f64) -> Self {
        Self(random_f64_range(min, max), random_f64_range(min, max), random_f64_range(min, max))
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let point = Vec3::random_range(-1.0, 1.0);
            if point.length_squared() < 1.0 {
                return point
            }
        }
    }

    pub fn random_unit_vector() -> Self {
        unit_vector(Vec3::random_in_unit_sphere())
    }

    pub fn random_in_unit_disc() -> Self {
        loop {
            let point = Vec3::from(random_f64_range(-1.0, 1.0), random_f64_range(-1.0, 1.0), 0.0);
            if point.length_squared() < 1.0 {
                return point
            }
        }
    }

    pub fn x(&self) -> f64 {
        self.0
    }
    pub fn y(&self) -> f64 {
        self.1
    }
    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    fn length_squared(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.0.abs() < s && self.1.abs() < s && self.2.abs() < s
    }

    pub fn reflect(&self, normal: Vec3) -> Self {
        *self - 2.0 * dot(*self, normal) * normal
    }

    pub fn refract(&self, normal: Vec3, etai_over_etat: f64) -> Self {
        let cos_theta = dot(-*self, normal).min(1.0);
        let r_out_perp = etai_over_etat * (*self + cos_theta * normal);
        let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * normal;
        r_out_perp + r_out_parallel
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2);
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        (1.0 / rhs) * self
    }
}

pub fn dot(a: Vec3, b: Vec3) -> f64 {
    a.0 * b.0 + a.1 * b.1 + a.2 * b.2
}

pub fn cross(a: Vec3, b: Vec3) -> Vec3 {
    Vec3(
        a.1 * b.2 - a.2 * b.1,
        a.2 * b.0 - a.0 * b.2,
        a.0 * b.1 - a.1 * b.0,
    )
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

pub type Point3 = Vec3;
pub type Color = Vec3;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn negate_vec3() {
        let neg = -Vec3(1.0, 2.0, 3.0);
        assert_eq!(neg.0, -1.0);
        assert_eq!(neg.1, -2.0);
        assert_eq!(neg.2, -3.0);
    }
}
