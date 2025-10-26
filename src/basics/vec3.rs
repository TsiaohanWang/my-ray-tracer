#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Vec3(f64, f64, f64);

use std::ops::{Add, Mul, Sub};
use super::coord3::Coord3;

impl Vec3 {
    pub fn new() -> Self {
        Self(0.0, 0.0, 0.0)
    }

    pub fn new_from(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z)
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

    pub fn magnitude(&self) -> f64 {
        f64::sqrt(self.x() * self.x() + self.y() * self.y() + self.z() * self.z())
    }

    pub fn normalize(&self) -> Self {
        Self::new_from(
            self.x() / self.magnitude(),
            self.y() / self.magnitude(),
            self.z() / self.magnitude()
        )
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z()
    }
}

impl Add<Vec3> for &Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3::new_from(
            self.x() + rhs.x(),
            self.y() + rhs.y(),
            self.z() + rhs.z()
        )
    }
}

impl Add<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn add(self, rhs: &Vec3) -> Self::Output {
        Vec3::new_from(
            self.x() + rhs.x(),
            self.y() + rhs.y(),
            self.z() + rhs.z()
        )
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3::new_from(
            self.x() + rhs.x(),
            self.y() + rhs.y(),
            self.z() + rhs.z()
        )
    }
}

impl Add<&Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: &Vec3) -> Self::Output {
        Vec3::new_from(
            self.x() + rhs.x(),
            self.y() + rhs.y(),
            self.z() + rhs.z()
        )
    }
}

impl Sub<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: &Vec3) -> Self::Output {
        Vec3::new_from(
            self.x() - rhs.x(),
            self.y() - rhs.y(),
            self.z() - rhs.z()
        )
    }
}

impl Sub<&Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: &Vec3) -> Self::Output {
        Vec3::new_from(
            self.x() - rhs.x(),
            self.y() - rhs.y(),
            self.z() - rhs.z()
        )
    }
}

impl Sub<Vec3> for &Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3::new_from(
            self.x() - rhs.x(),
            self.y() - rhs.y(),
            self.z() - rhs.z()
        )
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3::new_from(
            self.x() - rhs.x(),
            self.y() - rhs.y(),
            self.z() - rhs.z()
        )
    }
}

impl Mul<&Vec3> for &Vec3 {
    type Output = f64;
    fn mul(self, rhs: &Vec3) -> Self::Output {
        self.dot(rhs)
    }
}

impl Mul<&Vec3> for Vec3 {
    type Output = f64;
    fn mul(self, rhs: &Vec3) -> Self::Output {
        self.dot(rhs)
    }
}

impl Mul<Vec3> for &Vec3 {
    type Output = f64;
    fn mul(self, rhs: Vec3) -> Self::Output {
        self.dot(&rhs)
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = f64;
    fn mul(self, rhs: Vec3) -> Self::Output {
        self.dot(&rhs)
    }
}

impl Mul<f64> for &Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new_from(
            self.x() * rhs,
            self.y() * rhs,
            self.z() * rhs
        )
    }
}

impl Mul<&Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3::new_from(
            self * rhs.x(),
            self * rhs.y(),
            self * rhs.z()
        )
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new_from(
            self.x() * rhs,
            self.y() * rhs,
            self.z() * rhs
        )
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new_from(
            self * rhs.x(),
            self * rhs.y(),
            self * rhs.z()
        )
    }
}

impl From<&Coord3> for Vec3 {
    fn from(value: &Coord3) -> Self {
        Self::new_from(value.x(), value.y(), value.z())
    }
}

impl From<Coord3> for Vec3 {
    fn from(value: Coord3) -> Self {
        Self::new_from(value.x(), value.y(), value.z())
    }
}