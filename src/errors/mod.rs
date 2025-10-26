pub mod nan;

use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct RayTracerErr {
    details: String,
}

impl Display for RayTracerErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ray Tracer Error: {}", self.details)
    }
}

impl Error for RayTracerErr {}
