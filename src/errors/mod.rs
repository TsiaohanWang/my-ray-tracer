pub mod nan;

use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct MainErr {
    details: String,
}

impl MainErr {
    pub fn e(details: &str) -> Self {
        Self { details: details.to_string() }
    }
}

impl Display for MainErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Err[{}]", self.details)
    }
}

impl Error for MainErr {}

impl MainErr {
    pub fn handle(self) {
        eprintln!("[Main Error] {}", self);
    }
}