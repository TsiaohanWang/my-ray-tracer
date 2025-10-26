use super::RayTracerErr;

pub trait f64NaNErr {
    fn from_msg(msg: String) -> Self;
}

impl f64NaNErr for RayTracerErr {
    fn from_msg(msg: String) -> Self {
        Self { details: msg }
    }
}

pub fn check<E: f64NaNErr>(value: f64, val_details: &str) -> Result<f64, E> {
    if value.is_nan() {
        return Err(E::from_msg("NaN found when checking ".to_string() + val_details))
    }

    Ok(value)
}