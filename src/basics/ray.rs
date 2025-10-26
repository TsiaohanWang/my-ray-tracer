use std::error::Error;

use super::coord3::ORIGIN;
use super::coord3::Coord3;
use super::vec3::Vec3;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Ray {
    /// 光线的源点坐标
    origin: Coord3,
    /// 光线的射出方向
    direction: Vec3
}

impl Ray {
    pub fn new_from(origin: Coord3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn get_origin(&self) -> &Coord3 {
        &self.origin
    }

    pub fn get_direction(&self) -> &Vec3 {
        &self.direction
    }
}

pub trait RayIntersectOpaque {
    /// 光线与不透明物体的交点
    fn intersection(&self, ray: &Ray) -> Result<Option<Coord3>, RayIntersectErr>;
}

#[derive(Debug)]
pub enum RayIntersectErr {
    /// 光线由不透明物体内部发出
    InnerRayErr,
    /// 计算时出现 f64::NaN 无效值
    ExistsNaNErr(String),
    /// 二次方程的根（表示光线沿射出方向行进的时间）为负
    NegativeRootErr,
}