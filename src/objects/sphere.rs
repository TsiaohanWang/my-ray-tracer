use super::texture::*;
use crate::basics::{coord3::Coord3, vec3::Vec3};
use crate::rays::ray::{Ray, RayIntersectOpaque, RayIntersectErr};
use std::error::Error;
use std::cmp::Ordering;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
struct OpaqueSphere {
    center: Coord3,
    radius: f64,
    texture: OpaqueTexture,
}

impl OpaqueSphere {
    fn new_from(center: Coord3, radius: f64, texture: OpaqueTexture) -> Self {
        Self {
            center,
            radius,
            texture,
        }
    }

    fn get_center(&self) -> &Coord3 {
        &self.center
    }
    fn get_radius(&self) -> f64 {
        self.radius
    }
}

impl RayIntersectOpaque for OpaqueSphere {
    fn intersection(&self, ray: &Ray) -> Result<Option<Coord3>, Box<dyn Error>> {
        let oc_vec: &Vec3 = &(self.get_center() - ray.get_origin());

        if oc_vec.magnitude() < self.get_radius() {
            return Err(Box::new(RayIntersectErr::InnerRayErr));
        }

        let a: f64 = ray.get_direction() * ray.get_direction();
        let b: f64 = 2.0 * &(oc_vec * ray.get_direction());
        let c: f64 = oc_vec * oc_vec - self.get_radius().powi(2);

        let quad_eq_delta: f64 = b.powi(2) - 4.0 * a * c;

        match quad_eq_delta.total_cmp(&0.0) {
            Ordering::Less => return Ok(None),
            Ordering::Equal => {
                let t_root: f64 = (-b) / (2.0 * a);
                let origin: &Vec3 = &ray.get_origin().into();
                let intersection: Vec3 = origin + t_root * ray.get_direction();
                return Ok(Some(intersection.into()))
            },
            Ordering::Greater => {
                let t_root1: f64 = ((-b) - quad_eq_delta.sqrt()) / (2.0 * a);
                let t_root2: f64 = ((-b) + quad_eq_delta.sqrt()) / (2.0 * a);

                if t_root1 < 0.0 || t_root2 < 0.0 {
                    return Err(Box::new(RayIntersectErr::NegativeRootErr));
                }

                let min_root = match t_root1 < t_root2 {
                    true => t_root1,
                    false => t_root2
                };

                let origin: &Vec3 = &ray.get_origin().into();
                let intersection: Vec3 = origin + min_root * ray.get_direction();
                return Ok(Some(intersection.into()))
            }
        }
    }
}
