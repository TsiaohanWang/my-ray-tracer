use std::error::Error;

use crate::basics::coord3::Coord3;
use crate::{
    errors::{MainErr, nan},
};
use crate::rays::ray::{Ray, RayIntersectOpaque};

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct AlignedBox {
    x_axis_bound: (f64, f64),
    y_axis_bound: (f64, f64),
    z_axis_bound: (f64, f64),
}

impl AlignedBox {
    pub fn new_from(
        x_axis_bound: (f64, f64),
        y_axis_bound: (f64, f64),
        z_axis_bound: (f64, f64),
    ) -> Result<Self, Box<dyn Error>> {
        let check = |bound_tuple: (f64, f64)| -> Result<(), Box<dyn Error>> {
            let _ = nan::check::<MainErr>(bound_tuple.1, "AlignedBox::new_from")?;
            Ok(())
        };

        check(x_axis_bound)?;
        check(y_axis_bound)?;
        check(z_axis_bound)?;

        return Ok(Self {
            x_axis_bound,
            y_axis_bound,
            z_axis_bound,
        });
    }

    pub fn get_x(&self) -> (f64, f64) {
        self.x_axis_bound
    }

    pub fn get_y(&self) -> (f64, f64) {
        self.y_axis_bound
    }

    pub fn get_z(&self) -> (f64, f64) {
        self.z_axis_bound
    }

    pub fn enter_n_exit(&self, ray: &Ray) -> Result<Option<(Coord3, Coord3)>, Box<dyn Error>> {
        let t1_x = (self.get_x().0 - ray.get_origin().x()) / ray.get_direction().x();
        let t2_x = (self.get_x().1 - ray.get_origin().x()) / ray.get_direction().x();
        let t1_y = (self.get_y().0 - ray.get_origin().y()) / ray.get_direction().y();
        let t2_y = (self.get_y().1 - ray.get_origin().y()) / ray.get_direction().y();
        let t1_z = (self.get_z().0 - ray.get_origin().z()) / ray.get_direction().z();
        let t2_z = (self.get_z().1 - ray.get_origin().z()) / ray.get_direction().z();

        let check_if_enter_exit = |t1:f64, t2: f64| -> bool {
            if t1 > 0.0 && t2 > 0.0 {
                return true;
            } else {
                return false;
            }
        };

        let enter_exit_box_x = check_if_enter_exit(t1_x, t2_x);
        let enter_exit_box_y = check_if_enter_exit(t1_y, t2_y);
        let enter_exit_box_z = check_if_enter_exit(t1_z, t2_z);

        let min = |f1: f64, f2: f64| -> f64 {
            if f1 > f2 {
                return f2;
            } else {
                return f1;
            }
        };
        let max = |f1: f64, f2: f64| -> f64 {
            if f1 < f2 {
                return f2;
            } else {
                return f1;
            }
        };

        if enter_exit_box_x && enter_exit_box_y && enter_exit_box_z {
            let enter = Coord3::new_from(min(t1_x, t2_x), min(t1_y, t2_y), min(t1_z, t2_z));
            let exit = Coord3::new_from(max(t1_x, t2_x), max(t1_y, t2_y), max(t1_z, t2_z));
            Ok(Some((enter, exit)))
        } else {
            Ok(None)
        }
    }
}

impl RayIntersectOpaque for AlignedBox {
    fn intersection(&self, ray: &Ray) -> Result<Option<Coord3>, Box<dyn Error>> {
        let t1_x = (self.get_x().0 - ray.get_origin().x()) / ray.get_direction().x();
        let t2_x = (self.get_x().1 - ray.get_origin().x()) / ray.get_direction().x();
        let t1_y = (self.get_y().0 - ray.get_origin().y()) / ray.get_direction().y();
        let t2_y = (self.get_y().1 - ray.get_origin().y()) / ray.get_direction().y();
        let t1_z = (self.get_z().0 - ray.get_origin().z()) / ray.get_direction().z();
        let t2_z = (self.get_z().1 - ray.get_origin().z()) / ray.get_direction().z();

        let check_if_enter_exit = |t1:f64, t2: f64| -> bool {
            if t1 > 0.0 && t2 > 0.0 {
                return true;
            } else {
                return false;
            }
        };

        let enter_exit_box_x = check_if_enter_exit(t1_x, t2_x);
        let enter_exit_box_y = check_if_enter_exit(t1_y, t2_y);
        let enter_exit_box_z = check_if_enter_exit(t1_z, t2_z);

        let min = |f1: f64, f2: f64| -> f64 {
            if f1 > f2 {
                return f2;
            } else {
                return f1;
            }
        };

        if enter_exit_box_x && enter_exit_box_y && enter_exit_box_z {
            Ok(Some(Coord3::new_from(min(t1_x, t2_x), min(t1_y, t2_y), min(t1_z, t2_z))))
        } else {
            Ok(None)
        }
    }
}
