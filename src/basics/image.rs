use crate::errors::{RayTracerErr, nan::check};

pub struct ImgPixel {
    float_r: f64,
    float_g: f64,
    float_b: f64,
}

impl ImgPixel {
    fn new_from(float_r: f64, float_g: f64, float_b: f64) -> Self {
        Self {
            float_r: check::<RayTracerErr>(float_r, "ImgPixel::new_from").unwrap(),
            float_g: check::<RayTracerErr>(float_g, "ImgPixel::new_from").unwrap(),
            float_b: check::<RayTracerErr>(float_b, "ImgPixel::new_from").unwrap(),
        }
    }
}
