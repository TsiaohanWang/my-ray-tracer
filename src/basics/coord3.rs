use std::ops::Sub;

use crate::basics::vec3::Vec3;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Coord3(f64, f64, f64);

impl Coord3 {
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

    pub fn distance(&self) -> f64 {
        f64::sqrt(self.x() * self.x() + self.y() * self.y() + self.z() * self.z())
    }

    pub fn distance_to(&self, other: &Self) -> f64 {
        (self - other).magnitude()
    }
}

impl Sub<&Coord3> for &Coord3 {
    type Output = Vec3;
    fn sub(self, rhs: &Coord3) -> Self::Output {
        let vec1: Vec3 = self.into();
        let vec2: Vec3 = rhs.into();
        &vec1 - &vec2
    }
}

impl Sub<Coord3> for &Coord3 {
    type Output = Vec3;
    fn sub(self, rhs: Coord3) -> Self::Output {
        let vec1: Vec3 = self.into();
        let vec2: Vec3 = rhs.into();
        &vec1 - &vec2
    }
}

impl Sub<&Coord3> for Coord3 {
    type Output = Vec3;
    fn sub(self, rhs: &Coord3) -> Self::Output {
        let vec1: Vec3 = self.into();
        let vec2: Vec3 = rhs.into();
        &vec1 - &vec2
    }
}

impl Sub<Coord3> for Coord3 {
    type Output = Vec3;
    fn sub(self, rhs: Coord3) -> Self::Output {
        let vec1: Vec3 = self.into();
        let vec2: Vec3 = rhs.into();
        &vec1 - &vec2
    }
}

impl From<&Vec3> for Coord3 {
    fn from(value: &Vec3) -> Self {
        Self(value.x(), value.y(), value.z())
    }
}

impl From<Vec3> for Coord3 {
    fn from(value: Vec3) -> Self {
        Self(value.x(), value.y(), value.z())
    }
}

pub const ORIGIN: Coord3 = Coord3(0.0, 0.0, 0.0);