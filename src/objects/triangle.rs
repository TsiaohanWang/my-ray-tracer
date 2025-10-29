use std::{error::Error, fmt::Display};

use crate::basics::{
    coord3::Coord3,
    vec3::*,
};
use crate::rays::ray::{Ray, RayIntersectOpaque};

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct OpaqueTriangle {
    p1: Coord3,
    p2: Coord3,
    p3: Coord3,
}

impl OpaqueTriangle {
    pub fn new_from(p1: Coord3, p2: Coord3, p3: Coord3) -> Result<Self, Box<dyn Error>> {
        let ab: Vec3 = p2 - p1;
        let ac: Vec3 = p3 - p1;
        match (&ab).cross(&ac) {
            ZERO_VEC3 => return Err(Box::new(TriagErr::InvalidParamErr)),
            _ => (),
        }

        Ok(Self { p1, p2, p3 })
    }
}

impl RayIntersectOpaque for OpaqueTriangle {
    fn intersection(&self, ray: &Ray) -> Result<Option<Coord3>, Box<dyn Error>> {
        let e1 = self.p2 - self.p1;
        let e2 = self.p3 - self.p1;
        let s = ray.get_origin() - self.p1;
        let s1 = ray.get_direction().cross(&e2);
        let s2 = s.cross(&e1);

        let reciproc_s1_dot_e1 = 1.0 / (s1 * e1);

        let t = s2 * e2 * reciproc_s1_dot_e1;
        let b1 = s1 * s * reciproc_s1_dot_e1;
        let b2 = s2 * ray.get_direction() * reciproc_s1_dot_e1;

        if t >= 0.0 && b1 > 0.0 && b2 > 0.0 && b1 + b2 < 1.0 {
            let origin: &Vec3 = &ray.get_origin().into();
            let intersection: Vec3 = origin + t * ray.get_direction();
            return Ok(Some(intersection.into()));
        } else {
            return Ok(None);
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TriagErr {
    /// 输入了无效的初始化参数
    InvalidParamErr,
}

impl Display for TriagErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidParamErr => write!(f, "invalid triangle construction"),
        }
    }
}
impl Error for TriagErr {}

impl TriagErr {
    pub fn handle(&self) {
        eprintln!("[Triangle Error] {}", self);
    }
}
